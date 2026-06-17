use axum::{response::IntoResponse, Json};
use serde_json::json;
use utoipa::OpenApi;

use crate::{
    dto::{
        ActiveAvatarResponse, AuthResponse, AvatarResponse, ConfirmRegisterRequest, CreateServerRequest,
        DiscordOAuthUrl, FavoriteServerResponse, LoginRequest, RecordServerVisitRequest,
        RegisterCodeResponse, RegisterRequest, SaveAvatarRequest, ServerHistoryResponse, ServerResponse,
        UpdateAvatarRequest, UpdateServerRequest, UpdateUserRequest, UserResponse,
    },
    routes::openapi_paths::openapi_paths,
};

pub async fn openapi_json() -> impl IntoResponse {
    let mut document = serde_json::to_value(ApiDoc::openapi()).unwrap_or_else(|_| json!({}));
    if let Some(root) = document.as_object_mut() {
        root.insert("paths".to_string(), openapi_paths());
    }
    Json(document)
}

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        RegisterRequest,
        ConfirmRegisterRequest,
        RegisterCodeResponse,
        LoginRequest,
        AuthResponse,
        UserResponse,
        UpdateUserRequest,
        DiscordOAuthUrl,
        CreateServerRequest,
        UpdateServerRequest,
        RecordServerVisitRequest,
        ServerResponse,
        ServerHistoryResponse,
        FavoriteServerResponse,
        AvatarResponse,
        ActiveAvatarResponse,
        SaveAvatarRequest,
        UpdateAvatarRequest
    )),
    tags(
        (name = "openapi", description = "Document OpenAPI"),
        (name = "auth", description = "Authentification"),
        (name = "users", description = "Utilisateurs"),
        (name = "friends", description = "Amis et demandes d'amis"),
        (name = "avatars", description = "Avatars"),
        (name = "servers", description = "Serveurs"),
        (name = "mail", description = "Contact, support et mail"),
        (name = "security", description = "Sécurité du compte")
    ),
    servers((url = "/api"))
)]
struct ApiDoc;
