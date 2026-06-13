use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

use crate::{
    models::{avatar, Avatar, User},
    services::Claims,
    AppState,
};

#[derive(Debug, Serialize)]
pub struct MatrixColorResponse {
    pub user_id: String,
    pub kind: String,
    pub base_kind: String,
    pub texture_data: Option<Value>,
}

pub async fn matrix_color_preflight() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn get_user_matrix_color(
    Extension(_claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<MatrixColorResponse>, StatusCode> {
    let user = User::find_by_id(user_id.clone())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let active_avatar = Avatar::find()
        .filter(avatar::Column::UserId.eq(user_id.clone()))
        .filter(avatar::Column::IsActive.eq(true))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(active_avatar) = active_avatar {
        let texture_data = serde_json::from_str::<Value>(&active_avatar.texture_data)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(Json(MatrixColorResponse {
            user_id,
            kind: "custom".to_string(),
            base_kind: active_avatar.base_kind,
            texture_data: Some(texture_data),
        }));
    }

    Ok(Json(MatrixColorResponse {
        user_id,
        kind: "default".to_string(),
        base_kind: user.avatar,
        texture_data: None,
    }))
}
