use axum::routing::post;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/mdx", post(handlers::sync::sync_from_github))
        .route("/json", post(handlers::sync::sync_json))
}