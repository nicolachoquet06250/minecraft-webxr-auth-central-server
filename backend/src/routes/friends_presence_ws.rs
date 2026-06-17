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
