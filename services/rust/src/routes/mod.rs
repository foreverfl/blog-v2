mod assets;
mod bug_reports;
mod comments;
mod hackernews;
mod import;
mod likes;
mod posts;
mod recipe;
mod recipe_cooking_method_types;
mod recipe_cuisines;
mod recipe_ingredients;
mod recipe_sauce_usage_types;

use axum::http::{header, HeaderName, Method};
use axum::routing::get;
use axum::Router;
use opentelemetry::trace::TraceContextExt;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

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
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            HeaderName::from_static("traceparent"),
        ])
        .allow_credentials(true);

    let upload_limit = state.config.max_upload_size;

    Router::new()
        .route("/health", get(health))
        .nest("/assets", assets::router(upload_limit))
        .nest("/posts", posts::router())
        .nest("/recipe", recipe::router())
        .nest("/import", import::router())
        .nest("/hackernews", hackernews::router())
        .nest("/bug-reports", bug_reports::router())
        .nest("/comments", comments::router())
        .nest("/likes", likes::router())
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
                    // Adopt the caller's trace (browser traceparent) BEFORE reading
                    // the id — set_parent rewrites the span's trace id
                    let parent_context = opentelemetry::global::get_text_map_propagator(
                        |propagator| propagator.extract(&HeaderExtractor(req.headers())),
                    );
                    span.set_parent(parent_context);
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

// Minimal W3C trace-context extractor — saves the opentelemetry-http dependency
struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

impl opentelemetry::propagation::Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|key| key.as_str()).collect()
    }
}