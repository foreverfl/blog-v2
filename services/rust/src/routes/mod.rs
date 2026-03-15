use axum::extract::DefaultBodyLimit;
use axum::http::{header, Method};
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

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
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    let upload_limit = state.config.max_upload_size;

    let api_routes = Router::new()
        .route("/posts", get(handlers::posts::list).post(handlers::posts::create))
        .route("/posts/{id}", get(handlers::posts::get))
        .route("/posts/{id}", put(handlers::posts::update))
        .route("/posts/{id}", delete(handlers::posts::delete))
        .route(
            "/uploads",
            post(handlers::uploads::upload).layer(DefaultBodyLimit::max(upload_limit)),
        )
        .route("/sync/mdx", post(handlers::sync::sync_from_github))
        .route("/sync/json", post(handlers::sync::sync_json));

    Router::new()
        .route("/health", get(health))
        .merge(api_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                    )
                })
                .on_response(
                    |res: &axum::http::Response<_>, latency: std::time::Duration, _span: &Span| {
                        tracing::info!(status = %res.status(), latency = ?latency, "response");
                    },
                ),
        )
        .layer(cors)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
