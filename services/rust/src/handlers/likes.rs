use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;

use crate::auth;
use crate::config::AppState;
use crate::stores::{likes as store, posts as posts_store};
use crate::types::{ApiError, LikeStatus};

/// GET /likes/{classification}/{category}/{slug}
/// Public. `liked` reflects the caller only when a valid Bearer token is sent.
/// Response: 200 LikeStatus, 404 when the post is unknown.
pub async fn status(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug)): Path<(String, String, String)>,
) -> Result<Json<LikeStatus>, ApiError> {
    let post = posts_store::get_by_slug(&state.db, &classification, &category, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    let like_count = store::count(&state.db, post.id).await?;

    let liked = match auth::extract_user_id(&state.config, &headers).ok() {
        Some(user_id) => store::has_liked(&state.db, post.id, user_id).await?,
        None => false,
    };

    Ok(Json(LikeStatus { like_count, liked }))
}

/// POST /likes/{classification}/{category}/{slug}
/// Auth: Bearer JWT. Idempotent. Response: 204, 401, 404 unknown post.
pub async fn add(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug)): Path<(String, String, String)>,
) -> Result<StatusCode, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;

    let post = posts_store::get_by_slug(&state.db, &classification, &category, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    store::add(&state.db, post.id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /likes/{classification}/{category}/{slug}
/// Auth: Bearer JWT. Response: 204, 401, 404 unknown post.
pub async fn remove(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug)): Path<(String, String, String)>,
) -> Result<StatusCode, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;

    let post = posts_store::get_by_slug(&state.db, &classification, &category, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    store::remove(&state.db, post.id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
