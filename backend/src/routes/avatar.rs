use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{ActiveAvatarResponse, AvatarResponse, SaveAvatarRequest, UpdateAvatarRequest},
    models::{avatar, Avatar},
    services::Claims,
    AppState,
};

pub async fn list_avatars(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AvatarResponse>>, StatusCode> {
    let avatars = Avatar::find()
        .filter(avatar::Column::UserId.eq(claims.sub))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(avatars.into_iter().map(to_response).collect()))
}

pub async fn get_active_avatar(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ActiveAvatarResponse>, StatusCode> {
    let avatar = Avatar::find()
        .filter(avatar::Column::UserId.eq(claims.sub))
        .filter(avatar::Column::IsActive.eq(true))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ActiveAvatarResponse {
        kind: if avatar.is_some() { "custom" } else { "default" }.to_string(),
        avatar: avatar.map(to_response),
    }))
}

pub async fn create_avatar_copy(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SaveAvatarRequest>,
) -> Result<Json<AvatarResponse>, StatusCode> {
    payload.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
    let now = chrono::Utc::now().naive_utc();
    let id = Uuid::new_v4().to_string();
    let texture_data = serde_json::to_string(&payload.texture_data).map_err(|_| StatusCode::BAD_REQUEST)?;

    let created = avatar::ActiveModel {
        id: Set(id),
        user_id: Set(claims.sub),
        name: Set(payload.name),
        base_kind: Set(normalize_base_kind(&payload.base_kind)),
        is_active: Set(false),
        texture_data: Set(texture_data),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(to_response(created)))
}

pub async fn update_avatar(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(avatar_id): Path<String>,
    Json(payload): Json<UpdateAvatarRequest>,
) -> Result<Json<AvatarResponse>, StatusCode> {
    payload.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    if avatar_id == "steve" || avatar_id == "alex" {
        return Err(StatusCode::FORBIDDEN);
    }

    let avatar = Avatar::find_by_id(avatar_id)
        .filter(avatar::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active: avatar::ActiveModel = avatar.into();
    if let Some(name) = payload.name {
        active.name = Set(name);
    }
    active.texture_data = Set(serde_json::to_string(&payload.texture_data).map_err(|_| StatusCode::BAD_REQUEST)?);
    active.updated_at = Set(chrono::Utc::now().naive_utc());

    let updated = active
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(to_response(updated)))
}

pub async fn select_avatar(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(avatar_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let owned = Avatar::find_by_id(avatar_id.clone())
        .filter(avatar::Column::UserId.eq(claims.sub.clone()))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let avatars = Avatar::find()
        .filter(avatar::Column::UserId.eq(claims.sub))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for item in avatars {
        let should_activate = item.id == owned.id;
        let mut active: avatar::ActiveModel = item.into();
        active.is_active = Set(should_activate);
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active.update(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

fn normalize_base_kind(value: &str) -> String {
    match value {
        "steve" => "steve".to_string(),
        "alex" => "alex".to_string(),
        _ => "custom".to_string(),
    }
}

fn to_response(model: avatar::Model) -> AvatarResponse {
    AvatarResponse {
        id: model.id,
        name: model.name,
        base_kind: model.base_kind,
        is_active: model.is_active,
        texture_data: serde_json::from_str::<Value>(&model.texture_data).unwrap_or(Value::Null),
        created_at: model.created_at.to_string(),
        updated_at: model.updated_at.to_string(),
    }
}
