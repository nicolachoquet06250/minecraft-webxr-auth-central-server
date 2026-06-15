use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateServerRequest {
    #[validate(length(min = 3, max = 50))]
    #[schema(min_length = 3, max_length = 50)]
    pub name: String,

    #[validate(url)]
    #[schema(format = Uri)]
    pub game_domain: String,

    #[validate(length(max = 500))]
    #[schema(max_length = 500)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateServerRequest {
    #[validate(length(min = 3, max = 50))]
    #[schema(min_length = 3, max_length = 50)]
    pub name: Option<String>,

    #[validate(length(max = 500))]
    #[schema(max_length = 500)]
    pub description: Option<String>,

    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RecordServerVisitRequest {
    #[schema(format = Uri)]
    pub server_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerResponse {
    #[schema(pattern = "^[0-9a-fA-F-]{36}$")]
    pub id: String,
    #[schema(pattern = "^[0-9a-fA-F-]{36}$")]
    pub owner_id: String,
    pub name: String,
    pub game_domain: String,
    pub description: Option<String>,
    pub is_active: bool,
    #[schema(format = DateTime)]
    pub created_at: String,
    #[schema(format = DateTime)]
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerHistoryResponse {
    pub server: ServerResponse,
    pub is_favorite: bool,
    #[schema(format = DateTime)]
    pub visited_at: Option<String>,
    #[schema(format = DateTime)]
    pub favorited_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FavoriteServerResponse {
    pub server: ServerResponse,
    pub is_favorite: bool,
    #[schema(format = DateTime)]
    pub favorited_at: String,
}
