use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;

use crate::auth;
use crate::config::AppState;
use crate::stores::diet_profiles as store;
use crate::types::{ApiError, DietProfile, UpsertDietProfileRequest};

// GET /diet/profile
//
// Request: Authorization: Bearer <access_token> (user JWT).
// Response: 200 with the caller's own profile. 401 missing/bad/expired token,
//           404 when they have not made one yet.
pub async fn get_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<DietProfile>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    let profile = store::get(&state.db, user_id).await?;
    Ok(Json(profile))
}

// PUT /diet/profile
//
// Request: Authorization: Bearer <access_token> (user JWT),
//          JSON { height_cm, target_weight_kg?, bmr_kcal? }.
// Response: 200 with the stored profile, created on the first call and replaced
//           after that. 401 missing/bad/expired token, 400 a value at or below zero.
pub async fn upsert_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpsertDietProfileRequest>,
) -> Result<Json<DietProfile>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    let profile = store::upsert(&state.db, user_id, &req).await?;
    Ok(Json(profile))
}
