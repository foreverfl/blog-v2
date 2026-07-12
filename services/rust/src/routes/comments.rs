use axum::routing::get;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/{classification}/{category}/{slug}",
        get(handlers::comments::list).post(handlers::comments::create),
    )
}
