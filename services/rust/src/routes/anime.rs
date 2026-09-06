use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, post};
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
    .route("/clips/{id}/view", post(handlers::anime_clips::view_clip))
    .route(
        "/clips/{id}/like",
        post(handlers::anime_clips::like_clip).delete(handlers::anime_clips::unlike_clip),
    )
    .route(
        "/clips/{id}/media",
        delete(handlers::anime_clips::clear_clip_media),
    )
}
