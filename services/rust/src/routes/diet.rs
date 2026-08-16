use axum::Router;

use crate::config::AppState;

use super::diet_profiles;

/// Diet domain routes, mounted under `/diet`.
/// Every row here belongs to one user, so each endpoint reads the caller from
/// their JWT; new diet endpoints nest here.
pub fn router() -> Router<AppState> {
    Router::new().nest("/profile", diet_profiles::router())
}
