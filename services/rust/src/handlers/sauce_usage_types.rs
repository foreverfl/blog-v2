use axum::extract::State;
use axum::Json;

use crate::config::AppState;
use crate::stores::sauce_usage_types as store;
use crate::types::{ApiError, SauceUsageType};

// GET /recipe/sauce-usage-types
//
// Response: 200, JSON array of { code, name_ko, name_ja, name_en }, ordered by code.
// Public read-only — no auth.
pub async fn list_sauce_usage_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<SauceUsageType>>, ApiError> {
    let types = store::list(&state.db).await?;
    Ok(Json(types))
}
