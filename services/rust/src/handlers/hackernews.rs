use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Serialize;

use crate::auth;
use crate::config::AppState;
use crate::stores::hackernews_likes as store;
use crate::types::ApiError;

#[derive(Serialize)]
pub struct LikesResponse {
    pub ids: Vec<i64>,
}

// GET /hackernews/likes
pub async fn list_likes(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<LikesResponse>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    let ids = store::list(&state.db, user_id).await?;
    Ok(Json(LikesResponse { ids }))
}

// POST /hackernews/likes/{hn_id}
pub async fn add_like(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(hn_id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    store::insert(&state.db, user_id, hn_id).await?;
    Ok(StatusCode::OK)
}

// DELETE /hackernews/likes/{hn_id}
pub async fn remove_like(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(hn_id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;
    store::delete(&state.db, user_id, hn_id).await?;
    Ok(StatusCode::OK)
}
