use reqwest::Client;
use serde::{Deserialize, Serialize};

const DISCORD_API_BASE: &str = "https://discord.com/api/v10";

#[derive(Debug, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub email: Option<String>,
    pub verified: Option<bool>,
}

pub struct DiscordService {
    client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl DiscordService {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub fn get_oauth_url(&self, state: Option<&str>) -> String {
        let state_param = state.unwrap_or("random_state");
        format!(
            "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify%20email&state={}",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            state_param
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<DiscordTokenResponse, reqwest::Error> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        self.client
            .post(&format!("{}/oauth2/token", DISCORD_API_BASE))
            .form(&params)
            .send()
            .await?
            .json::<DiscordTokenResponse>()
            .await
    }

    pub async fn get_user(&self, access_token: &str) -> Result<DiscordUser, reqwest::Error> {
        self.client
            .get(&format!("{}/users/@me", DISCORD_API_BASE))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<DiscordUser>()
            .await
    }
}
