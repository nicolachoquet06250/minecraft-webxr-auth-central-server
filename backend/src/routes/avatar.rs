use axum::{extract::{Extension, Path, State}, http::{header, HeaderValue, StatusCode}, response::{IntoResponse, Json, Response}};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;
use validator::Validate;

use crate::{dto::{ActiveAvatarResponse, AvatarResponse, SaveAvatarRequest, UpdateAvatarRequest}, models::{avatar, Avatar, User}, services::Claims, AppState};

pub async fn profile_pic_preflight() -> StatusCode { StatusCode::NO_CONTENT }

pub async fn list_avatars(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>) -> Result<Json<Vec<AvatarResponse>>, StatusCode> {
    let avatars = Avatar::find().filter(avatar::Column::UserId.eq(claims.sub)).all(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(avatars.into_iter().map(to_response).collect()))
}

pub async fn get_active_avatar(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>) -> Result<Json<ActiveAvatarResponse>, StatusCode> {
    let avatar = Avatar::find().filter(avatar::Column::UserId.eq(claims.sub)).filter(avatar::Column::IsActive.eq(true)).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ActiveAvatarResponse { kind: if avatar.is_some() { "custom" } else { "default" }.to_string(), avatar: avatar.map(to_response) }))
}

pub async fn get_profile_pic_svg(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>) -> Result<Response, StatusCode> {
    profile_pic_response(&state, &claims.sub).await
}

pub async fn get_user_profile_pic_svg(Extension(_claims): Extension<Claims>, State(state): State<Arc<AppState>>, Path(user_id): Path<String>) -> Result<Response, StatusCode> {
    profile_pic_response(&state, &user_id).await
}

pub async fn create_avatar_copy(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>, Json(payload): Json<SaveAvatarRequest>) -> Result<Json<AvatarResponse>, StatusCode> {
    payload.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
    let now = chrono::Utc::now().naive_utc();
    let id = Uuid::new_v4().to_string();
    let texture_data = serde_json::to_string(&payload.texture_data).map_err(|_| StatusCode::BAD_REQUEST)?;
    let user_id = claims.sub.clone();
    let created = avatar::ActiveModel { id: Set(id), user_id: Set(user_id.clone()), name: Set(payload.name), base_kind: Set(normalize_base_kind(&payload.base_kind)), is_active: Set(false), texture_data: Set(texture_data), created_at: Set(now), updated_at: Set(now) }.insert(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    notify_avatar_created(&state, &user_id, &created).await;
    Ok(Json(to_response(created)))
}

pub async fn update_avatar(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>, Path(avatar_id): Path<String>, Json(payload): Json<UpdateAvatarRequest>) -> Result<Json<AvatarResponse>, StatusCode> {
    payload.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
    if avatar_id == "steve" || avatar_id == "alex" { return Err(StatusCode::FORBIDDEN); }
    let user_id = claims.sub.clone();
    let avatar = Avatar::find_by_id(avatar_id).filter(avatar::Column::UserId.eq(user_id.clone())).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let mut active: avatar::ActiveModel = avatar.into();
    if let Some(name) = payload.name { active.name = Set(name); }
    active.texture_data = Set(serde_json::to_string(&payload.texture_data).map_err(|_| StatusCode::BAD_REQUEST)?);
    active.updated_at = Set(chrono::Utc::now().naive_utc());
    let updated = active.update(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    notify_avatar_updated(&state, &user_id, &updated).await;
    Ok(Json(to_response(updated)))
}

async fn notify_avatar_created(state: &Arc<AppState>, user_id: &str, saved_avatar: &avatar::Model) {
    let Ok(Some(current_user)) = User::find_by_id(user_id.to_string()).one(&state.db).await else { tracing::warn!(user_id, "avatar created mail skipped: user not found"); return; };
    let preview_image_data_url = avatar_mail_preview_image_data_url(&saved_avatar.texture_data);
    if let Err(error) = state.mail_service.send_avatar_created_email(&current_user.email, &current_user.username, &saved_avatar.name, &preview_image_data_url).await { tracing::warn!(?error, avatar_id = %saved_avatar.id, "avatar created mail could not be sent"); }
}

async fn notify_avatar_updated(state: &Arc<AppState>, user_id: &str, saved_avatar: &avatar::Model) {
    let Ok(Some(current_user)) = User::find_by_id(user_id.to_string()).one(&state.db).await else { tracing::warn!(user_id, "avatar updated mail skipped: user not found"); return; };
    let preview_image_data_url = avatar_mail_preview_image_data_url(&saved_avatar.texture_data);
    if let Err(error) = state.mail_service.send_avatar_updated_email(&current_user.email, &current_user.username, &saved_avatar.name, &preview_image_data_url).await { tracing::warn!(?error, avatar_id = %saved_avatar.id, "avatar updated mail could not be sent"); }
}

pub async fn select_avatar(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>, Path(avatar_id): Path<String>) -> Result<StatusCode, StatusCode> {
    let owned = Avatar::find_by_id(avatar_id.clone()).filter(avatar::Column::UserId.eq(claims.sub.clone())).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    let avatars = Avatar::find().filter(avatar::Column::UserId.eq(claims.sub)).all(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    for item in avatars {
        let should_activate = item.id == owned.id;
        let mut active: avatar::ActiveModel = item.into();
        active.is_active = Set(should_activate);
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active.update(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn clear_active_avatar(Extension(claims): Extension<Claims>, State(state): State<Arc<AppState>>) -> Result<StatusCode, StatusCode> {
    let avatars = Avatar::find().filter(avatar::Column::UserId.eq(claims.sub)).all(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    for item in avatars {
        let mut active: avatar::ActiveModel = item.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active.update(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn profile_pic_response(state: &Arc<AppState>, user_id: &str) -> Result<Response, StatusCode> {
    let active_avatar = Avatar::find().filter(avatar::Column::UserId.eq(user_id)).filter(avatar::Column::IsActive.eq(true)).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let svg = if let Some(active_avatar) = active_avatar { svg_from_texture_data(&active_avatar.texture_data)? } else {
        let current_user = User::find_by_id(user_id.to_string()).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
        default_profile_pic_svg(&current_user.avatar)
    };
    Ok(([(header::CONTENT_TYPE, HeaderValue::from_static("image/svg+xml; charset=utf-8")), (header::CACHE_CONTROL, HeaderValue::from_static("no-store"))], svg).into_response())
}

fn avatar_mail_preview_image_data_url(texture_data: &str) -> String {
    let svg = svg_from_texture_data(texture_data).unwrap_or_else(|_| empty_svg());
    format!("data:image/svg+xml;base64,{}", BASE64_STANDARD.encode(svg.as_bytes()))
}

fn normalize_base_kind(value: &str) -> String {
    match value { "steve" => "steve".to_string(), "alex" => "alex".to_string(), _ => "custom".to_string() }
}

fn to_response(model: avatar::Model) -> AvatarResponse {
    AvatarResponse { id: model.id, name: model.name, base_kind: model.base_kind, is_active: model.is_active, texture_data: serde_json::from_str::<Value>(&model.texture_data).unwrap_or(Value::Null), created_at: model.created_at.to_string(), updated_at: model.updated_at.to_string() }
}

fn svg_from_texture_data(texture_data: &str) -> Result<String, StatusCode> {
    let data = serde_json::from_str::<Value>(texture_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let palette = data.get("palette").and_then(Value::as_object).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let face = data.get("parts").and_then(|parts| parts.get("head")).and_then(|head| head.get("front")).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let width = face.get("width").and_then(Value::as_u64).ok_or(StatusCode::INTERNAL_SERVER_ERROR)? as usize;
    let height = face.get("height").and_then(Value::as_u64).ok_or(StatusCode::INTERNAL_SERVER_ERROR)? as usize;
    let matrix = face.get("matrix").and_then(Value::as_array).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut colors = HashMap::new();
    for (key, value) in palette {
        let channel_values = value.as_array().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        if channel_values.len() != 4 { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
        let mut channels = [0.0_f64; 4];
        for (index, channel) in channel_values.iter().enumerate() { channels[index] = channel.as_f64().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?; }
        colors.insert(key.clone(), channels);
    }
    let rows = matrix.iter().map(|row| row.as_str().ok_or(StatusCode::INTERNAL_SERVER_ERROR).map(str::to_string)).collect::<Result<Vec<_>, _>>()?;
    build_svg(width, height, &rows, |key| colors.get(&key.to_string()).copied())
}

fn default_profile_pic_svg(avatar: &str) -> String {
    match avatar { "steve" => build_svg(8, 8, &steve_head_matrix(), steve_head_color).unwrap_or_else(|_| empty_svg()), _ => build_svg(8, 8, &alex_head_matrix(), alex_head_color).unwrap_or_else(|_| empty_svg()) }
}

fn build_svg<F>(width: usize, height: usize, matrix: &[String], color_for: F) -> Result<String, StatusCode>
where F: Fn(char) -> Option<[f64; 4]> {
    let cell = 12;
    let svg_width = width * cell;
    let svg_height = height * cell;
    let mut rects = String::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, key) in row.chars().enumerate() {
            let Some([r, g, b, a]) = color_for(key) else { continue };
            if a <= 0.0 { continue; }
            let r = (r * 255.0).round().clamp(0.0, 255.0) as u8;
            let g = (g * 255.0).round().clamp(0.0, 255.0) as u8;
            let b = (b * 255.0).round().clamp(0.0, 255.0) as u8;
            rects.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgb({},{},{})" fill-opacity="{}"/>"#, x * cell, y * cell, cell, cell, r, g, b, a.clamp(0.0, 1.0)));
        }
    }
    Ok(format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}" shape-rendering="crispEdges">{}</svg>"#, svg_width, svg_height, svg_width, svg_height, rects))
}

fn empty_svg() -> String { r#"<svg xmlns="http://www.w3.org/2000/svg" width="96" height="96" viewBox="0 0 96 96" shape-rendering="crispEdges"/>"#.to_string() }
fn steve_head_matrix() -> Vec<String> { vec!["FFFFFFFF".to_string(), "FAAKKKAF".to_string(), "ABBJBBGK".to_string(), "NBNNGBII".to_string(), "JCENJEMJ".to_string(), "IBNLLNGD".to_string(), "IIPHHKDD".to_string(), "HHOPPODD".to_string()] }
fn alex_head_matrix() -> Vec<String> { vec!["BBBBBBBB".to_string(), "BCCDDCBB".to_string(), "BCCHHCCB".to_string(), "BGGHHGGB".to_string(), "GJKHHKJG".to_string(), "GGGHGGGG".to_string(), "GGGLLGGG".to_string(), "GGGGGGGG".to_string()] }

fn steve_head_color(key: char) -> Option<[f64; 4]> {
    match key { 'A' => Some([0.24, 0.17, 0.09, 1.0]), 'B' => Some([0.67, 0.51, 0.42, 1.0]), 'C' => Some([0.96, 0.96, 0.97, 1.0]), 'D' => Some([0.48, 0.33, 0.22, 1.0]), 'E' => Some([0.24, 0.12, 0.59, 1.0]), 'F' => Some([0.16, 0.11, 0.04, 1.0]), 'G' => Some([0.59, 0.44, 0.33, 1.0]), 'H' => Some([0.44, 0.28, 0.20, 1.0]), 'I' => Some([0.56, 0.39, 0.28, 1.0]), 'J' => Some([0.69, 0.55, 0.47, 1.0]), 'K' => Some([0.28, 0.19, 0.11, 1.0]), 'L' => Some([0.38, 0.24, 0.18, 1.0]), 'M' => Some([0.91, 0.89, 0.94, 1.0]), 'N' => Some([0.63, 0.47, 0.41, 1.0]), 'O' => Some([0.22, 0.12, 0.03, 1.0]), 'P' => Some([0.26, 0.15, 0.07, 1.0]), _ => None }
}

fn alex_head_color(key: char) -> Option<[f64; 4]> {
    match key { 'A' => Some([0.74, 0.33, 0.05, 1.0]), 'B' => Some([0.86, 0.43, 0.08, 1.0]), 'C' => Some([0.95, 0.55, 0.12, 1.0]), 'D' => Some([1.0, 0.66, 0.20, 1.0]), 'E' => Some([0.82, 0.55, 0.34, 1.0]), 'F' => Some([0.93, 0.70, 0.47, 1.0]), 'G' => Some([1.0, 0.80, 0.55, 1.0]), 'H' => Some([1.0, 0.88, 0.68, 1.0]), 'I' => Some([0.72, 0.43, 0.26, 1.0]), 'J' => Some([0.96, 0.94, 0.88, 1.0]), 'K' => Some([0.10, 0.35, 0.16, 1.0]), 'L' => Some([0.74, 0.42, 0.34, 1.0]), _ => None }
}
