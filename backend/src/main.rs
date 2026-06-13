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

use crate::services::{DiscordService, JwtService};

pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_service: JwtService,
    pub discord_service: DiscordService,
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

    let state = Arc::new(AppState {
        db,
        jwt_service,
        discord_service,
    });

    let public_routes = Router::new()
        .route("/auth/register", post(routes::auth::register))
        .route("/auth/login", post(routes::auth::login))
        .route("/auth/discord/url", get(routes::auth::discord_oauth_url))
        .route("/auth/discord/callback", get(routes::auth::discord_callback))
        .route("/users/:id", get(routes::user::get_user_by_id))
        .route("/servers/:id", get(routes::server::get_server));

    let protected_routes = Router::new()
        .route("/users/me", get(routes::user::get_profile))
        .route("/users/me", put(routes::user::update_profile))
        .route("/users/me", delete(routes::user::delete_account))
        .route("/users/me/avatar", get(routes::avatar::get_active_avatar))
        .route("/users/me/avatar", delete(routes::avatar::clear_active_avatar))
        .route("/users/me/avatars", get(routes::avatar::list_avatars))
        .route("/users/me/avatars", post(routes::avatar::create_avatar_copy))
        .route("/users/me/avatars/:id", put(routes::avatar::update_avatar))
        .route("/users/me/avatars/:id", delete(routes::avatar::delete_avatar))
        .route("/users/me/avatars/:id/select", put(routes::avatar::select_avatar))
        .route("/users/me/profile-pic.svg", get(routes::avatar::get_profile_pic_svg))
        .route("/users/me/profile-pic.svg", options(routes::avatar::profile_pic_preflight))
        .route("/users/:id/profile-pic.svg", get(routes::avatar::get_user_profile_pic_svg))
        .route("/users/:id/profile-pic.svg", options(routes::avatar::profile_pic_preflight))
        .route("/users/:id/matrix-color", get(routes::matrix_color::get_user_matrix_color))
        .route("/users/:id/matrix-color", options(routes::matrix_color::matrix_color_preflight))
        .route("/servers", post(routes::server::create_server))
        .route("/servers", get(routes::server::get_user_servers))
        .route("/servers/:id", put(routes::server::update_server))
        .route("/servers/:id", delete(routes::server::delete_server))
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
