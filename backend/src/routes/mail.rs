use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    models::{server, Server, User},
    services::{Claims, MailKind, MailPayload},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ContactMailRequest {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SupportMailRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub category: String,
    pub subject: String,
    pub message: String,
    pub server_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MailStatusResponse {
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct MailSentResponse {
    pub sent: bool,
}

pub async fn mail_status(State(state): State<Arc<AppState>>) -> Json<MailStatusResponse> {
    Json(MailStatusResponse {
        enabled: state.mail_service.is_enabled(),
    })
}

pub async fn send_contact_mail(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ContactMailRequest>,
) -> Result<Json<MailSentResponse>, StatusCode> {
    validate_common(&payload.name, &payload.email, &payload.subject, &payload.message)?;

    state
        .mail_service
        .send(MailPayload {
            sender_name: payload.name,
            sender_email: payload.email,
            subject: payload.subject,
            message: payload.message,
            kind: MailKind::Contact,
            metadata: vec![],
            cc: None,
        })
        .await
        .map_err(|error| {
            tracing::error!(?error, "failed to send contact email");
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    Ok(Json(MailSentResponse { sent: true }))
}

pub async fn send_support_mail(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SupportMailRequest>,
) -> Result<Json<MailSentResponse>, StatusCode> {
    let name = payload.name.unwrap_or_else(|| claims.username.clone());
    let email = payload.email.unwrap_or_else(|| format!("{}@local.voxicraft", claims.sub));
    validate_common(&name, &email, &payload.subject, &payload.message)?;

    let category = payload.category;
    let server_url = payload.server_url.map(|value| value.trim().to_string()).filter(|value| !value.is_empty());
    if category == "server" && server_url.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut metadata = vec![
        ("Catégorie".to_string(), category.clone()),
        ("Utilisateur".to_string(), claims.username),
        ("ID utilisateur".to_string(), claims.sub),
    ];

    let mut cc = None;
    if let Some(server_url) = server_url {
        metadata.push(("URL du serveur".to_string(), server_url.clone()));
        if category == "bug" {
            cc = find_server_owner_email(&state, &server_url).await?;
            if cc.is_some() {
                metadata.push(("Copie propriétaire serveur".to_string(), "Oui".to_string()));
            }
        }
    }

    state
        .mail_service
        .send(MailPayload {
            sender_name: name,
            sender_email: email,
            subject: payload.subject,
            message: payload.message,
            kind: MailKind::Support,
            metadata,
            cc,
        })
        .await
        .map_err(|error| {
            tracing::error!(?error, "failed to send support email");
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    Ok(Json(MailSentResponse { sent: true }))
}

async fn find_server_owner_email(state: &Arc<AppState>, server_url: &str) -> Result<Option<String>, StatusCode> {
    let normalized_server_url = normalize_server_url(server_url);
    let Some(server) = Server::find()
        .filter(server::Column::GameDomain.eq(normalized_server_url))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Ok(None);
    };

    let owner = User::find_by_id(server.owner_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(owner.map(|owner| owner.email))
}

fn normalize_server_url(server_url: &str) -> String {
    server_url.trim().trim_end_matches('/').to_string()
}

fn validate_common(name: &str, email: &str, subject: &str, message: &str) -> Result<(), StatusCode> {
    if name.trim().len() < 2 || name.len() > 120 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !email.contains('@') || email.len() > 180 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if subject.trim().len() < 3 || subject.len() > 180 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if message.trim().len() < 10 || message.len() > 5000 {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}
