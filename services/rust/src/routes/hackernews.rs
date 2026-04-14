use axum::routing::{delete, get, post};
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/likes", get(handlers::hackernews::list_likes))
        .route("/likes/{hn_id}", post(handlers::hackernews::add_like))
        .route("/likes/{hn_id}", delete(handlers::hackernews::remove_like))
}
