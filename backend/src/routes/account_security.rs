use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    models::{user, User},
    services::{hash_password, Claims},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct RequestCredentialCodeRequest {
    pub next_secret: String,
    pub next_secret_confirmation: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmCredentialChangeRequest {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct CredentialCodeResponse {
    pub sent: bool,
    pub expires_in_minutes: u8,
}

#[derive(Debug, Serialize)]
pub struct CredentialChangedResponse {
    pub changed: bool,
}

pub async fn request_credential_code(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RequestCredentialCodeRequest>,
) -> Result<Json<CredentialCodeResponse>, StatusCode> {
    if payload.next_secret.len() < 8 || payload.next_secret.len() > 128 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if payload.next_secret != payload.next_secret_confirmation {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = load_user(&state, &claims.sub).await?;
    let next_hash = hash_password(&payload.next_secret).map_err(|error| {
        tracing::error!(?error, "failed to hash pending account credential");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let code = state.password_change_codes.create_code(&claims.sub, next_hash).await;
    state
        .mail_service
        .send_password_change_code_email(&user.email, &user.username, &code)
        .await
        .map_err(|error| {
            tracing::error!(?error, "failed to send account security code email");
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    Ok(Json(CredentialCodeResponse { sent: true, expires_in_minutes: 10 }))
}

pub async fn confirm_credential_change(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ConfirmCredentialChangeRequest>,
) -> Result<Json<CredentialChangedResponse>, StatusCode> {
    let new_hash = state
        .password_change_codes
        .verify_and_consume(&claims.sub, &payload.code)
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = load_user(&state, &claims.sub).await?;
    let mut active_user: user::ActiveModel = user.into();
    active_user.password_hash = Set(Some(new_hash));
    active_user.updated_at = Set(Utc::now().naive_utc());
    active_user.update(&state.db).await.map_err(|error| {
        tracing::error!(?error, "failed to update account credential");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CredentialChangedResponse { changed: true }))
}

async fn load_user(state: &AppState, user_id: &str) -> Result<user::Model, StatusCode> {
    User::find()
        .filter(user::Column::Id.eq(user_id))
        .one(&state.db)
        .await
        .map_err(|error| {
            tracing::error!(?error, "failed to load user account");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)
}
