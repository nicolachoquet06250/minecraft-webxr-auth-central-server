use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use validator::Validate;

use crate::{
    dto::{UpdateUserRequest, UserResponse},
    models::{avatar, user, Avatar, User},
    services::Claims,
    AppState,
};

const USER_SEARCH_MIN_QUERY_LEN: usize = 2;
const USER_SEARCH_DEFAULT_PAGE: u64 = 1;
const USER_SEARCH_DEFAULT_PAGE_SIZE: u64 = 20;
const USER_SEARCH_MAX_PAGE_SIZE: u64 = 50;

#[derive(Debug, Deserialize)]
pub struct UserSearchQuery {
    pub q: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct UserSearchAvatar {
    pub kind: String,
    pub base_kind: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct UserSearchResult {
    pub id: String,
    pub username: String,
    pub avatar: UserSearchAvatar,
}

#[derive(Debug, Serialize)]
pub struct PaginatedUserSearchResponse {
    pub items: Vec<UserSearchResult>,
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub total_pages: u64,
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
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
) -> Result<Json<PaginatedUserSearchResponse>, StatusCode> {
    let query = params.q.as_deref().map(str::trim).filter(|value| !value.is_empty());
    if let Some(query) = query {
        if query.chars().count() < USER_SEARCH_MIN_QUERY_LEN {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let page = params.page.unwrap_or(USER_SEARCH_DEFAULT_PAGE).max(1);
    let page_size = params
        .page_size
        .unwrap_or(USER_SEARCH_DEFAULT_PAGE_SIZE)
        .clamp(1, USER_SEARCH_MAX_PAGE_SIZE);
    let offset = (page - 1) * page_size;

    let mut base_query = User::find().filter(user::Column::Id.ne(claims.sub.clone()));
    if let Some(query) = query {
        base_query = base_query.filter(user::Column::Username.contains(query));
    }

    let total = base_query
        .clone()
        .count(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users = base_query
        .order_by_asc(user::Column::Username)
        .offset(offset)
        .limit(page_size)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_ids = users.iter().map(|user| user.id.clone()).collect::<Vec<_>>();
    let active_avatars = if user_ids.is_empty() {
        Vec::new()
    } else {
        Avatar::find()
            .filter(avatar::Column::UserId.is_in(user_ids))
            .filter(avatar::Column::IsActive.eq(true))
            .all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };
    let active_avatars_by_user_id = active_avatars
        .into_iter()
        .map(|avatar| (avatar.user_id.clone(), avatar))
        .collect::<HashMap<_, _>>();

    let items = users
        .into_iter()
        .map(|user| user_to_search_result(user, &active_avatars_by_user_id))
        .collect();
    let total_pages = if total == 0 { 0 } else { total.div_ceil(page_size) };
    let next_url = if page < total_pages {
        Some(user_search_page_url(query, page + 1, page_size))
    } else {
        None
    };
    let previous_url = if page > 1 && total_pages > 0 {
        Some(user_search_page_url(query, page - 1, page_size))
    } else {
        None
    };

    Ok(Json(PaginatedUserSearchResponse {
        items,
        page,
        page_size,
        total,
        total_pages,
        next_url,
        previous_url,
    }))
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

fn user_to_search_result(user: user::Model, active_avatars_by_user_id: &HashMap<String, avatar::Model>) -> UserSearchResult {
    let avatar_url = format!("/api/users/{}/profile-pic.svg", user.id);
    let avatar = if let Some(active_avatar) = active_avatars_by_user_id.get(&user.id) {
        UserSearchAvatar {
            kind: "custom".to_string(),
            base_kind: active_avatar.base_kind.clone(),
            name: active_avatar.name.clone(),
            url: avatar_url,
        }
    } else {
        let base_kind = user.avatar.clone();
        UserSearchAvatar {
            kind: "default".to_string(),
            name: base_avatar_name(&base_kind).to_string(),
            base_kind,
            url: avatar_url,
        }
    };

    UserSearchResult {
        id: user.id,
        username: user.username,
        avatar,
    }
}

fn user_search_page_url(query: Option<&str>, page: u64, page_size: u64) -> String {
    if let Some(query) = query {
        format!(
            "/api/users/search?q={}&page={}&page_size={}",
            urlencoding::encode(query),
            page,
            page_size,
        )
    } else {
        format!("/api/users/search?page={}&page_size={}", page, page_size)
    }
}

fn base_avatar_name(base_kind: &str) -> &str {
    match base_kind {
        "steve" => "Steve",
        "alex" => "Alex",
        _ => "Avatar par défaut",
    }
}
