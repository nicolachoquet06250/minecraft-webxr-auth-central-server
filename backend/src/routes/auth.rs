use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Json, Redirect},
};
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{AuthResponse, DiscordCallbackQuery, DiscordOAuthUrl, LoginRequest, RegisterRequest, UserResponse},
    models::{user, User},
    routes::{join_ticket, login_origin},
    services::{hash_password, verify_password},
    AppState,
};

const SERVER_ORIGIN_HEADER: &str = "x-voxicraft-server-origin";

pub async fn register(State(state): State<Arc<AppState>>, Json(payload): Json<RegisterRequest>) -> Result<Json<AuthResponse>, StatusCode> {
    if let Err(e) = payload.validate() { tracing::error!("Validation error: {:?}", e); return Err(StatusCode::BAD_REQUEST); }
    let existing_user = User::find().filter(user::Column::Email.eq(&payload.email)).one(&state.db).await.map_err(|e| { tracing::error!("Database error checking existing user: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR })?;
    if existing_user.is_some() { tracing::warn!("User already exists: {}", payload.email); return Err(StatusCode::CONFLICT); }
    let password_hash = hash_password(&payload.password).map_err(|e| { tracing::error!("Password hashing error: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR })?;
    let birthdate = NaiveDate::parse_from_str(&payload.birthdate, "%Y-%m-%d").map_err(|e| { tracing::error!("Birthdate parsing error: {:?}", e); StatusCode::BAD_REQUEST })?;
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();
    let new_user = user::ActiveModel { id: Set(user_id), username: Set(payload.username.clone()), email: Set(payload.email.clone()), password_hash: Set(Some(password_hash)), avatar: Set(payload.avatar.clone()), bio: Set(payload.bio.clone()), birthdate: Set(birthdate), age_verified: Set(false), discord_id: Set(None), discord_username: Set(None), created_at: Set(now), updated_at: Set(now) };
    let user = new_user.insert(&state.db).await.map_err(|e| { tracing::error!("Database error inserting user: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR })?;
    tracing::info!("User registered successfully: {}", user.username);
    if let Err(error) = state.mail_service.send_welcome_email(&user.email, &user.username).await { tracing::warn!(?error, "welcome email could not be sent"); }
    let token = state.jwt_service.generate_token(&user.id, &user.username).map_err(|e| { tracing::error!("JWT generation error: {:?}", e); StatusCode::INTERNAL_SERVER_ERROR })?;
    Ok(Json(AuthResponse { token, user: user_to_response(user) }))
}

pub async fn login(State(state): State<Arc<AppState>>, headers: HeaderMap, Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, StatusCode> {
    if let Some(ticket) = payload.central_join_ticket.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        let game_domain = headers
            .get(SERVER_ORIGIN_HEADER)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);
        let ticket_user = join_ticket::consume_join_ticket(ticket.to_string(), None, game_domain).await?;
        let user = User::find_by_id(ticket_user.id)
            .one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;
        let token = state.jwt_service.generate_token(&user.id, &user.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(AuthResponse { token, user: user_to_response(user) }));
    }

    if !login_origin::is_allowed(&state, &headers).await? { return Err(StatusCode::FORBIDDEN); }
    let email = payload.email.as_deref().map(str::trim).filter(|value| !value.is_empty()).ok_or(StatusCode::BAD_REQUEST)?;
    let password = payload.password.as_deref().ok_or(StatusCode::BAD_REQUEST)?;
    let user = User::find().filter(user::Column::Email.eq(email)).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let password_hash = user.password_hash.as_ref().ok_or(StatusCode::UNAUTHORIZED)?;
    let valid = verify_password(password, password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !valid { return Err(StatusCode::UNAUTHORIZED); }
    let token = state.jwt_service.generate_token(&user.id, &user.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(AuthResponse { token, user: user_to_response(user) }))
}

pub async fn discord_oauth_url(State(state): State<Arc<AppState>>) -> Result<Json<DiscordOAuthUrl>, StatusCode> {
    let url = state.discord_service.get_oauth_url(Some("voxicraft_auth"));
    Ok(Json(DiscordOAuthUrl { url }))
}

pub async fn discord_callback(State(state): State<Arc<AppState>>, Query(params): Query<DiscordCallbackQuery>) -> Result<Redirect, StatusCode> {
    let token_response = state.discord_service.exchange_code(&params.code).await.map_err(|error| { tracing::warn!(?error, "discord token exchange failed"); StatusCode::BAD_REQUEST })?;
    let discord_user = state.discord_service.get_user(&token_response.access_token).await.map_err(|error| { tracing::warn!(?error, "discord user fetch failed"); StatusCode::BAD_REQUEST })?;
    let existing_user = User::find().filter(user::Column::DiscordId.eq(&discord_user.id)).one(&state.db).await.map_err(|error| { tracing::error!(?error, "database error checking discord user"); StatusCode::INTERNAL_SERVER_ERROR })?;
    let user = if let Some(user) = existing_user {
        user
    } else if let Some(discord_email) = discord_user.email.as_deref().filter(|email| !email.trim().is_empty()) {
        let existing_email_user = User::find().filter(user::Column::Email.eq(discord_email)).one(&state.db).await.map_err(|error| { tracing::error!(?error, "database error checking discord email user"); StatusCode::INTERNAL_SERVER_ERROR })?;
        if let Some(existing_email_user) = existing_email_user {
            let mut active_user: user::ActiveModel = existing_email_user.into();
            active_user.discord_id = Set(Some(discord_user.id.clone()));
            active_user.discord_username = Set(Some(discord_user.username.clone()));
            active_user.updated_at = Set(chrono::Utc::now().naive_utc());
            active_user.update(&state.db).await.map_err(|error| { tracing::error!(?error, "database error linking discord to existing email user"); StatusCode::INTERNAL_SERVER_ERROR })?
        } else { create_discord_user(&state, &discord_user).await? }
    } else { create_discord_user(&state, &discord_user).await? };
    let token = state.jwt_service.generate_token(&user.id, &user.username).map_err(|error| { tracing::error!(?error, "JWT generation error after discord callback"); StatusCode::INTERNAL_SERVER_ERROR })?;
    Ok(Redirect::to(&format!("/profile?auth_token={}", urlencoding::encode(&token))))
}

async fn create_discord_user(state: &Arc<AppState>, discord_user: &crate::services::DiscordUser) -> Result<user::Model, StatusCode> {
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();
    let default_birthdate = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let email = discord_user.email.clone().filter(|email| !email.trim().is_empty()).unwrap_or_else(|| format!("{}@discord.local", discord_user.id));
    let new_user = user::ActiveModel { id: Set(user_id), username: Set(discord_user.username.clone()), email: Set(email), password_hash: Set(None), avatar: Set("steve".to_string()), bio: Set(None), birthdate: Set(default_birthdate), age_verified: Set(discord_user.verified.unwrap_or(false)), discord_id: Set(Some(discord_user.id.clone())), discord_username: Set(Some(discord_user.username.clone())), created_at: Set(now), updated_at: Set(now) };
    new_user.insert(&state.db).await.map_err(|error| { tracing::error!(?error, "database error inserting discord user"); StatusCode::INTERNAL_SERVER_ERROR })
}

fn user_to_response(user: user::Model) -> UserResponse {
    UserResponse { id: user.id.to_string(), username: user.username, email: user.email, avatar: user.avatar, bio: user.bio, birthdate: user.birthdate.to_string(), age_verified: user.age_verified, discord_username: user.discord_username, created_at: user.created_at.to_string() }
}
