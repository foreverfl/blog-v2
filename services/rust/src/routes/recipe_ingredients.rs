use axum::routing::{get, post};
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::recipe_ingredients::create_ingredient))
        .route(
            "/{id}",
            get(handlers::recipe_ingredients::get_ingredient)
                .patch(handlers::recipe_ingredients::update_ingredient)
                .delete(handlers::recipe_ingredients::delete_ingredient),
        )
}
