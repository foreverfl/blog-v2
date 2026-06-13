use axum::routing::get;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handlers::recipe_cuisines::list_cuisines))
}
