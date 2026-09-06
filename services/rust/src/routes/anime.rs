use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router(upload_limit: usize) -> Router<AppState> {
    Router::new().route(
        "/clips",
        post(handlers::anime_clips::upload_clip)
            .get(handlers::anime_clips::list_clips)
            .layer(DefaultBodyLimit::max(upload_limit)),
    )
}
