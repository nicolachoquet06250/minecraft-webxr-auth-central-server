use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
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
    pub server_id: Option<String>,
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

    let mut metadata = vec![
        ("Catégorie".to_string(), payload.category),
        ("Utilisateur".to_string(), claims.username),
        ("ID utilisateur".to_string(), claims.sub),
    ];
    if let Some(server_id) = payload.server_id {
        metadata.push(("ID serveur".to_string(), server_id));
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
        })
        .await
        .map_err(|error| {
            tracing::error!(?error, "failed to send support email");
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    Ok(Json(MailSentResponse { sent: true }))
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
