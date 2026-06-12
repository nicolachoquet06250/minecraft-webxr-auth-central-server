use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderValue, Method, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use url::Url;

use crate::{models::server, AppState};

/// Middleware pour gérer le CORS de manière dynamique en fonction des serveurs enregistrés.
///
/// Les origines configurées via `CORS_ORIGIN` sont toujours prioritaires et ne dépendent pas de la DB.
/// `CORS_ORIGIN` accepte une ou plusieurs origines séparées par des virgules.
pub async fn dynamic_cors_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let origin = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);

    let is_preflight = request.method() == Method::OPTIONS;
    let is_allowed = match origin.as_deref() {
        Some(origin) => is_origin_allowed(origin, &state).await,
        None => false,
    };

    // Pour les requêtes OPTIONS (preflight), répondre immédiatement.
    // Important : ne jamais laisser le fallback ou un handler applicatif répondre au preflight.
    if is_preflight {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = if is_allowed {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::FORBIDDEN
        };

        apply_cors_headers(&mut response, origin.as_deref(), is_allowed);
        return Ok(response);
    }

    let mut response = next.run(request).await;
    apply_cors_headers(&mut response, origin.as_deref(), is_allowed);

    Ok(response)
}

async fn is_origin_allowed(origin: &str, state: &Arc<AppState>) -> bool {
    if configured_origins().iter().any(|allowed| allowed == origin) {
        return true;
    }

    let servers = match server::Entity::find()
        .filter(server::Column::IsActive.eq(true))
        .all(&state.db)
        .await
    {
        Ok(servers) => servers,
        Err(error) => {
            tracing::warn!("Failed to fetch servers for CORS, falling back to configured origins only: {}", error);
            return false;
        }
    };

    servers.into_iter().any(|server| {
        origin_from_url(&server.relay_domain).as_deref() == Some(origin)
            || origin_from_url(&server.game_domain).as_deref() == Some(origin)
    })
}

fn configured_origins() -> Vec<String> {
    let configured = std::env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173,http://localhost:5176".to_string());

    configured
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(str::to_string)
        .collect()
}

fn origin_from_url(raw_url: &str) -> Option<String> {
    let url = Url::parse(raw_url).ok()?;
    let host = url.host_str()?;

    match url.port() {
        Some(port) => Some(format!("{}://{}:{}", url.scheme(), host, port)),
        None => Some(format!("{}://{}", url.scheme(), host)),
    }
}

fn apply_cors_headers(response: &mut Response, origin: Option<&str>, is_allowed: bool) {
    if is_allowed {
        if let Some(origin) = origin {
            if let Ok(origin) = HeaderValue::from_str(origin) {
                response
                    .headers_mut()
                    .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
            }
        }

        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            HeaderValue::from_static("content-type, authorization"),
        );
    }

    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS, PATCH"),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("authorization, content-type, accept, origin, x-requested-with"),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_MAX_AGE,
        HeaderValue::from_static("3600"),
    );
    response.headers_mut().insert(
        header::VARY,
        HeaderValue::from_static("Origin"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_origin_from_url_without_port() {
        assert_eq!(
            origin_from_url("https://game.example.com/path"),
            Some("https://game.example.com".to_string())
        );
    }

    #[test]
    fn test_extract_origin_from_url_with_port() {
        assert_eq!(
            origin_from_url("https://game.example.com:8080/path"),
            Some("https://game.example.com:8080".to_string())
        );
    }
}
