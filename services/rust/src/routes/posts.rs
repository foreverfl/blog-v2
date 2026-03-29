use axum::routing::{get, post, put};
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::posts::create))
        .route("/recent", get(handlers::posts::list_posts))
        .route("/translate", post(handlers::translate::translate))
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