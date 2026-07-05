use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router(upload_limit: usize) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::assets::list_assets)
                .post(handlers::uploads::upload)
                .layer(DefaultBodyLimit::max(upload_limit)),
        )
        .route("/{id}", get(handlers::assets::get_asset))
}
