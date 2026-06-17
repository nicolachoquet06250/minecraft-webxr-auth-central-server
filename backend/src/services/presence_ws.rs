use axum::extract::ws::Message;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct PresenceWsHub {
    sessions: Arc<RwLock<HashMap<String, Vec<PresenceSession>>>>,
}

#[derive(Clone)]
struct PresenceSession {
    id: String,
    sender: mpsc::UnboundedSender<Message>,
}

impl PresenceWsHub {
    pub async fn register(&self, user_id: &str) -> (String, mpsc::UnboundedReceiver<Message>) {
        let session_id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut sessions = self.sessions.write().await;
        sessions
            .entry(user_id.to_string())
            .or_default()
            .push(PresenceSession {
                id: session_id.clone(),
                sender,
            });
        (session_id, receiver)
    }

    pub async fn unregister(&self, user_id: &str, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        if let Some(user_sessions) = sessions.get_mut(user_id) {
            user_sessions.retain(|session| session.id != session_id);
            if user_sessions.is_empty() {
                sessions.remove(user_id);
            }
        }
    }

    pub async fn send_to_users(&self, user_ids: &[String], message: String) {
        let mut sessions = self.sessions.write().await;
        for user_id in user_ids {
            if let Some(user_sessions) = sessions.get_mut(user_id) {
                user_sessions.retain(|session| session.sender.send(Message::Text(message.clone())).is_ok());
            }
        }
        sessions.retain(|_, user_sessions| !user_sessions.is_empty());
    }
}
