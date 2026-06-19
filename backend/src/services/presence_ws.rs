use axum::extract::ws::Message;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ServerPresencePlayer {
    pub user_id: String,
    pub player_id: String,
    pub nickname: String,
}

#[derive(Clone)]
pub struct ServerPresenceSubscriber {
    pub user_id: String,
    pub sender: mpsc::UnboundedSender<Message>,
}

#[derive(Clone, Default)]
pub struct PresenceWsHub {
    state: Arc<RwLock<PresenceWsState>>,
}

#[derive(Default)]
struct PresenceWsState {
    user_sessions: HashMap<String, Vec<PresenceSession>>,
    server_sessions: HashMap<String, Vec<PresenceSession>>,
    server_players: HashMap<String, HashMap<String, ServerPresencePlayer>>,
    session_players: HashMap<String, (String, String)>,
}

#[derive(Clone)]
struct PresenceSession {
    id: String,
    user_id: String,
    sender: mpsc::UnboundedSender<Message>,
}

impl PresenceWsHub {
    pub async fn register(
        &self,
        user_id: &str,
        server_subscription_id: Option<&str>,
    ) -> (String, mpsc::UnboundedReceiver<Message>) {
        let session_id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::unbounded_channel();
        let session = PresenceSession {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            sender,
        };

        let mut state = self.state.write().await;
        state
            .user_sessions
            .entry(user_id.to_string())
            .or_default()
            .push(session.clone());

        if let Some(server_id) = server_subscription_id {
            state
                .server_sessions
                .entry(server_id.to_string())
                .or_default()
                .push(session);
        }

        (session_id, receiver)
    }

    pub async fn unregister(&self, user_id: &str, session_id: &str) {
        let mut state = self.state.write().await;
        if let Some(user_sessions) = state.user_sessions.get_mut(user_id) {
            user_sessions.retain(|session| session.id != session_id);
            if user_sessions.is_empty() {
                state.user_sessions.remove(user_id);
            }
        }

        for server_sessions in state.server_sessions.values_mut() {
            server_sessions.retain(|session| session.id != session_id);
        }
        state
            .server_sessions
            .retain(|_, server_sessions| !server_sessions.is_empty());
    }

    pub async fn upsert_server_player(
        &self,
        session_id: &str,
        server_id: &str,
        player: ServerPresencePlayer,
    ) -> Vec<String> {
        let mut state = self.state.write().await;
        let mut affected_server_ids = Vec::new();

        if let Some((previous_server_id, previous_player_key)) = state.session_players.remove(session_id) {
            if let Some(players) = state.server_players.get_mut(&previous_server_id) {
                players.remove(&previous_player_key);
                if players.is_empty() {
                    state.server_players.remove(&previous_server_id);
                }
            }
            affected_server_ids.push(previous_server_id);
        }

        let player_key = session_id.to_string();
        state
            .server_players
            .entry(server_id.to_string())
            .or_default()
            .insert(player_key.clone(), player);
        state
            .session_players
            .insert(session_id.to_string(), (server_id.to_string(), player_key));

        if !affected_server_ids.iter().any(|id| id == server_id) {
            affected_server_ids.push(server_id.to_string());
        }

        affected_server_ids
    }

    pub async fn remove_server_player_by_session(&self, session_id: &str) -> Option<String> {
        let mut state = self.state.write().await;
        let (server_id, player_key) = state.session_players.remove(session_id)?;
        if let Some(players) = state.server_players.get_mut(&server_id) {
            players.remove(&player_key);
            if players.is_empty() {
                state.server_players.remove(&server_id);
            }
        }
        Some(server_id)
    }

    pub async fn server_players(&self, server_id: &str) -> Vec<ServerPresencePlayer> {
        let state = self.state.read().await;
        state
            .server_players
            .get(server_id)
            .map(|players| players.values().cloned().collect())
            .unwrap_or_default()
    }

    pub async fn server_subscribers(&self, server_id: &str) -> Vec<ServerPresenceSubscriber> {
        let state = self.state.read().await;
        state
            .server_sessions
            .get(server_id)
            .map(|sessions| {
                sessions
                    .iter()
                    .map(|session| ServerPresenceSubscriber {
                        user_id: session.user_id.clone(),
                        sender: session.sender.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn send_to_users(&self, user_ids: &[String], message: String) {
        let mut state = self.state.write().await;
        for user_id in user_ids {
            if let Some(user_sessions) = state.user_sessions.get_mut(user_id) {
                user_sessions.retain(|session| session.sender.send(Message::Text(message.clone())).is_ok());
            }
        }
        state.user_sessions.retain(|_, user_sessions| !user_sessions.is_empty());
    }
}
