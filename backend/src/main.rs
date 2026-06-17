mod dto;
mod middleware;
mod models;
mod routes;
mod services;
mod static_files;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, options, post, put},
    Router,
};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::{env, net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::services::{DiscordService, JwtService, MailService, PasswordChangeCodeStore};

pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_service: JwtService,
    pub discord_service: DiscordService,
    pub mail_service: MailService,
    pub password_change_codes: PasswordChangeCodeStore,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "voxicraft_auth_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let discord_client_id = env::var("DISCORD_CLIENT_ID").unwrap_or_else(|_| "".to_string());
    let discord_client_secret = env::var("DISCORD_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());
    let discord_redirect_uri = env::var("DISCORD_REDIRECT_URI").unwrap_or_else(|_| "".to_string());
    let api_port = env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());
    let mut api_host = env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let jwt_service = JwtService::new(&jwt_secret);
    let discord_service = DiscordService::new(
        discord_client_id,
        discord_client_secret,
        discord_redirect_uri,
    );
    let mail_service = MailService::from_env();
    let password_change_codes = PasswordChangeCodeStore::new();

    let state = Arc::new(AppState {
        db,
        jwt_service,
        discord_service,
        mail_service,
        password_change_codes,
    });

    let public_routes = Router::new()
        .route("/openapi.json", get(routes::openapi::openapi_json))
        .route("/auth/register", post(routes::auth::register))
        .route("/auth/login", post(routes::auth::login))
        .route("/auth/discord/url", get(routes::auth::discord_oauth_url))
        .route("/auth/discord/callback", get(routes::auth::discord_callback))
        .route("/mail/status", get(routes::mail::mail_status))
        .route("/contact", post(routes::mail::send_contact_mail))
        .route("/support", post(routes::mail::send_support_mail));

    let protected_routes = Router::new()
        .route("/users/me", get(routes::user::get_profile))
        .route("/users/me", put(routes::user::update_profile))
        .route("/users/me", delete(routes::user::delete_account))
        .route("/users/me/discord", delete(routes::user::unlink_discord))
        .route("/users/me/avatar", get(routes::avatar::get_active_avatar))
        .route("/users/me/avatar", delete(routes::avatar::clear_active_avatar))
        .route("/users/me/avatars", get(routes::avatar::list_avatars))
        .route("/users/me/avatars", post(routes::avatar::create_avatar_copy))
        .route("/users/me/avatars/:id", put(routes::avatar::update_avatar))
        .route("/users/me/avatars/:id", delete(routes::avatar_delete::delete_avatar))
        .route("/users/me/avatars/:id/select", put(routes::avatar::select_avatar))
        .route("/users/me/profile-pic.svg", get(routes::avatar::get_profile_pic_svg))
        .route("/users/me/profile-pic.svg", options(routes::avatar::profile_pic_preflight))
        .route("/users/search", get(routes::user::search_users))
        .route("/users/:id", get(routes::user::get_user_by_id))
        .route("/users/:id/profile-pic.svg", get(routes::avatar::get_user_profile_pic_svg))
        .route("/users/:id/profile-pic.svg", options(routes::avatar::profile_pic_preflight))
        .route("/users/:id/matrix-color", get(routes::matrix_color::get_user_matrix_color))
        .route("/users/:id/matrix-color", options(routes::matrix_color::matrix_color_preflight))
        .route("/friends", get(routes::friends::get_friends))
        .route("/friends/requests", post(routes::friends::create_friend_request))
        .route("/friends/requests/incoming", get(routes::friends::get_incoming_friend_requests))
        .route("/friends/requests/outgoing", get(routes::friends::get_outgoing_friend_requests))
        .route("/friends/requests/:id/accept", post(routes::friends::accept_friend_request))
        .route("/friends/requests/:id/refuse", post(routes::friends::refuse_friend_request))
        .route("/friends/:userId", delete(routes::friends::delete_friend))
        .route("/servers", post(routes::server::create_server))
        .route("/servers", get(routes::server::get_user_servers))
        .route("/servers/recent", get(routes::server::get_recent_servers))
        .route("/servers/favorites", get(routes::server::get_favorite_servers))
        .route("/servers/visit", post(routes::server::record_server_visit))
        .route("/servers/:id", get(routes::server::get_server))
        .route("/servers/:id/favorite", post(routes::server::favorite_server))
        .route("/servers/:id/favorite", delete(routes::server::unfavorite_server))
        .route("/servers/:id", put(routes::server::update_server))
        .route("/servers/:id", delete(routes::server::delete_server))
        .route("/users/me/password/change-code", post(routes::account_security::request_credential_code))
        .route("/users/me/password", put(routes::account_security::confirm_credential_change))
        .layer(axum_middleware::from_fn_with_state(state.clone(), middleware::auth_middleware));

    let app = Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .fallback(static_files::static_handler)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::cors::dynamic_cors_middleware,
        ))
        .with_state(state);

    if api_host.contains(':') {
        api_host = format!("[{}]", api_host);
    }

    let addr: SocketAddr = format!("{}:{}", api_host, api_port).parse().unwrap();

    tracing::info!("Server listening on http://{}:{}", api_host, api_port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
