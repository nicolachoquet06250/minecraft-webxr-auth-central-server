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
    if let Err(_) = payload.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let owner_id = claims.sub.clone();

    // Check relay server health
    let health_url = format!("{}/health", payload.relay_domain.trim_end_matches('/'));
    
    tracing::info!("Checking relay server health at: {}", health_url);
    
    let health_check = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| {
            tracing::error!("Failed to build HTTP client: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .get(&health_url)
        .send()
        .await;

    match health_check {
        Ok(response) => {
            if !response.status().is_success() {
                tracing::error!(
                    "Relay server health check failed with status: {}",
                    response.status()
                );
                return Err(StatusCode::SERVICE_UNAVAILABLE);
            }
            tracing::info!("Relay server health check passed");
        }
        Err(e) => {
            tracing::error!("Failed to connect to relay server: {}", e);
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    }

    // Check if relay_domain or game_domain are already registered
    let existing_relay = Server::find()
        .filter(server::Column::RelayDomain.eq(&payload.relay_domain))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_relay.is_some() {
        return Err(StatusCode::CONFLICT);
    }

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
        relay_domain: Set(payload.relay_domain.clone()),
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

    Ok(Json(ServerResponse {
        id: server.id.to_string(),
        owner_id: server.owner_id.to_string(),
        name: server.name,
        relay_domain: server.relay_domain,
        game_domain: server.game_domain,
        description: server.description,
        is_active: server.is_active,
        created_at: server.created_at.to_string(),
        updated_at: server.updated_at.to_string(),
    }))
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

    let response = servers
        .into_iter()
        .map(|s| ServerResponse {
            id: s.id.to_string(),
            owner_id: s.owner_id.to_string(),
            name: s.name,
            relay_domain: s.relay_domain,
            game_domain: s.game_domain,
            description: s.description,
            is_active: s.is_active,
            created_at: s.created_at.to_string(),
            updated_at: s.updated_at.to_string(),
        })
        .collect();

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

    Ok(Json(ServerResponse {
        id: server.id.to_string(),
        owner_id: server.owner_id.to_string(),
        name: server.name,
        relay_domain: server.relay_domain,
        game_domain: server.game_domain,
        description: server.description,
        is_active: server.is_active,
        created_at: server.created_at.to_string(),
        updated_at: server.updated_at.to_string(),
    }))
}

pub async fn update_server(
    Extension(claims): Extension<Claims>,
    Path(server_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateServerRequest>,
) -> Result<Json<ServerResponse>, StatusCode> {
    if let Err(_) = payload.validate() {
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

    Ok(Json(ServerResponse {
        id: updated_server.id.to_string(),
        owner_id: updated_server.owner_id.to_string(),
        name: updated_server.name,
        relay_domain: updated_server.relay_domain,
        game_domain: updated_server.game_domain,
        description: updated_server.description,
        is_active: updated_server.is_active,
        created_at: updated_server.created_at.to_string(),
        updated_at: updated_server.updated_at.to_string(),
    }))
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
