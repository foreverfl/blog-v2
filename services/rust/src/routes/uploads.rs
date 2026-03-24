use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router(upload_limit: usize) -> Router<AppState> {
    Router::new().route(
        "/",
        post(handlers::uploads::upload).layer(DefaultBodyLimit::max(upload_limit)),
    )
}