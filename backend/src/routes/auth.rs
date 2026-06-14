use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{
        AuthResponse, DiscordCallbackQuery, DiscordOAuthUrl, LoginRequest, RegisterRequest,
        UserResponse,
    },
    models::{user, User},
    services::{hash_password, verify_password},
    AppState,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    if let Err(e) = payload.validate() {
        tracing::error!("Validation error: {:?}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    let existing_user = User::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Database error checking existing user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if existing_user.is_some() {
        tracing::warn!("User already exists: {}", payload.email);
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let password_hash = hash_password(&payload.password)
        .map_err(|e| {
            tracing::error!("Password hashing error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Parse birthdate
    let birthdate = NaiveDate::parse_from_str(&payload.birthdate, "%Y-%m-%d")
        .map_err(|e| {
            tracing::error!("Birthdate parsing error: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    // Create user
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();

    let new_user = user::ActiveModel {
        id: Set(user_id),
        username: Set(payload.username.clone()),
        email: Set(payload.email.clone()),
        password_hash: Set(Some(password_hash)),
        avatar: Set(payload.avatar.clone()),
        bio: Set(payload.bio.clone()),
        birthdate: Set(birthdate),
        age_verified: Set(false),
        discord_id: Set(None),
        discord_username: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let user = new_user
        .insert(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Database error inserting user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!("User registered successfully: {}", user.username);

    if let Err(error) = state.mail_service.send_welcome_email(&user.email, &user.username).await {
        tracing::warn!(?error, "welcome email could not be sent");
    }

    // Generate JWT
    let token = state
        .jwt_service
        .generate_token(&user.id, &user.username)
        .map_err(|e| {
            tracing::error!("JWT generation error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            avatar: user.avatar,
            bio: user.bio,
            birthdate: user.birthdate.to_string(),
            age_verified: user.age_verified,
            discord_username: user.discord_username,
            created_at: user.created_at.to_string(),
        },
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    if let Err(_) = payload.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Find user
    let user = User::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let password_hash = user.password_hash.as_ref().ok_or(StatusCode::UNAUTHORIZED)?;
    
    let valid = verify_password(&payload.password, password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT
    let token = state
        .jwt_service
        .generate_token(&user.id, &user.username)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            avatar: user.avatar,
            bio: user.bio,
            birthdate: user.birthdate.to_string(),
            age_verified: user.age_verified,
            discord_username: user.discord_username,
            created_at: user.created_at.to_string(),
        },
    }))
}

pub async fn discord_oauth_url(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DiscordOAuthUrl>, StatusCode> {
    let url = state.discord_service.get_oauth_url(Some("voxicraft_auth"));
    Ok(Json(DiscordOAuthUrl { url }))
}

pub async fn discord_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DiscordCallbackQuery>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Exchange code for token
    let token_response = state
        .discord_service
        .exchange_code(&params.code)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Get Discord user info
    let discord_user = state
        .discord_service
        .get_user(&token_response.access_token)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Check if user exists with this Discord ID
    let existing_user = User::find()
        .filter(user::Column::DiscordId.eq(&discord_user.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = if let Some(user) = existing_user {
        user
    } else {
        // Create new user
        let user_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();
        let default_birthdate = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();

        let new_user = user::ActiveModel {
            id: Set(user_id),
            username: Set(discord_user.username.clone()),
            email: Set(discord_user.email.clone().unwrap_or_default()),
            password_hash: Set(None),
            avatar: Set("steve".to_string()),
            bio: Set(None),
            birthdate: Set(default_birthdate),
            age_verified: Set(discord_user.verified.unwrap_or(false)),
            discord_id: Set(Some(discord_user.id.clone())),
            discord_username: Set(Some(discord_user.username.clone())),
            created_at: Set(now),
            updated_at: Set(now),
        };

        new_user
            .insert(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    // Generate JWT
    let token = state
        .jwt_service
        .generate_token(&user.id, &user.username)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            avatar: user.avatar,
            bio: user.bio,
            birthdate: user.birthdate.to_string(),
            age_verified: user.age_verified,
            discord_username: user.discord_username,
            created_at: user.created_at.to_string(),
        },
    }))
}
