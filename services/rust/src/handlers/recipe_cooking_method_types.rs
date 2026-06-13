use axum::extract::State;
use axum::Json;

use crate::config::AppState;
use crate::stores::recipe_cooking_method_types as store;
use crate::types::{ApiError, CookingMethodType};

// GET /recipe/cooking-method-types
//
// Response: 200, JSON array of { code, name_ko, name_ja, name_en }, ordered by code.
// Public read-only — no auth.
pub async fn list_cooking_method_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<CookingMethodType>>, ApiError> {
    let types = store::list(&state.db).await?;
    Ok(Json(types))
}
