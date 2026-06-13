use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::{
    models::{avatar, Avatar},
    services::Claims,
    AppState,
};

pub async fn delete_avatar(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(avatar_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    if avatar_id == "steve" || avatar_id == "alex" {
        return Err(StatusCode::FORBIDDEN);
    }

    let avatar = Avatar::find_by_id(avatar_id)
        .filter(avatar::Column::UserId.eq(claims.sub))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let active: avatar::ActiveModel = avatar.into();
    active
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
