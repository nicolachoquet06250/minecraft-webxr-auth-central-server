use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateServerRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    
    #[validate(url)]
    pub relay_domain: String,
    
    #[validate(url)]
    pub game_domain: String,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateServerRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
    
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ServerResponse {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub relay_domain: String,
    pub game_domain: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}
