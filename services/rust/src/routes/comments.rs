use axum::routing::{get, patch};
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/{classification}/{category}/{slug}",
            get(handlers::comments::list).post(handlers::comments::create),
        )
        .route(
            "/{classification}/{category}/{slug}/{comment_id}",
            patch(handlers::comments::update).delete(handlers::comments::remove),
        )
        .route(
            "/{classification}/{category}/{slug}/{comment_id}/admin",
            patch(handlers::comments::update_reply).delete(handlers::comments::delete_reply),
        )
}
