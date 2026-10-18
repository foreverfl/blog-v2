use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, put};
use axum::{Json, Router};

use crate::auth;
use crate::config::AppState;
use crate::types::ApiError;

use super::model::{ListPlacesQuery, Place, PlaceVisit, SaveVisitRequest};
use super::store;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_places))
        .route("/mine", get(list_mine))
        .route("/{id}/mine", put(save_mine))
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

// PUT /places/{id}/mine
//
// Request: JSON { applied_count, last_applied_on, visited_on, visited_with, review };
// the whole record is replaced, so a missing field saves as null.
// Response: 200 with the saved PlaceVisit; 401 without a user JWT; 404 when
// no place has that id.
async fn save_mine(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(place_id): Path<uuid::Uuid>,
    Json(req): Json<SaveVisitRequest>,
) -> Result<Json<PlaceVisit>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    let visit = store::save_visit(&state.db, user_id, place_id, &req).await?;
    Ok(Json(visit))
}
