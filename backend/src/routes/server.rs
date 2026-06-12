use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{CreateServerRequest, ServerResponse, UpdateServerRequest},
    models::{server, Server},
    services::Claims,
    AppState,
};

pub async fn create_server(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateServerRequest>,
) -> Result<Json<ServerResponse>, StatusCode> {
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let owner_id = claims.sub.clone();

    // Check game server health before registering it.
    check_game_server_health(&payload.game_domain).await?;

    // Check if game_domain is already registered.
    let existing_game = Server::find()
        .filter(server::Column::GameDomain.eq(&payload.game_domain))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_game.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let server_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();

    let new_server = server::ActiveModel {
        id: Set(server_id),
        owner_id: Set(owner_id),
        name: Set(payload.name.clone()),
        game_domain: Set(payload.game_domain.clone()),
        description: Set(payload.description.clone()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let server = new_server
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(server_to_response(server)))
}

async fn check_game_server_health(game_domain: &str) -> Result<(), StatusCode> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| {
            tracing::error!("Failed to build HTTP client: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let health_urls = build_game_server_health_urls(game_domain);
    let mut last_error: Option<String> = None;

    for health_url in health_urls {
        tracing::info!("Checking game server health at: {}", health_url);

        match client.get(&health_url).send().await {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    tracing::info!("Game server health check passed at: {}", health_url);
                    return Ok(());
                }

                let message = format!(
                    "Game server health check failed at {} with status: {}",
                    health_url, status
                );
                tracing::warn!("{}", message);
                last_error = Some(message);
            }
            Err(e) => {
                let message = format!(
                    "Failed to connect to game server health endpoint {}: {}",
                    health_url, e
                );
                tracing::warn!("{}", message);
                last_error = Some(message);
            }
        }
    }

    tracing::error!(
        "Game server health check failed for all known endpoints: {}",
        last_error.unwrap_or_else(|| "no health endpoint generated".to_string())
    );

    Err(StatusCode::SERVICE_UNAVAILABLE)
}

fn build_game_server_health_urls(game_domain: &str) -> Vec<String> {
    let game_domain = game_domain.trim_end_matches('/');
    let mut urls = Vec::new();

    push_unique(&mut urls, format!("{}/healthz", game_domain));

    urls
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

pub async fn get_user_servers(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ServerResponse>>, StatusCode> {
    let owner_id = claims.sub.clone();

    let servers = Server::find()
        .filter(server::Column::OwnerId.eq(&owner_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = servers.into_iter().map(server_to_response).collect();

    Ok(Json(response))
}

pub async fn get_server(
    Path(server_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ServerResponse>, StatusCode> {
    let server = Server::find_by_id(server_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(server_to_response(server)))
}

pub async fn update_server(
    Extension(claims): Extension<Claims>,
    Path(server_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateServerRequest>,
) -> Result<Json<ServerResponse>, StatusCode> {
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let owner_id = claims.sub.clone();

    let server = Server::find_by_id(server_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if server.owner_id != owner_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut server_active: server::ActiveModel = server.into();

    if let Some(name) = payload.name {
        server_active.name = Set(name);
    }
    if let Some(description) = payload.description {
        server_active.description = Set(Some(description));
    }
    if let Some(is_active) = payload.is_active {
        server_active.is_active = Set(is_active);
    }

    server_active.updated_at = Set(chrono::Utc::now().naive_utc());

    let updated_server = server_active
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(server_to_response(updated_server)))
}

pub async fn delete_server(
    Extension(claims): Extension<Claims>,
    Path(server_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, StatusCode> {
    let owner_id = claims.sub.clone();

    let server = Server::find_by_id(server_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if server.owner_id != owner_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let server_active: server::ActiveModel = server.into();
    server_active
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

fn server_to_response(server: server::Model) -> ServerResponse {
    ServerResponse {
        id: server.id,
        owner_id: server.owner_id,
        name: server.name,
        game_domain: server.game_domain,
        description: server.description,
        is_active: server.is_active,
        created_at: server.created_at.to_string(),
        updated_at: server.updated_at.to_string(),
    }
}
