use axum::extract::{Path, State};
use axum::Json;

use crate::config::AppState;
use crate::stores::{comments as store, posts as posts_store};
use crate::types::{ApiError, CommentResponse};

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
