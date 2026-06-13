use axum::Router;

use crate::config::AppState;

use super::recipe_cooking_method_types;
use super::recipe_cuisines;
use super::recipe_sauce_usage_types;

/// Recipe domain routes, mounted under `/recipe`.
/// Mirrors the `recipe` Postgres schema; new recipe endpoints nest here.
pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/cuisines", recipe_cuisines::router())
        .nest("/sauce-usage-types", recipe_sauce_usage_types::router())
        .nest("/cooking-method-types", recipe_cooking_method_types::router())
}
