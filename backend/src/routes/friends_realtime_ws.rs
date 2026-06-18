use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Query, State},
    http::StatusCode,
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, sync::OnceLock};

use crate::{services::presence_ws::PresenceWsHub, AppState};

static FRIENDS_REALTIME_HUB: OnceLock<PresenceWsHub> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct FriendsRealtimeQuery {
    pub auth: String,
}

#[derive(Debug, Serialize)]
struct FriendsRealtimeEvent<'a> {
    #[serde(rename = "type")]
    event_type: &'a str,
    payload: FriendsRealtimePayload<'a>,
}

#[derive(Debug, Serialize)]
struct FriendsRealtimePayload<'a> {
    refresh_friends: bool,
    refresh_incoming_requests: bool,
    refresh_outgoing_requests: bool,
    incoming_request_count_changed: bool,
    notification: Option<FriendsRealtimeNotification<'a>>,
}

#[derive(Debug, Serialize)]
struct FriendsRealtimeNotification<'a> {
    title: &'a str,
    body: String,
}

pub async fn friends_realtime_socket(
    State(state): State<Arc<AppState>>,
    Query(query): Query<FriendsRealtimeQuery>,
    ws: WebSocketUpgrade,
) -> Result<Response, StatusCode> {
    let claims = state
        .jwt_service
        .verify_token(query.auth.trim())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(ws.on_upgrade(move |socket| handle_socket(claims.sub, socket)))
}

pub async fn notify_request_received(receiver_user_id: &str, requester_username: &str) {
    send_to_user(
        receiver_user_id,
        "friend_request_received",
        Some(FriendsRealtimeNotification {
            title: "Nouvelle demande d'ami",
            body: format!("{} t'a envoyé une demande d'ami.", requester_username),
        }),
    ).await;
}

pub async fn notify_request_accepted(requester_user_id: &str, receiver_username: &str) {
    send_to_user(
        requester_user_id,
        "friend_request_accepted",
        Some(FriendsRealtimeNotification {
            title: "Demande d'ami acceptée",
            body: format!("{} a accepté ta demande d'ami.", receiver_username),
        }),
    ).await;
}

pub async fn notify_friend_state_changed(user_ids: &[String]) {
    send_to_users(user_ids, "friends_state_changed", None).await;
}

async fn handle_socket(user_id: String, mut socket: WebSocket) {
    let hub = hub().clone();
    let (session_id, mut receiver) = hub.register(&user_id).await;

    while let Some(message) = receiver.recv().await {
        if socket.send(message).await.is_err() {
            break;
        }
    }

    hub.unregister(&user_id, &session_id).await;
}

async fn send_to_user(user_id: &str, event_type: &str, notification: Option<FriendsRealtimeNotification<'_>>) {
    send_to_users(&[user_id.to_string()], event_type, notification).await;
}

async fn send_to_users(user_ids: &[String], event_type: &str, notification: Option<FriendsRealtimeNotification<'_>>) {
    let event = FriendsRealtimeEvent {
        event_type,
        payload: FriendsRealtimePayload {
            refresh_friends: true,
            refresh_incoming_requests: true,
            refresh_outgoing_requests: true,
            incoming_request_count_changed: true,
            notification,
        },
    };

    if let Ok(serialized) = serde_json::to_string(&event) {
        hub().send_to_users(user_ids, serialized).await;
    }
}

fn hub() -> &'static PresenceWsHub {
    FRIENDS_REALTIME_HUB.get_or_init(PresenceWsHub::default)
}
