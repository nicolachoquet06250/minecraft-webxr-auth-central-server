use hyper::{body::Bytes, Method, Request, Uri};
use hyper_util::client::legacy::Client;
use hyper_rustls::HttpsConnectorBuilder;
use http_body_util::{BodyExt, Full};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

const DISCORD_API_BASE: &str = "https://discord.com/api/v10";

#[derive(Debug)]
pub enum DiscordError {
    Http(String),
    Json(serde_json::Error),
    Body(String),
}

impl fmt::Display for DiscordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiscordError::Http(e) => write!(f, "HTTP error: {}", e),
            DiscordError::Json(e) => write!(f, "JSON error: {}", e),
            DiscordError::Body(e) => write!(f, "Body error: {}", e),
        }
    }
}

impl Error for DiscordError {}

impl From<serde_json::Error> for DiscordError {
    fn from(err: serde_json::Error) -> Self {
        DiscordError::Json(err)
    }
}

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
    client: Client<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>, Full<Bytes>>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl DiscordService {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);

        Self {
            client,
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

    pub async fn exchange_code(&self, code: &str) -> Result<DiscordTokenResponse, DiscordError> {
        let params = format!(
            "client_id={}&client_secret={}&grant_type=authorization_code&code={}&redirect_uri={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.client_secret),
            urlencoding::encode(code),
            urlencoding::encode(&self.redirect_uri)
        );

        let uri: Uri = format!("{}/oauth2/token", DISCORD_API_BASE)
            .parse()
            .map_err(|e| DiscordError::Http(format!("Invalid URI: {}", e)))?;

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Full::new(Bytes::from(params)))
            .map_err(|e| DiscordError::Http(format!("Failed to build request: {}", e)))?;

        let res = self
            .client
            .request(req)
            .await
            .map_err(|e| DiscordError::Http(format!("Request failed: {}", e)))?;

        let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| DiscordError::Body(format!("Failed to read body: {}", e)))?
            .to_bytes();

        serde_json::from_slice(&body).map_err(DiscordError::from)
    }

    pub async fn get_user(&self, access_token: &str) -> Result<DiscordUser, DiscordError> {
        let uri: Uri = format!("{}/users/@me", DISCORD_API_BASE)
            .parse()
            .map_err(|e| DiscordError::Http(format!("Invalid URI: {}", e)))?;

        let req = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("authorization", format!("Bearer {}", access_token))
            .body(Full::new(Bytes::new()))
            .map_err(|e| DiscordError::Http(format!("Failed to build request: {}", e)))?;

        let res = self
            .client
            .request(req)
            .await
            .map_err(|e| DiscordError::Http(format!("Request failed: {}", e)))?;

        let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| DiscordError::Body(format!("Failed to read body: {}", e)))?
            .to_bytes();

        serde_json::from_slice(&body).map_err(DiscordError::from)
    }
}
