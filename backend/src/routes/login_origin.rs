use axum::{http::{header, HeaderMap, StatusCode}};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use url::Url;

use crate::{models::{server, Server}, AppState};

const CENTRAL_HOSTS: [&str; 2] = ["central.voxicraft.fr", "localhost"];

pub async fn is_allowed(state: &Arc<AppState>, headers: &HeaderMap) -> Result<bool, StatusCode> {
    let Some(origin) = origin_from_headers(headers) else {
        return Ok(false);
    };

    if is_central_origin(&origin) {
        return Ok(true);
    }

    let server = Server::find()
        .filter(server::Column::GameDomain.eq(origin))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(server.is_some())
}

fn origin_from_headers(headers: &HeaderMap) -> Option<String> {
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

fn is_central_origin(origin: &str) -> bool {
    let Ok(url) = Url::parse(origin) else {
        return false;
    };
    let Some(host) = url.host_str() else {
        return false;
    };
    CENTRAL_HOSTS.contains(&host)
}
