use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct AvatarResponse {
    #[schema(pattern = "^[0-9a-fA-F-]{36}$")]
    pub id: String,
    pub name: String,
    #[schema(example = "custom")]
    pub base_kind: String,
    pub is_active: bool,
    #[schema(value_type = Object)]
    pub texture_data: Value,
    #[schema(format = DateTime)]
    pub created_at: String,
    #[schema(format = DateTime)]
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ActiveAvatarResponse {
    pub kind: String,
    pub avatar: Option<AvatarResponse>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SaveAvatarRequest {
    #[validate(length(min = 1, max = 80))]
    #[schema(min_length = 1, max_length = 80)]
    pub name: String,
    #[validate(length(min = 1, max = 20))]
    #[schema(min_length = 1, max_length = 20)]
    pub base_kind: String,
    #[schema(value_type = Object)]
    pub texture_data: Value,
    #[validate(length(max = 1_500_000))]
    #[schema(max_length = 1500000)]
    pub preview_image_data_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAvatarRequest {
    #[validate(length(min = 1, max = 80))]
    #[schema(min_length = 1, max_length = 80)]
    pub name: Option<String>,
    #[schema(value_type = Object)]
    pub texture_data: Value,
    #[validate(length(max = 1_500_000))]
    #[schema(max_length = 1500000)]
    pub preview_image_data_url: Option<String>,
}
