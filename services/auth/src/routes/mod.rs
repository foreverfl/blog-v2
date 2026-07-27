use axum::http::{header, Method};
use axum::routing::{get, post};
use axum::Router;
use opentelemetry::trace::TraceContextExt;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

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
        .route("/login-cli", get(handlers::login_cli))
        .route("/callback/{provider}", get(handlers::callback))
        .route("/refresh", post(handlers::refresh))
        .route("/logout", post(handlers::logout))
        .route("/me", get(handlers::me))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    // Route template, not the real path — keeps span names low-cardinality
                    let route = req
                        .extensions()
                        .get::<axum::extract::MatchedPath>()
                        .map(axum::extract::MatchedPath::as_str)
                        .unwrap_or("unmatched");
                    let span = tracing::info_span!(
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                        trace_id = tracing::field::Empty,
                        otel.name = %format!("{} {}", req.method(), route),
                        otel.kind = "server",
                        otel.status_code = tracing::field::Empty,
                    );
                    // Otel assigns the id at span creation; expose it for log↔trace links
                    let trace_id = span.context().span().span_context().trace_id();
                    span.record("trace_id", tracing::field::display(trace_id));
                    span
                })
                .on_response(
                    |res: &axum::http::Response<_>, latency: std::time::Duration, span: &Span| {
                        if res.status().is_server_error() {
                            span.record("otel.status_code", "ERROR");
                        }
                        // Numeric field so Loki can filter on it (| json | latency_ms > 100)
                        tracing::info!(
                            status = res.status().as_u16(),
                            latency_ms = latency.as_millis() as u64,
                            "response"
                        );
                    },
                ),
        )
        .layer(cors)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}