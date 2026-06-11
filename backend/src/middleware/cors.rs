use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use url::Url;

use crate::{models::server, AppState};

/// Middleware pour gérer le CORS de manière dynamique en fonction des serveurs enregistrés
pub async fn dynamic_cors_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Récupérer l'origine de la requête et la cloner immédiatement
    let origin = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();

    // Toujours autoriser localhost pour le développement
    let frontend_origin = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5176".to_string());
    
    let mut allowed_origins = vec![frontend_origin];
    
    // Récupérer tous les domaines des serveurs actifs
    let servers = server::Entity::find()
        .filter(server::Column::IsActive.eq(true))
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch servers for CORS: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Extraire les domaines des URLs
    for server in servers {
        // Ajouter relay_domain
        if let Ok(url) = Url::parse(&server.relay_domain) {
            if let Some(host) = url.host_str() {
                let origin_url = format!("{}://{}", url.scheme(), host);
                if !allowed_origins.contains(&origin_url) {
                    allowed_origins.push(origin_url);
                }
            }
        }
        
        // Ajouter game_domain
        if let Ok(url) = Url::parse(&server.game_domain) {
            if let Some(host) = url.host_str() {
                let origin_url = format!("{}://{}", url.scheme(), host);
                if !allowed_origins.contains(&origin_url) {
                    allowed_origins.push(origin_url);
                }
            }
        }
    }

    // Vérifier si l'origine est autorisée
    let is_allowed = allowed_origins.iter().any(|allowed| allowed == &origin);

    // Pour les requêtes OPTIONS (preflight), répondre immédiatement
    if request.method() == axum::http::Method::OPTIONS {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = StatusCode::NO_CONTENT;
        
        if is_allowed && !origin.is_empty() {
            response.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_str(&origin).unwrap(),
            );
        }
        
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS, PATCH"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("authorization, content-type, accept"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_static("3600"),
        );
        
        return Ok(response);
    }

    // Passer la requête au handler suivant
    let mut response = next.run(request).await;

    // Ajouter les headers CORS à la réponse
    if is_allowed && !origin.is_empty() {
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str(&origin).unwrap(),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            HeaderValue::from_static("content-type, authorization"),
        );
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_origin_from_url() {
        let url = Url::parse("https://game.example.com:8080/path").unwrap();
        assert_eq!(url.host_str(), Some("game.example.com"));
        assert_eq!(url.scheme(), "https");
        
        let origin = format!("{}://{}", url.scheme(), url.host_str().unwrap());
        assert_eq!(origin, "https://game.example.com");
    }
}
