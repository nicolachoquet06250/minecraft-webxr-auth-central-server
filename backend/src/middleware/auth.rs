use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

use crate::{
    models::{server, server_visit, Server, ServerVisit},
    AppState,
};

const MAX_RECENT_SERVERS: u64 = 10;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if req.method() == Method::OPTIONS {
        return Ok(next.run(req).await);
    }

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    match state.jwt_service.verify_token(token) {
        Ok(claims) => {
            record_server_visit_from_origin(&state, &claims.sub, req.headers()).await;
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn record_server_visit_from_origin(state: &Arc<AppState>, user_id: &str, headers: &HeaderMap) {
    let Some(origin) = request_origin(headers) else {
        return;
    };

    let Ok(Some(server)) = Server::find()
        .filter(server::Column::GameDomain.eq(&origin))
        .one(&state.db)
        .await
    else {
        return;
    };

    let now = chrono::Utc::now().naive_utc();
    let existing_visit = ServerVisit::find()
        .filter(server_visit::Column::UserId.eq(user_id))
        .filter(server_visit::Column::ServerId.eq(&server.id))
        .one(&state.db)
        .await;

    match existing_visit {
        Ok(Some(visit)) => {
            let mut active: server_visit::ActiveModel = visit.into();
            active.visited_at = Set(now);
            if let Err(error) = active.update(&state.db).await {
                tracing::warn!(?error, user_id = %user_id, server_id = %server.id, "server visit update failed");
            }
        }
        Ok(None) => {
            let active = server_visit::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                user_id: Set(user_id.to_string()),
                server_id: Set(server.id.clone()),
                visited_at: Set(now),
            };
            if let Err(error) = active.insert(&state.db).await {
                tracing::warn!(?error, user_id = %user_id, server_id = %server.id, "server visit insert failed");
                return;
            }
        }
        Err(error) => {
            tracing::warn!(?error, user_id = %user_id, server_id = %server.id, "server visit lookup failed");
            return;
        }
    }

    prune_recent_servers(state, user_id).await;
}

fn request_origin(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())
        .and_then(normalize_origin)
}

fn normalize_origin(value: &str) -> Option<String> {
    let url = Url::parse(value).ok()?;
    let host = url.host_str()?;
    let port = url.port().map(|port| format!(":{}", port)).unwrap_or_default();
    Some(format!("{}://{}{}", url.scheme(), host, port))
}

async fn prune_recent_servers(state: &Arc<AppState>, user_id: &str) {
    let Ok(visits) = ServerVisit::find()
        .filter(server_visit::Column::UserId.eq(user_id))
        .order_by_desc(server_visit::Column::VisitedAt)
        .offset(MAX_RECENT_SERVERS)
        .all(&state.db)
        .await
    else {
        return;
    };

    for visit in visits {
        if let Err(error) = ServerVisit::delete_by_id(visit.id).exec(&state.db).await {
            tracing::warn!(?error, user_id = %user_id, "server visit prune failed");
        }
    }
}
