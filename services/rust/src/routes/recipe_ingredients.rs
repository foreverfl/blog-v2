use axum::routing::{delete, post};
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::recipe_ingredients::create_ingredient))
        .route(
            "/{id}",
            delete(handlers::recipe_ingredients::delete_ingredient),
        )
}
