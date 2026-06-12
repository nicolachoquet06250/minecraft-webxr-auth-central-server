use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct AvatarResponse {
    pub id: String,
    pub name: String,
    pub base_kind: String,
    pub is_active: bool,
    pub texture_data: Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ActiveAvatarResponse {
    pub kind: String,
    pub avatar: Option<AvatarResponse>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SaveAvatarRequest {
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    #[validate(length(min = 1, max = 20))]
    pub base_kind: String,
    pub texture_data: Value,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAvatarRequest {
    #[validate(length(min = 1, max = 80))]
    pub name: Option<String>,
    pub texture_data: Value,
}
