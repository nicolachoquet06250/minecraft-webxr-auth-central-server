use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use std::env;

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
pub struct Assets;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == "index.html" {
        return index_html().await;
    }

    match <Assets as rust_embed::Embed>::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            
            // Si c'est un fichier JavaScript, remplacer l'URL de l'API
            let body = if mime.as_ref() == "application/javascript" || mime.as_ref() == "text/javascript" {
                replace_api_url(content.data.as_ref())
            } else {
                content.data.to_vec()
            };
            
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(body))
                .unwrap()
        }
        None => {
            // Pour les routes SPA, retourner index.html
            if should_return_index(path) {
                index_html().await
            } else {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"))
                    .unwrap()
            }
        }
    }
}

async fn index_html() -> Response {
    match <Assets as rust_embed::Embed>::get("index.html") {
        Some(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(content.data))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("index.html not found"))
            .unwrap(),
    }
}

fn should_return_index(path: &str) -> bool {
    // Retourne index.html pour les routes SPA (pas de point = pas d'extension de fichier)
    !path.contains('.')
}

/// Remplace l'URL de l'API par défaut par la valeur de la variable DOMAIN
fn replace_api_url(content: &[u8]) -> Vec<u8> {
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_url = format!("{}/api", domain);
    
    // Convertir le contenu en String
    if let Ok(mut text) = String::from_utf8(content.to_vec()) {
        // Remplacer l'URL par défaut par la valeur de DOMAIN
        text = text.replace("http://localhost:8080/api", &api_url);
        
        tracing::debug!("Replaced API URL with: {}", api_url);
        
        text.into_bytes()
    } else {
        // Si ce n'est pas du texte valide UTF-8, retourner tel quel
        content.to_vec()
    }
}
