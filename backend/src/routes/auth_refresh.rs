use axum::{extract::{Extension, State}, http::StatusCode, response::Json};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    dto::UserResponse,
    models::{user, User},
    services::{consume_refresh_token, issue_refresh_token, revoke_refresh_token, Claims},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct RefreshPayload {
    pub refresh: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshIssueResponse {
    pub refresh: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshRotateResponse {
    pub token: String,
    pub refresh: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct RefreshRevokeResponse {
    pub revoked: bool,
}

pub async fn issue(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RefreshIssueResponse>, StatusCode> {
    let refresh = issue_refresh_token(&state.db, &claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RefreshIssueResponse { refresh }))
}

pub async fn rotate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<Json<RefreshRotateResponse>, StatusCode> {
    let consumed = consume_refresh_token(&state.db, payload.refresh.trim())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = User::find_by_id(consumed.user_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = state
        .jwt_service
        .generate_token(&user.id, &user.username)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let refresh = issue_refresh_token(&state.db, &user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RefreshRotateResponse {
        token,
        refresh,
        user: user_to_response(user),
    }))
}

pub async fn revoke(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<Json<RefreshRevokeResponse>, StatusCode> {
    revoke_refresh_token(&state.db, payload.refresh.trim())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RefreshRevokeResponse { revoked: true }))
}

fn user_to_response(user: user::Model) -> UserResponse {
    UserResponse {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        avatar: user.avatar,
        bio: user.bio,
        birthdate: user.birthdate.to_string(),
        age_verified: user.age_verified,
        discord_username: user.discord_username,
        created_at: user.created_at.to_string(),
    }
}
