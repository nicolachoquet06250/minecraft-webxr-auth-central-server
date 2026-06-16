use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    dto::{UpdateUserRequest, UserResponse},
    models::{user, User},
    services::Claims,
    AppState,
};

const USER_SEARCH_MIN_QUERY_LEN: usize = 2;
const USER_SEARCH_DEFAULT_LIMIT: u64 = 20;
const USER_SEARCH_MAX_LIMIT: u64 = 50;

#[derive(Debug, Deserialize)]
pub struct UserSearchQuery {
    pub q: String,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct UserSearchResult {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub avatar_url: String,
}

pub async fn get_profile(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>) -> Result<Json<UserResponse>, StatusCode> {
    let user_id = claims.sub.clone();
    let user = User::find_by_id(user_id).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user_to_response(user)))
}

pub async fn search_users(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<UserSearchQuery>,
) -> Result<Json<Vec<UserSearchResult>>, StatusCode> {
    let query = params.q.trim();
    if query.chars().count() < USER_SEARCH_MIN_QUERY_LEN {
        return Err(StatusCode::BAD_REQUEST);
    }

    let limit = params
        .limit
        .unwrap_or(USER_SEARCH_DEFAULT_LIMIT)
        .min(USER_SEARCH_MAX_LIMIT);

    let users = User::find()
        .filter(user::Column::Id.ne(&claims.sub))
        .filter(user::Column::Username.contains(query))
        .order_by_asc(user::Column::Username)
        .limit(limit)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let results = users
        .into_iter()
        .map(user_to_search_result)
        .collect();

    Ok(Json(results))
}

pub async fn get_user_by_id(Path(user_id): Path<String>, State(state): State<Arc<AppState>>) -> Result<Json<UserResponse>, StatusCode> {
    let user = User::find_by_id(user_id).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user_to_response(user)))
}

pub async fn update_profile(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>, Json(payload): Json<UpdateUserRequest>) -> Result<Json<UserResponse>, StatusCode> {
    if payload.validate().is_err() { return Err(StatusCode::BAD_REQUEST); }
    let user_id = claims.sub.clone();
    let user = User::find_by_id(user_id).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let mut user_active: user::ActiveModel = user.into();
    if let Some(username) = payload.username { user_active.username = Set(username); }
    if let Some(avatar) = payload.avatar { user_active.avatar = Set(avatar); }
    if let Some(bio) = payload.bio { user_active.bio = Set(Some(bio)); }
    user_active.updated_at = Set(chrono::Utc::now().naive_utc());
    let updated_user = user_active.update(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user_to_response(updated_user)))
}

pub async fn unlink_discord(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>) -> Result<Json<UserResponse>, StatusCode> {
    let user_id = claims.sub.clone();
    let user = User::find_by_id(user_id).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let mut user_active: user::ActiveModel = user.into();
    user_active.discord_id = Set(None);
    user_active.discord_username = Set(None);
    user_active.updated_at = Set(chrono::Utc::now().naive_utc());
    let updated_user = user_active.update(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user_to_response(updated_user)))
}

pub async fn delete_account(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>) -> Result<StatusCode, StatusCode> {
    let user_id = claims.sub.clone();
    let user = User::find_by_id(user_id).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let user_active: user::ActiveModel = user.into();
    user_active.delete(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

fn user_to_response(user: user::Model) -> UserResponse {
    UserResponse { id: user.id.to_string(), username: user.username, email: user.email, avatar: user.avatar, bio: user.bio, birthdate: user.birthdate.to_string(), age_verified: user.age_verified, discord_username: user.discord_username, created_at: user.created_at.to_string() }
}

fn user_to_search_result(user: user::Model) -> UserSearchResult {
    UserSearchResult {
        avatar_url: format!("/api/users/{}/profile-pic.svg", user.id),
        id: user.id,
        username: user.username,
        avatar: user.avatar,
    }
}
