mod posts;
mod import;
mod uploads;

use axum::http::{header, Method};
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::config::AppState;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            state
                .config
                .frontend_url
                .parse::<axum::http::HeaderValue>()
                .expect("FRONTEND_URL must be a valid header value"),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            "X-Import-Secret".parse().unwrap(),
        ])
        .allow_credentials(true);

    let upload_limit = state.config.max_upload_size;

    Router::new()
        .route("/health", get(health))
        .nest("/posts", posts::router())
        .nest("/uploads", uploads::router(upload_limit))
        .nest("/import", import::router())
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