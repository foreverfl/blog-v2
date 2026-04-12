use axum::http::{header, Method};
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;

use crate::config::AppState;
use crate::handlers;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            state
                .config
                .frontend_url
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::COOKIE,
        ])
        .allow_credentials(true);

    Router::new()
        .route("/health", get(health))
        .route("/login/{provider}", get(handlers::login))
        .route("/callback/{provider}", get(handlers::callback))
        .route("/refresh", post(handlers::refresh))
        .route("/logout", post(handlers::logout))
        .route("/me", get(handlers::me))
        .layer(cors)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}