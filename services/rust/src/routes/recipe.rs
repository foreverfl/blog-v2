use axum::Router;

use crate::config::AppState;

use super::cuisines;
use super::sauce_usage_types;

/// Recipe domain routes, mounted under `/recipe`.
/// Mirrors the `recipe` Postgres schema; new recipe endpoints nest here.
pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/cuisines", cuisines::router())
        .nest("/sauce-usage-types", sauce_usage_types::router())
}
