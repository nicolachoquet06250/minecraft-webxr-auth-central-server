use axum::{
    extract::{Request, State},
    http::{header, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::{env, sync::Arc};
use url::Url;

use crate::{
    models::{server, Server},
    AppState,
};

pub async fn dynamic_cors_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let origin = req.headers().get(header::ORIGIN).cloned();

    if origin.is_none() {
        return Ok(next.run(req).await);
    }

    let origin = origin.expect("origin is checked above");
    let origin_text = origin.to_str().map_err(|_| StatusCode::FORBIDDEN)?;
    let normalized_origin = normalize_origin(origin_text).ok_or(StatusCode::FORBIDDEN)?;
    let is_allowed = is_origin_allowed(&state, &normalized_origin).await?;

    if !is_allowed {
        return Err(StatusCode::FORBIDDEN);
    }

    if req.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        add_preflight_cors_headers(response.headers_mut(), &origin);
        return Ok(response);
    }

    let mut response = next.run(req).await;
    add_response_cors_headers(response.headers_mut(), &origin);
    Ok(response)
}

async fn is_origin_allowed(state: &Arc<AppState>, origin: &str) -> Result<bool, StatusCode> {
    if configured_static_origins().iter().any(|allowed| allowed == origin) {
        return Ok(true);
    }

    let servers = Server::find()
        .filter(server::Column::IsActive.eq(true))
        .all(&state.db)
        .await
        .map_err(|error| {
            tracing::error!("Failed to fetch servers for CORS: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(servers
        .iter()
        .filter_map(|server| normalize_origin(&server.game_domain))
        .any(|allowed| allowed == origin))
}

fn configured_static_origins() -> Vec<String> {
    env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173,http://localhost:5176".to_string())
        .split(',')
        .filter_map(normalize_origin)
        .collect()
}

fn normalize_origin(value: &str) -> Option<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return None;
    }

    match Url::parse(trimmed) {
        Ok(url) => {
            let scheme = url.scheme();
            let host = url.host_str()?;
            match url.port() {
                Some(port) => Some(format!("{}://{}:{}", scheme, host, port)),
                None => Some(format!("{}://{}", scheme, host)),
            }
        }
        Err(_) => None,
    }
}

fn add_preflight_cors_headers(headers: &mut header::HeaderMap, origin: &HeaderValue) {
    add_common_cors_headers(headers, origin);
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS, PATCH"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("authorization, content-type, accept, x-requested-with"),
    );
    headers.insert(
        header::ACCESS_CONTROL_MAX_AGE,
        HeaderValue::from_static("3600"),
    );
}

fn add_response_cors_headers(headers: &mut header::HeaderMap, origin: &HeaderValue) {
    add_common_cors_headers(headers, origin);
    headers.insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("content-type, authorization"),
    );
}

fn add_common_cors_headers(headers: &mut header::HeaderMap, origin: &HeaderValue) {
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    headers.insert(header::VARY, HeaderValue::from_static("Origin"));
}
