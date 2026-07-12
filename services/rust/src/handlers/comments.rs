use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth;
use crate::config::AppState;
use crate::stores::{comments as store, posts as posts_store};
use crate::types::{ApiError, CommentResponse};

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

/// GET /comments/{classification}/{category}/{slug}
/// Response: 200 [CommentResponse] (oldest first), 404 when the post is unknown.
pub async fn list(
    State(state): State<AppState>,
    Path((classification, category, slug)): Path<(String, String, String)>,
) -> Result<Json<Vec<CommentResponse>>, ApiError> {
    let post = posts_store::get_by_slug(&state.db, &classification, &category, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    let comments = store::list_for_post(&state.db, post.id).await?;
    Ok(Json(comments))
}

/// POST /comments/{classification}/{category}/{slug}
/// Auth: Bearer JWT. Request: { content }. Response: 200 CommentResponse,
/// 401 unauthenticated, 404 unknown post, 400 empty content.
pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug)): Path<(String, String, String)>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;

    if req.content.trim().is_empty() {
        return Err(ApiError::BadRequest("content is required".into()));
    }

    let post = posts_store::get_by_slug(&state.db, &classification, &category, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    let comment = store::create(&state.db, post.id, user_id, &req.content).await?;
    Ok(Json(comment))
}

/// PATCH /comments/{classification}/{category}/{slug}/{comment_id}
/// Auth: Bearer JWT. Request: { content }. Edits only the caller's own comment.
/// Response: 200 CommentResponse, 401, 404 (unknown or not owned), 400 empty.
pub async fn update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_classification, _category, _slug, comment_id)): Path<(String, String, String, Uuid)>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;

    if req.content.trim().is_empty() {
        return Err(ApiError::BadRequest("content is required".into()));
    }

    let comment = store::update(&state.db, comment_id, user_id, &req.content)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(comment))
}

/// DELETE /comments/{classification}/{category}/{slug}/{comment_id}
/// Auth: Bearer JWT. Removes only the caller's own comment.
/// Response: 204 No Content, 401, 404 (unknown or not owned).
pub async fn remove(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_classification, _category, _slug, comment_id)): Path<(String, String, String, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;

    if store::delete(&state.db, comment_id, user_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}
