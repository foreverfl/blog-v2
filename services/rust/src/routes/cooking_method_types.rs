use axum::routing::get;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handlers::cooking_method_types::list_cooking_method_types))
}
