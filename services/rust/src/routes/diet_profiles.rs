use axum::routing::get;
use axum::Router;

use crate::config::AppState;
use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/",
        get(handlers::diet_profiles::get_profile).put(handlers::diet_profiles::upsert_profile),
    )
}
