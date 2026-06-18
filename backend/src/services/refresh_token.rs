use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::{refresh_token, RefreshToken};

const REFRESH_TOKEN_DAYS: i64 = 30;

pub fn generate_refresh_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(96)
        .map(char::from)
        .collect()
}

pub fn hash_refresh_token(token: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub async fn issue_refresh_token(db: &DatabaseConnection, user_id: &str) -> Result<String, sea_orm::DbErr> {
    let token = generate_refresh_token();
    let now = Utc::now().naive_utc();
    let expires_at = now + Duration::days(REFRESH_TOKEN_DAYS);

    refresh_token::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        token_hash: Set(hash_refresh_token(&token)),
        user_id: Set(user_id.to_string()),
        expires_at: Set(expires_at),
        revoked_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(db)
    .await?;

    Ok(token)
}

pub async fn consume_refresh_token(db: &DatabaseConnection, token: &str) -> Result<Option<refresh_token::Model>, sea_orm::DbErr> {
    let token_hash = hash_refresh_token(token);
    let existing = RefreshToken::find()
        .filter(refresh_token::Column::TokenHash.eq(token_hash))
        .one(db)
        .await?;

    let Some(existing) = existing else {
        return Ok(None);
    };

    if existing.revoked_at.is_some() || existing.expires_at < Utc::now().naive_utc() {
        return Ok(None);
    }

    let now = Utc::now().naive_utc();
    let mut active: refresh_token::ActiveModel = existing.clone().into();
    active.revoked_at = Set(Some(now));
    active.updated_at = Set(now);
    active.update(db).await?;

    Ok(Some(existing))
}

pub async fn revoke_refresh_token(db: &DatabaseConnection, token: &str) -> Result<(), sea_orm::DbErr> {
    let token_hash = hash_refresh_token(token);
    if let Some(existing) = RefreshToken::find()
        .filter(refresh_token::Column::TokenHash.eq(token_hash))
        .one(db)
        .await? {
        let now = Utc::now().naive_utc();
        let mut active: refresh_token::ActiveModel = existing.into();
        active.revoked_at = Set(Some(now));
        active.updated_at = Set(now);
        active.update(db).await?;
    }

    Ok(())
}
