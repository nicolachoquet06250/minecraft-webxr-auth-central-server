use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Uniform, Rng};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct PasswordChangeCodeStore {
    codes: Arc<Mutex<HashMap<String, PasswordChangeCode>>>,
}

#[derive(Clone)]
struct PasswordChangeCode {
    code: String,
    pending_password_hash: String,
    expires_at: DateTime<Utc>,
}

impl PasswordChangeCodeStore {
    pub fn new() -> Self {
        Self {
            codes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_code(&self, user_id: &str, pending_password_hash: String) -> String {
        let code = generate_code();
        let expires_at = Utc::now() + Duration::minutes(10);
        let mut codes = self.codes.lock().await;
        codes.insert(
            user_id.to_string(),
            PasswordChangeCode {
                code: code.clone(),
                pending_password_hash,
                expires_at,
            },
        );
        code
    }

    pub async fn verify_and_consume(&self, user_id: &str, submitted_code: &str) -> Option<String> {
        let mut codes = self.codes.lock().await;
        let stored = codes.get(user_id)?;

        if stored.expires_at < Utc::now() || stored.code != submitted_code.trim() {
            return None;
        }

        codes.remove(user_id).map(|code| code.pending_password_hash)
    }
}

fn generate_code() -> String {
    let range = Uniform::from(0..10);
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.sample(range).to_string()).collect()
}
