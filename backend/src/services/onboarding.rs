use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Uniform, Rng};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct OnboardingStore {
    entries: Arc<Mutex<HashMap<String, OnboardingEntry>>>,
}

#[derive(Clone)]
pub struct OnboardingEntry {
    pub username: String,
    pub email: String,
    pub hash: String,
    pub avatar: String,
    pub birthdate: String,
    pub bio: Option<String>,
    token: String,
    expires_at: DateTime<Utc>,
}

impl OnboardingStore {
    pub fn new() -> Self {
        Self { entries: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub async fn put(&self, username: String, email: String, hash: String, avatar: String, birthdate: String, bio: Option<String>) -> String {
        let token = random_token();
        let entry = OnboardingEntry {
            username,
            email: email.clone(),
            hash,
            avatar,
            birthdate,
            bio,
            token: token.clone(),
            expires_at: Utc::now() + Duration::minutes(15),
        };
        self.entries.lock().await.insert(email.to_lowercase(), entry);
        token
    }

    pub async fn take(&self, email: &str, token: &str) -> Option<OnboardingEntry> {
        let mut entries = self.entries.lock().await;
        let key = email.to_lowercase();
        let entry = entries.get(&key)?;
        if entry.expires_at < Utc::now() || entry.token != token.trim() {
            return None;
        }
        entries.remove(&key)
    }
}

fn random_token() -> String {
    let range = Uniform::from(0..10);
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.sample(range).to_string()).collect()
}
