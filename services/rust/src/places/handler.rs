use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::routing::get;
use axum::{Json, Router};

use crate::auth;
use crate::config::AppState;
use crate::types::ApiError;

use super::model::{ListPlacesQuery, Place, PlaceVisit};
use super::store;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_places))
        .route("/mine", get(list_mine))
}

// GET /places?category=&subcategory=
//
// Response: 200, JSON array of Place ordered by name; ITS-only fields are null
// for rows that are not ITS facilities. Public read-only — no auth.
async fn list_places(
    State(state): State<AppState>,
    Query(query): Query<ListPlacesQuery>,
) -> Result<Json<Vec<Place>>, ApiError> {
    let places = store::list(
        &state.db,
        query.category.as_deref(),
        query.subcategory.as_deref(),
    )
    .await?;
    Ok(Json(places))
}

// GET /places/mine
//
// Response: 200, JSON array of PlaceVisit for the caller (empty when none).
// Requires a user JWT; 401 without one.
async fn list_mine(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<PlaceVisit>>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    let visits = store::list_visits(&state.db, user_id).await?;
    Ok(Json(visits))
}
