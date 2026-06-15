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
    paths(
        openapi_json_doc,
        register_doc,
        confirm_register_doc,
        login_doc,
        discord_url_doc,
        discord_callback_doc,
        mail_status_doc,
        contact_doc,
        support_doc,
        get_public_user_doc,
        get_profile_doc,
        update_profile_doc,
        delete_account_doc,
        unlink_discord_doc,
        get_active_avatar_doc,
        clear_active_avatar_doc,
        list_avatars_doc,
        create_avatar_doc,
        update_avatar_doc,
        delete_avatar_doc,
        select_avatar_doc,
        get_own_profile_pic_doc,
        get_user_profile_pic_doc,
        get_matrix_color_doc,
        list_servers_doc,
        create_server_doc,
        list_recent_servers_doc,
        list_favorite_servers_doc,
        record_server_visit_doc,
        get_server_doc,
        update_server_doc,
        delete_server_doc,
        favorite_server_doc,
        unfavorite_server_doc,
        request_password_code_doc,
        confirm_password_change_doc
    ),
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

#[utoipa::path(get, path = "/openapi.json", tag = "openapi", responses((status = 200, description = "Document OpenAPI", body = Object)))]
#[allow(dead_code)]
async fn openapi_json_doc() {}

#[utoipa::path(post, path = "/auth/register", tag = "auth", request_body = RegisterRequest, responses((status = 200, body = RegisterCodeResponse), (status = 400, description = "Requête invalide")))]
#[allow(dead_code)]
async fn register_doc() {}

#[utoipa::path(post, path = "/auth/register/confirm", tag = "auth", request_body = ConfirmRegisterRequest, responses((status = 200, body = AuthResponse), (status = 400, description = "Code invalide")))]
#[allow(dead_code)]
async fn confirm_register_doc() {}

#[utoipa::path(post, path = "/auth/login", tag = "auth", request_body = LoginRequest, responses((status = 200, body = AuthResponse), (status = 401, description = "Identifiants invalides")))]
#[allow(dead_code)]
async fn login_doc() {}

#[utoipa::path(get, path = "/auth/discord/url", tag = "auth", responses((status = 200, body = DiscordOAuthUrl)))]
#[allow(dead_code)]
async fn discord_url_doc() {}

#[utoipa::path(get, path = "/auth/discord/callback", tag = "auth", responses((status = 302, description = "Redirection après authentification Discord")))]
#[allow(dead_code)]
async fn discord_callback_doc() {}

#[utoipa::path(get, path = "/mail/status", tag = "mail", responses((status = 200, description = "Statut mail", body = Object)))]
#[allow(dead_code)]
async fn mail_status_doc() {}

#[utoipa::path(post, path = "/contact", tag = "mail", responses((status = 200, description = "Message envoyé", body = Object)))]
#[allow(dead_code)]
async fn contact_doc() {}

#[utoipa::path(post, path = "/support", tag = "mail", responses((status = 200, description = "Demande support envoyée", body = Object)))]
#[allow(dead_code)]
async fn support_doc() {}

#[utoipa::path(get, path = "/users/{id}", tag = "users", params(("id" = String, Path, description = "UUID utilisateur. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 200, body = UserResponse), (status = 404, description = "Utilisateur introuvable")))]
#[allow(dead_code)]
async fn get_public_user_doc() {}

#[utoipa::path(get, path = "/users/me", tag = "users", security(("bearerAuth" = [])), responses((status = 200, body = UserResponse)))]
#[allow(dead_code)]
async fn get_profile_doc() {}

#[utoipa::path(put, path = "/users/me", tag = "users", security(("bearerAuth" = [])), request_body = UpdateUserRequest, responses((status = 200, body = UserResponse)))]
#[allow(dead_code)]
async fn update_profile_doc() {}

#[utoipa::path(delete, path = "/users/me", tag = "users", security(("bearerAuth" = [])), responses((status = 204, description = "Compte supprimé")))]
#[allow(dead_code)]
async fn delete_account_doc() {}

#[utoipa::path(delete, path = "/users/me/discord", tag = "users", security(("bearerAuth" = [])), responses((status = 204, description = "Discord délié")))]
#[allow(dead_code)]
async fn unlink_discord_doc() {}

#[utoipa::path(get, path = "/users/me/avatar", tag = "avatars", security(("bearerAuth" = [])), responses((status = 200, body = ActiveAvatarResponse)))]
#[allow(dead_code)]
async fn get_active_avatar_doc() {}

#[utoipa::path(delete, path = "/users/me/avatar", tag = "avatars", security(("bearerAuth" = [])), responses((status = 204, description = "Avatar actif supprimé")))]
#[allow(dead_code)]
async fn clear_active_avatar_doc() {}

#[utoipa::path(get, path = "/users/me/avatars", tag = "avatars", security(("bearerAuth" = [])), responses((status = 200, body = [AvatarResponse])))]
#[allow(dead_code)]
async fn list_avatars_doc() {}

#[utoipa::path(post, path = "/users/me/avatars", tag = "avatars", security(("bearerAuth" = [])), request_body = SaveAvatarRequest, responses((status = 200, body = AvatarResponse)))]
#[allow(dead_code)]
async fn create_avatar_doc() {}

#[utoipa::path(put, path = "/users/me/avatars/{id}", tag = "avatars", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID avatar. Regex: ^[0-9a-fA-F-]{36}$")), request_body = UpdateAvatarRequest, responses((status = 200, body = AvatarResponse)))]
#[allow(dead_code)]
async fn update_avatar_doc() {}

#[utoipa::path(delete, path = "/users/me/avatars/{id}", tag = "avatars", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID avatar. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 204, description = "Avatar supprimé")))]
#[allow(dead_code)]
async fn delete_avatar_doc() {}

#[utoipa::path(put, path = "/users/me/avatars/{id}/select", tag = "avatars", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID avatar. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 204, description = "Avatar sélectionné")))]
#[allow(dead_code)]
async fn select_avatar_doc() {}

#[utoipa::path(get, path = "/users/me/profile-pic.svg", tag = "avatars", security(("bearerAuth" = [])), responses((status = 200, description = "Image SVG", content_type = "image/svg+xml", body = String)))]
#[allow(dead_code)]
async fn get_own_profile_pic_doc() {}

#[utoipa::path(get, path = "/users/{id}/profile-pic.svg", tag = "avatars", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID utilisateur. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 200, description = "Image SVG", content_type = "image/svg+xml", body = String)))]
#[allow(dead_code)]
async fn get_user_profile_pic_doc() {}

#[utoipa::path(get, path = "/users/{id}/matrix-color", tag = "avatars", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID utilisateur. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 200, description = "Couleur Matrix", body = Object)))]
#[allow(dead_code)]
async fn get_matrix_color_doc() {}

#[utoipa::path(get, path = "/servers", tag = "servers", security(("bearerAuth" = [])), responses((status = 200, body = [ServerResponse])))]
#[allow(dead_code)]
async fn list_servers_doc() {}

#[utoipa::path(post, path = "/servers", tag = "servers", security(("bearerAuth" = [])), request_body = CreateServerRequest, responses((status = 200, body = ServerResponse)))]
#[allow(dead_code)]
async fn create_server_doc() {}

#[utoipa::path(get, path = "/servers/recent", tag = "servers", security(("bearerAuth" = [])), responses((status = 200, body = [ServerHistoryResponse])))]
#[allow(dead_code)]
async fn list_recent_servers_doc() {}

#[utoipa::path(get, path = "/servers/favorites", tag = "servers", security(("bearerAuth" = [])), responses((status = 200, body = [FavoriteServerResponse])))]
#[allow(dead_code)]
async fn list_favorite_servers_doc() {}

#[utoipa::path(post, path = "/servers/visit", tag = "servers", security(("bearerAuth" = [])), request_body = RecordServerVisitRequest, responses((status = 200, description = "Visite enregistrée", body = Object)))]
#[allow(dead_code)]
async fn record_server_visit_doc() {}

#[utoipa::path(get, path = "/servers/{id}", tag = "servers", params(("id" = String, Path, description = "UUID serveur. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 200, body = ServerResponse)))]
#[allow(dead_code)]
async fn get_server_doc() {}

#[utoipa::path(put, path = "/servers/{id}", tag = "servers", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID serveur. Regex: ^[0-9a-fA-F-]{36}$")), request_body = UpdateServerRequest, responses((status = 200, body = ServerResponse)))]
#[allow(dead_code)]
async fn update_server_doc() {}

#[utoipa::path(delete, path = "/servers/{id}", tag = "servers", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID serveur. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 204, description = "Serveur supprimé")))]
#[allow(dead_code)]
async fn delete_server_doc() {}

#[utoipa::path(post, path = "/servers/{id}/favorite", tag = "servers", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID serveur. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 200, description = "Favori ajouté", body = Object)))]
#[allow(dead_code)]
async fn favorite_server_doc() {}

#[utoipa::path(delete, path = "/servers/{id}/favorite", tag = "servers", security(("bearerAuth" = [])), params(("id" = String, Path, description = "UUID serveur. Regex: ^[0-9a-fA-F-]{36}$")), responses((status = 204, description = "Favori supprimé")))]
#[allow(dead_code)]
async fn unfavorite_server_doc() {}

#[utoipa::path(post, path = "/users/me/password/change-code", tag = "security", security(("bearerAuth" = [])), responses((status = 200, description = "Code envoyé", body = Object)))]
#[allow(dead_code)]
async fn request_password_code_doc() {}

#[utoipa::path(put, path = "/users/me/password", tag = "security", security(("bearerAuth" = [])), responses((status = 200, description = "Mot de passe changé", body = Object)))]
#[allow(dead_code)]
async fn confirm_password_change_doc() {}
