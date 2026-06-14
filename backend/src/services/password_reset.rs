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
    expires_at: DateTime<Utc>,
}

impl PasswordChangeCodeStore {
    pub fn new() -> Self {
        Self {
            codes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_code(&self, user_id: &str) -> String {
        let code = generate_code();
        let expires_at = Utc::now() + Duration::minutes(10);
        let mut codes = self.codes.lock().await;
        codes.insert(
            user_id.to_string(),
            PasswordChangeCode {
                code: code.clone(),
                expires_at,
            },
        );
        code
    }

    pub async fn verify_and_consume(&self, user_id: &str, submitted_code: &str) -> bool {
        let mut codes = self.codes.lock().await;
        let Some(stored) = codes.get(user_id) else {
            return false;
        };

        if stored.expires_at < Utc::now() || stored.code != submitted_code.trim() {
            return false;
        }

        codes.remove(user_id);
        true
    }
}

fn generate_code() -> String {
    let range = Uniform::from(0..10);
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.sample(range).to_string()).collect()
}
