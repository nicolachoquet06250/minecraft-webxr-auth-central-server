use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Query, State},
    http::StatusCode,
    response::Response,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc, sync::OnceLock};

use crate::{
    models::{friendship, server, Friendship, Server},
    services::{
        presence_ws::{PresenceWsHub, ServerPresencePlayer},
        Claims,
    },
    AppState,
};

static PRESENCE_HUB: OnceLock<PresenceWsHub> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct PresenceSocketQuery {
    pub auth: String,
    #[serde(default, alias = "serverId")]
    pub server_id: Option<String>,
    #[serde(default, alias = "includeAllPlayers")]
    pub include_all_players: bool,
}

#[derive(Debug, Deserialize)]
struct PresenceSocketMessage {
    #[serde(rename = "type")]
    message_type: String,
    payload: PresenceSocketPayload,
}

#[derive(Debug, Deserialize)]
struct PresenceSocketPayload {
    player_id: String,
    nickname: String,
    game_domain: String,
}

#[derive(Debug, Serialize)]
struct PresenceSocketOutbound {
    #[serde(rename = "type")]
    message_type: String,
    payload: PresenceSocketOutboundPayload,
}

#[derive(Debug, Serialize)]
struct PresenceSocketOutboundPayload {
    user_id: String,
    player_id: String,
    nickname: String,
    server: Option<PresenceSocketServerPayload>,
    is_friend: bool,
}

#[derive(Debug, Serialize, Clone)]
struct PresenceSocketServerPayload {
    id: String,
    name: String,
    game_domain: String,
}

#[derive(Debug, Serialize)]
struct ServerPresenceSnapshotOutbound {
    #[serde(rename = "type")]
    message_type: &'static str,
    payload: ServerPresenceSnapshotPayload,
}

#[derive(Debug, Serialize)]
struct ServerPresenceSnapshotPayload {
    server: PresenceSocketServerPayload,
    players: Vec<ServerPresencePlayerOutbound>,
    current_connected_players: usize,
}

#[derive(Debug, Serialize)]
struct ServerPresencePlayerOutbound {
    user_id: String,
    central_user_id: String,
    player_id: String,
    nickname: String,
    is_friend: bool,
}

pub async fn friends_presence_socket(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PresenceSocketQuery>,
    ws: WebSocketUpgrade,
) -> Result<Response, StatusCode> {
    let claims = state
        .jwt_service
        .verify_token(query.auth.trim())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let subscribed_server = resolve_server_subscription(&state, &query).await?;

    Ok(ws.on_upgrade(move |socket| handle_socket(state, claims, subscribed_server, socket)))
}

async fn handle_socket(
    state: Arc<AppState>,
    claims: Claims,
    subscribed_server: Option<PresenceSocketServerPayload>,
    mut socket: WebSocket,
) {
    let user_id = claims.sub.clone();
    let hub = presence_hub().clone();
    let subscribed_server_id = subscribed_server.as_ref().map(|server| server.id.as_str());
    let (session_id, mut receiver) = hub.register(&user_id, subscribed_server_id).await;

    if let Some(server) = subscribed_server.as_ref() {
        if let Some(snapshot) = server_presence_snapshot_message(&state, &hub, &server.id, &user_id, Some(server.clone())).await {
            if socket.send(Message::Text(snapshot)).await.is_err() {
                hub.unregister(&user_id, &session_id).await;
                return;
            }
        }
    }

    loop {
        tokio::select! {
            Some(message) = receiver.recv() => {
                if socket.send(message).await.is_err() {
                    break;
                }
            }
            received = socket.recv() => {
                match received {
                    Some(Ok(Message::Text(text))) => {
                        let affected_server_ids = handle_inbound_message(&state, &hub, &claims, &session_id, &text).await;
                        for server_id in affected_server_ids {
                            broadcast_server_presence_snapshot(&state, &hub, &server_id).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
        }
    }

    hub.unregister(&user_id, &session_id).await;
    if let Some(server_id) = hub.remove_server_player_by_session(&session_id).await {
        broadcast_server_presence_snapshot(&state, &hub, &server_id).await;
    }
}

async fn handle_inbound_message(
    state: &Arc<AppState>,
    hub: &PresenceWsHub,
    claims: &Claims,
    session_id: &str,
    text: &str,
) -> Vec<String> {
    let Ok(message) = serde_json::from_str::<PresenceSocketMessage>(text) else {
        return Vec::new();
    };

    let message_type = message.message_type.trim();
    if message_type != "multiplayer_join" && message_type != "multiplayer_leave" {
        return Vec::new();
    }

    let mut affected_server_ids = Vec::new();
    let server = if message_type == "multiplayer_join" {
        find_server_payload(state, &message.payload.game_domain).await
    } else {
        None
    };

    if message_type == "multiplayer_join" {
        if let Some(server) = server.as_ref() {
            let player = ServerPresencePlayer {
                user_id: claims.sub.clone(),
                player_id: message.payload.player_id.clone(),
                nickname: message.payload.nickname.clone(),
            };
            affected_server_ids = hub.upsert_server_player(session_id, &server.id, player).await;
        }
    } else if let Some(server_id) = hub.remove_server_player_by_session(session_id).await {
        affected_server_ids.push(server_id);
    }

    let friend_ids = friend_user_ids(state, &claims.sub).await.unwrap_or_default();
    if !friend_ids.is_empty() {
        let outbound = PresenceSocketOutbound {
            message_type: message_type.to_string(),
            payload: PresenceSocketOutboundPayload {
                user_id: claims.sub.clone(),
                player_id: message.payload.player_id,
                nickname: message.payload.nickname,
                server,
                is_friend: true,
            },
        };

        if let Ok(serialized) = serde_json::to_string(&outbound) {
            hub.send_to_users(&friend_ids, serialized).await;
        }
    }

    affected_server_ids
}

fn presence_hub() -> &'static PresenceWsHub {
    PRESENCE_HUB.get_or_init(PresenceWsHub::default)
}

async fn resolve_server_subscription(
    state: &Arc<AppState>,
    query: &PresenceSocketQuery,
) -> Result<Option<PresenceSocketServerPayload>, StatusCode> {
    if !query.include_all_players {
        return Ok(None);
    }

    let Some(server_id) = query.server_id.as_deref().map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };

    let server = Server::find_by_id(server_id.to_string())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Some(server_to_payload(server)))
}

async fn find_server_payload(state: &Arc<AppState>, game_domain: &str) -> Option<PresenceSocketServerPayload> {
    let normalized_domain = game_domain.trim().trim_end_matches('/').to_string();
    let server = Server::find()
        .filter(server::Column::GameDomain.eq(normalized_domain))
        .one(&state.db)
        .await
        .ok()
        .flatten()?;

    Some(server_to_payload(server))
}

async fn find_server_payload_by_id(state: &Arc<AppState>, server_id: &str) -> Option<PresenceSocketServerPayload> {
    Server::find_by_id(server_id.to_string())
        .one(&state.db)
        .await
        .ok()
        .flatten()
        .map(server_to_payload)
}

fn server_to_payload(server: server::Model) -> PresenceSocketServerPayload {
    PresenceSocketServerPayload {
        id: server.id,
        name: server.name,
        game_domain: server.game_domain,
    }
}

async fn broadcast_server_presence_snapshot(state: &Arc<AppState>, hub: &PresenceWsHub, server_id: &str) {
    let subscribers = hub.server_subscribers(server_id).await;
    if subscribers.is_empty() {
        return;
    }

    for subscriber in subscribers {
        if let Some(snapshot) = server_presence_snapshot_message(state, hub, server_id, &subscriber.user_id, None).await {
            let _ = subscriber.sender.send(Message::Text(snapshot));
        }
    }
}

async fn server_presence_snapshot_message(
    state: &Arc<AppState>,
    hub: &PresenceWsHub,
    server_id: &str,
    viewer_user_id: &str,
    known_server: Option<PresenceSocketServerPayload>,
) -> Option<String> {
    let server = match known_server {
        Some(server) => server,
        None => find_server_payload_by_id(state, server_id).await?,
    };
    let friend_ids = friend_user_ids(state, viewer_user_id).await.unwrap_or_default();
    let friend_ids = friend_ids.into_iter().collect::<HashSet<_>>();
    let players = hub
        .server_players(server_id)
        .await
        .into_iter()
        .map(|player| {
            let is_friend = friend_ids.contains(&player.user_id);
            ServerPresencePlayerOutbound {
                central_user_id: player.user_id.clone(),
                user_id: player.user_id,
                player_id: player.player_id,
                nickname: player.nickname,
                is_friend,
            }
        })
        .collect::<Vec<_>>();

    serde_json::to_string(&ServerPresenceSnapshotOutbound {
        message_type: "server_presence_snapshot",
        payload: ServerPresenceSnapshotPayload {
            server,
            current_connected_players: players.len(),
            players,
        },
    })
    .ok()
}

async fn friend_user_ids(state: &Arc<AppState>, user_id: &str) -> Result<Vec<String>, sea_orm::DbErr> {
    let friendships = Friendship::find()
        .filter(
            Condition::any()
                .add(friendship::Column::UserAId.eq(user_id))
                .add(friendship::Column::UserBId.eq(user_id)),
        )
        .all(&state.db)
        .await?;

    Ok(friendships
        .into_iter()
        .map(|friendship| {
            if friendship.user_a_id == user_id {
                friendship.user_b_id
            } else {
                friendship.user_a_id
            }
        })
        .collect())
}
