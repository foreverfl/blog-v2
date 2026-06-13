use axum::Router;

use crate::config::AppState;

use super::cuisines;

/// Recipe domain routes, mounted under `/recipe`.
/// Mirrors the `recipe` Postgres schema; new recipe endpoints nest here.
pub fn router() -> Router<AppState> {
    Router::new().nest("/cuisines", cuisines::router())
}
