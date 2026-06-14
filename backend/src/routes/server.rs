use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{CreateServerRequest, FavoriteServerResponse, RecordServerVisitRequest, ServerHistoryResponse, ServerResponse, UpdateServerRequest},
    models::{server, server_favorite, server_visit, Server, ServerFavorite, ServerVisit},
    services::Claims,
    AppState,
};

const MAX_RECENT_SERVERS: u64 = 10;

pub async fn create_server(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateServerRequest>,
) -> Result<Json<ServerResponse>, StatusCode> {
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let owner_id = claims.sub.clone();
    let game_domain = normalize_server_url(&payload.game_domain);

    check_game_server_health(&game_domain).await?;

    let existing_game = Server::find()
        .filter(server::Column::GameDomain.eq(&game_domain))
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
        game_domain: Set(game_domain),
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

pub async fn get_recent_servers(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ServerHistoryResponse>>, StatusCode> {
    let user_id = claims.sub.clone();
    let visits = ServerVisit::find()
        .filter(server_visit::Column::UserId.eq(&user_id))
        .order_by_desc(server_visit::Column::VisitedAt)
        .limit(MAX_RECENT_SERVERS)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut response = Vec::new();
    for visit in visits {
        if let Some(server) = Server::find_by_id(visit.server_id.clone())
            .one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            let favorite = find_favorite(&state, &user_id, &server.id).await?;
            response.push(ServerHistoryResponse {
                server: server_to_response(server),
                is_favorite: favorite.is_some(),
                visited_at: Some(visit.visited_at.to_string()),
                favorited_at: favorite.map(|favorite| favorite.created_at.to_string()),
            });
        }
    }

    Ok(Json(response))
}

pub async fn get_favorite_servers(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FavoriteServerResponse>>, StatusCode> {
    let user_id = claims.sub.clone();
    let favorites = ServerFavorite::find()
        .filter(server_favorite::Column::UserId.eq(&user_id))
        .order_by_desc(server_favorite::Column::CreatedAt)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut response = Vec::new();
    for favorite in favorites {
        if let Some(server) = Server::find_by_id(favorite.server_id.clone())
            .one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            response.push(FavoriteServerResponse {
                server: server_to_response(server),
                is_favorite: true,
                favorited_at: favorite.created_at.to_string(),
            });
        }
    }

    Ok(Json(response))
}

pub async fn record_server_visit(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RecordServerVisitRequest>,
) -> Result<Json<ServerHistoryResponse>, StatusCode> {
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user_id = claims.sub.clone();
    let server_url = normalize_server_url(&payload.server_url);
    let server = Server::find()
        .filter(server::Column::GameDomain.eq(&server_url))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let now = chrono::Utc::now().naive_utc();
    if let Some(existing_visit) = ServerVisit::find()
        .filter(server_visit::Column::UserId.eq(&user_id))
        .filter(server_visit::Column::ServerId.eq(&server.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let mut active_visit: server_visit::ActiveModel = existing_visit.into();
        active_visit.server_url = Set(server_url.clone());
        active_visit.visited_at = Set(now);
        active_visit
            .update(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        server_visit::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            user_id: Set(user_id.clone()),
            server_id: Set(server.id.clone()),
            server_url: Set(server_url.clone()),
            visited_at: Set(now),
        }
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    prune_recent_servers(&state, &user_id).await?;
    let favorite = find_favorite(&state, &user_id, &server.id).await?;

    Ok(Json(ServerHistoryResponse {
        server: server_to_response(server),
        is_favorite: favorite.is_some(),
        visited_at: Some(now.to_string()),
        favorited_at: favorite.map(|favorite| favorite.created_at.to_string()),
    }))
}

pub async fn favorite_server(
    Extension(claims): Extension<Claims>,
    Path(server_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<FavoriteServerResponse>, StatusCode> {
    let user_id = claims.sub.clone();
    let server = Server::find_by_id(server_id.clone())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let favorite = if let Some(favorite) = find_favorite(&state, &user_id, &server_id).await? {
        favorite
    } else {
        server_favorite::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            user_id: Set(user_id),
            server_id: Set(server_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
        }
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(FavoriteServerResponse {
        server: server_to_response(server),
        is_favorite: true,
        favorited_at: favorite.created_at.to_string(),
    }))
}

pub async fn unfavorite_server(
    Extension(claims): Extension<Claims>,
    Path(server_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, StatusCode> {
    let user_id = claims.sub.clone();
    ServerFavorite::delete_many()
        .filter(server_favorite::Column::UserId.eq(user_id))
        .filter(server_favorite::Column::ServerId.eq(server_id))
        .exec(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
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

async fn find_favorite(
    state: &Arc<AppState>,
    user_id: &str,
    server_id: &str,
) -> Result<Option<server_favorite::Model>, StatusCode> {
    ServerFavorite::find()
        .filter(server_favorite::Column::UserId.eq(user_id))
        .filter(server_favorite::Column::ServerId.eq(server_id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn prune_recent_servers(state: &Arc<AppState>, user_id: &str) -> Result<(), StatusCode> {
    let visits_to_delete = ServerVisit::find()
        .filter(server_visit::Column::UserId.eq(user_id))
        .order_by_desc(server_visit::Column::VisitedAt)
        .offset(MAX_RECENT_SERVERS)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for visit in visits_to_delete {
        ServerVisit::delete_by_id(visit.id)
            .exec(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}

fn normalize_server_url(server_url: &str) -> String {
    server_url.trim().trim_end_matches('/').to_string()
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
