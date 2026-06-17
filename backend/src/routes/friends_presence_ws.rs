use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Query, State},
    http::StatusCode,
    response::Response,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    models::{friendship, server, Friendship, Server},
    services::Claims,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct PresenceSocketQuery {
    pub auth: String,
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
}

#[derive(Debug, Serialize)]
struct PresenceSocketServerPayload {
    id: String,
    name: String,
    game_domain: String,
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

    Ok(ws.on_upgrade(move |socket| handle_socket(state, claims, socket)))
}

async fn handle_socket(state: Arc<AppState>, claims: Claims, mut socket: WebSocket) {
    let user_id = claims.sub.clone();
    let (session_id, mut receiver) = state.presence_ws_hub.register(&user_id).await;

    loop {
        tokio::select! {
            Some(message) = receiver.recv() => {
                if socket.send(message).await.is_err() {
                    break;
                }
            }
            received = socket.recv() => {
                match received {
                    Some(Ok(Message::Text(text))) => handle_inbound_message(&state, &claims, &text).await,
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
        }
    }

    state.presence_ws_hub.unregister(&user_id, &session_id).await;
}

async fn handle_inbound_message(state: &Arc<AppState>, claims: &Claims, text: &str) {
    let Ok(message) = serde_json::from_str::<PresenceSocketMessage>(text) else {
        return;
    };

    let message_type = message.message_type.trim();
    if message_type != "multiplayer_join" && message_type != "multiplayer_leave" {
        return;
    }

    let friend_ids = friend_user_ids(state, &claims.sub).await.unwrap_or_default();
    if friend_ids.is_empty() {
        return;
    }

    let server = if message_type == "multiplayer_join" {
        find_server_payload(state, &message.payload.game_domain).await
    } else {
        None
    };

    let outbound = PresenceSocketOutbound {
        message_type: message_type.to_string(),
        payload: PresenceSocketOutboundPayload {
            user_id: claims.sub.clone(),
            player_id: message.payload.player_id,
            nickname: message.payload.nickname,
            server,
        },
    };

    if let Ok(serialized) = serde_json::to_string(&outbound) {
        state.presence_ws_hub.send_to_users(&friend_ids, serialized).await;
    }
}

async fn find_server_payload(state: &Arc<AppState>, game_domain: &str) -> Option<PresenceSocketServerPayload> {
    let normalized_domain = game_domain.trim().trim_end_matches('/').to_string();
    let server = Server::find()
        .filter(server::Column::GameDomain.eq(normalized_domain))
        .one(&state.db)
        .await
        .ok()
        .flatten()?;

    Some(PresenceSocketServerPayload {
        id: server.id,
        name: server.name,
        game_domain: server.game_domain,
    })
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
