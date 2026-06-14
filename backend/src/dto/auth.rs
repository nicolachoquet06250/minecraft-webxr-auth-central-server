use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    pub avatar: String, // "steve" or "alex"
    
    pub birthdate: String, // YYYY-MM-DD format
    
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ConfirmRegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 6))]
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterCodeResponse {
    pub sent: bool,
    pub expires_in_minutes: u8,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar: String,
    pub bio: Option<String>,
    pub birthdate: String,
    pub age_verified: bool,
    pub discord_username: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
    
    pub avatar: Option<String>,
    
    #[validate(length(max = 500))]
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DiscordCallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DiscordOAuthUrl {
    pub url: String,
}
