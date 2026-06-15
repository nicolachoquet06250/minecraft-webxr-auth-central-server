use axum::{response::IntoResponse, Json};
use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        OpenApi as OpenApiDocument,
    },
    Modify, OpenApi,
};

use crate::dto::{
    ActiveAvatarResponse, AuthResponse, AvatarResponse, ConfirmRegisterRequest, CreateServerRequest,
    DiscordOAuthUrl, FavoriteServerResponse, LoginRequest, RecordServerVisitRequest,
    RegisterCodeResponse, RegisterRequest, SaveAvatarRequest, ServerHistoryResponse, ServerResponse,
    UpdateAvatarRequest, UpdateServerRequest, UpdateUserRequest, UserResponse,
};

pub async fn openapi_json() -> impl IntoResponse {
    Json(ApiDoc::openapi())
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
    modifiers(&SecurityAddon),
    tags(
        (name = "openapi", description = "Document OpenAPI"),
        (name = "auth", description = "Authentification"),
        (name = "users", description = "Utilisateurs"),
        (name = "avatars", description = "Avatars"),
        (name = "servers", description = "Serveurs"),
        (name = "mail", description = "Contact, support et mail"),
        (name = "security", description = "Sécurité du compte")
    ),
    servers((url = "/api"))
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut OpenApiDocument) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
