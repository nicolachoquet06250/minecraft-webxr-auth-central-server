use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    
    #[sea_orm(unique)]
    pub username: String,
    
    #[sea_orm(unique)]
    pub email: String,
    
    pub password_hash: Option<String>,
    
    pub avatar: String, // "steve" or "alex"
    
    pub bio: Option<String>,
    
    pub birthdate: Date,
    
    pub age_verified: bool,
    
    #[sea_orm(unique)]
    pub discord_id: Option<String>,
    
    pub discord_username: Option<String>,
    
    pub created_at: DateTime,
    
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::server::Entity")]
    Servers,
}

impl Related<super::server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Servers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
