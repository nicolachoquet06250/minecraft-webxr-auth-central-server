use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    #[schema(min_length = 3, max_length = 20)]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    #[schema(format = Password, min_length = 8)]
    pub password: String,
    
    #[schema(example = "steve")]
    pub avatar: String, // "steve" or "alex"
    
    #[schema(format = Date)]
    pub birthdate: String, // YYYY-MM-DD format
    
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ConfirmRegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 6))]
    #[schema(min_length = 6, max_length = 6)]
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterCodeResponse {
    pub sent: bool,
    pub expires_in_minutes: u8,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "player@example.com")]
    pub email: Option<String>,
    
    #[schema(format = Password)]
    pub password: Option<String>,

    pub central_join_ticket: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    #[schema(pattern = "^[0-9a-fA-F-]{36}$")]
    pub id: String,
    pub username: String,
    pub email: String,
    #[schema(example = "steve")]
    pub avatar: String,
    pub bio: Option<String>,
    #[schema(format = Date)]
    pub birthdate: String,
    pub age_verified: bool,
    pub discord_username: Option<String>,
    #[schema(format = DateTime)]
    pub created_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 20))]
    #[schema(min_length = 3, max_length = 20)]
    pub username: Option<String>,
    
    #[schema(example = "alex")]
    pub avatar: Option<String>,
    
    #[validate(length(max = 500))]
    #[schema(max_length = 500)]
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DiscordCallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DiscordOAuthUrl {
    pub url: String,
}
