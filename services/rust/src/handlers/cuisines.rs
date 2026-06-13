use axum::extract::State;
use axum::Json;

use crate::config::AppState;
use crate::stores::cuisines as store;
use crate::types::{ApiError, Cuisine};

// GET /recipe/cuisines
//
// Response: 200, JSON array of { code, name_ko, name_ja, name_en }, ordered by code.
// Public read-only — no auth.
pub async fn list_cuisines(State(state): State<AppState>) -> Result<Json<Vec<Cuisine>>, ApiError> {
    let cuisines = store::list(&state.db).await?;
    Ok(Json(cuisines))
}
