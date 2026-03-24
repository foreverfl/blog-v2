use axum::routing::{get, put};
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::posts::list_all).post(handlers::posts::create))
        .route("/unindexed", get(handlers::posts::list_unindexed))
        .route("/mark-indexed", put(handlers::posts::mark_indexed))
        .route("/{classification}", get(handlers::posts::list_by_classification))
        .route("/{classification}/{category}", get(handlers::posts::list_by_category))
        .route(
            "/{classification}/{category}/{slug}",
            get(handlers::posts::get_by_slug)
                .put(handlers::posts::update)
                .delete(handlers::posts::delete),
        )
}