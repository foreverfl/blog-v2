use axum::routing::post;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handlers::bug_reports::create))
}
