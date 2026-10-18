use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::config::AppState;
use crate::types::ApiError;

use super::model::{ListPlacesQuery, Place};
use super::store;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_places))
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
