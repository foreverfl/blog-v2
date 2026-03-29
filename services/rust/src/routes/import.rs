use axum::routing::post;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/mdx", post(handlers::import::import_mdx_from_github))
        .route("/json", post(handlers::import::import_json))
}
