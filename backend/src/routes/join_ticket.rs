use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use chrono::{Duration, NaiveDateTime, Utc};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::Arc, sync::OnceLock};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    models::{Server, User},
    services::Claims,
    AppState,
};

const JOIN_TICKET_TTL_SECONDS: i64 = 60;
const SERVER_SECRET_HEADER: &str = "x-voxicraft-server-secret";

static JOIN_TICKETS: OnceLock<RwLock<HashMap<String, JoinTicket>>> = OnceLock::new();

#[derive(Clone)]
struct JoinTicket {
    user_id: String,
    username: String,
    email: String,
    server_id: String,
    game_domain: String,
    expires_at: NaiveDateTime,
    used: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateJoinTicketResponse {
    pub ticket: String,
    pub join_url: String,
    pub expires_in_seconds: i64,
}

#[derive(Debug, Deserialize)]
pub struct VerifyJoinTicketRequest {
    pub ticket: String,
    pub server_id: Option<String>,
    pub game_domain: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VerifyJoinTicketResponse {
    pub user: VerifyJoinTicketUser,
}

#[derive(Debug, Serialize)]
pub struct VerifyJoinTicketUser {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub async fn create_join_ticket(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<String>,
) -> Result<Json<CreateJoinTicketResponse>, StatusCode> {
    let server = Server::find_by_id(server_id.clone())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !server.is_active {
        return Err(StatusCode::CONFLICT);
    }

    let user = User::find_by_id(claims.sub.clone())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let ticket = Uuid::new_v4().to_string();
    let expires_at = Utc::now().naive_utc() + Duration::seconds(JOIN_TICKET_TTL_SECONDS);
    let game_domain = normalize_origin(&server.game_domain);

    tickets().write().await.insert(
        ticket.clone(),
        JoinTicket {
            user_id: user.id,
            username: user.username,
            email: user.email,
            server_id: server.id,
            game_domain: game_domain.clone(),
            expires_at,
            used: false,
        },
    );

    Ok(Json(CreateJoinTicketResponse {
        join_url: format!("{}#central_join_ticket={}", game_domain, urlencoding::encode(&ticket)),
        ticket,
        expires_in_seconds: JOIN_TICKET_TTL_SECONDS,
    }))
}

pub async fn verify_join_ticket(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<VerifyJoinTicketRequest>,
) -> Result<Json<VerifyJoinTicketResponse>, StatusCode> {
    verify_server_secret(&headers)?;

    let ticket_id = payload.ticket.trim().to_string();
    if ticket_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tickets = tickets().write().await;
    let ticket = tickets.get_mut(&ticket_id).ok_or(StatusCode::NOT_FOUND)?;

    if ticket.used || ticket.expires_at < Utc::now().naive_utc() {
        tickets.remove(&ticket_id);
        return Err(StatusCode::GONE);
    }

    if let Some(expected_server_id) = payload.server_id.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        if expected_server_id != ticket.server_id {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    if let Some(expected_domain) = payload.game_domain.as_deref().map(normalize_origin).filter(|value| !value.is_empty()) {
        if expected_domain != ticket.game_domain {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    ticket.used = true;
    let user = VerifyJoinTicketUser {
        id: ticket.user_id.clone(),
        username: ticket.username.clone(),
        email: ticket.email.clone(),
    };
    tickets.remove(&ticket_id);

    Ok(Json(VerifyJoinTicketResponse { user }))
}

fn verify_server_secret(headers: &HeaderMap) -> Result<(), StatusCode> {
    let expected_secret = env::var("CENTRAL_JOIN_TICKET_SECRET")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let provided_secret = headers
        .get(SERVER_SECRET_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if provided_secret != expected_secret {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(())
}

fn tickets() -> &'static RwLock<HashMap<String, JoinTicket>> {
    JOIN_TICKETS.get_or_init(|| RwLock::new(HashMap::new()))
}

fn normalize_origin(value: &str) -> String {
    value.trim().trim_end_matches('/').to_string()
}
