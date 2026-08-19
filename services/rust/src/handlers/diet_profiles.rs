use axum::Json;
use axum::extract::{Query, State};
use axum::http::HeaderMap;

use crate::auth;
use crate::config::AppState;
use crate::services::diet_stats;
use crate::stores::diet_profiles as store;
use crate::types::{
    ApiError, DietProfile, DietProfileStats, DietProfileStatsQuery, UpsertDietProfileRequest,
};

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

// GET /diet/profile/stats
//
// Request: Authorization: Bearer <access_token> (user JWT),
//          optional ?weight=<kg> to preview a weight being typed in.
// Response: 200 with BMI and how far the goal is, in kcal and in hours of
//           walking or running. 401 missing/bad/expired token, 404 no profile,
//           400 when no weight was given and none has been recorded.
pub async fn get_profile_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<DietProfileStatsQuery>,
) -> Result<Json<DietProfileStats>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    let profile = store::get(&state.db, user_id).await?;

    let weight_kg = match query.weight {
        Some(weight) => weight,
        None => store::latest_weight(&state.db, user_id)
            .await?
            .ok_or_else(|| ApiError::BadRequest("no weight recorded yet — pass ?weight=".into()))?,
    };
    if weight_kg <= 0.0 {
        return Err(ApiError::BadRequest("weight must be greater than 0".into()));
    }

    Ok(Json(diet_stats::derive(&profile, weight_kg)))
}
