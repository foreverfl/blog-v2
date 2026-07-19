use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use chrono::{FixedOffset, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth;
use crate::config::{AppConfig, AppState};
use crate::services::discord;
use crate::stores::{comments as store, posts as posts_store};
use crate::types::{ApiError, CommentResponse};

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

/// Best-effort admin notification about a comment event. Spawned so it never
/// blocks or fails the request — a Discord outage must not break commenting.
fn notify_admin(config: &AppConfig, message: String) {
    let Some(webhook_url) = config.discord_comments_webhook.clone() else {
        tracing::warn!("DISCORD_COMMENTS_WEBHOOK not set — comment notify skipped");
        return;
    };
    tokio::spawn(async move {
        if let Err(e) = discord::send_webhook(&webhook_url, &message).await {
            tracing::warn!("comment discord notify failed: {e}");
        }
    });
}

fn comment_message(
    frontend_url: &str,
    action: &str,
    classification: &str,
    category: &str,
    slug: &str,
    username: &str,
    content: Option<&str>,
) -> String {
    let now_kst = Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    let base = frontend_url.trim_end_matches('/');
    let mut message = format!(
        "[{}]\n# {action}\n\n## Post\n{base}/ko/{classification}/{category}/{slug}\n\n## User\n{username}",
        now_kst.format("%Y-%m-%d %H:%M:%S"),
    );
    if let Some(body) = content {
        message.push_str(&format!("\n\n## Content\n{body}"));
    }
    message
}

#[derive(Debug, Deserialize)]
pub struct AdminReplyRequest {
    pub reply: String,
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
    notify_admin(
        &state.config,
        comment_message(
            &state.config.frontend_url,
            "New Comment",
            &classification,
            &category,
            &slug,
            &comment.username,
            Some(&comment.content),
        ),
    );
    Ok(Json(comment))
}

/// PATCH /comments/{classification}/{category}/{slug}/{comment_id}
/// Auth: Bearer JWT. Request: { content }. Edits only the caller's own comment.
/// Response: 200 CommentResponse, 401, 404 (unknown or not owned), 400 empty.
pub async fn update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug, comment_id)): Path<(String, String, String, Uuid)>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;

    if req.content.trim().is_empty() {
        return Err(ApiError::BadRequest("content is required".into()));
    }

    let comment = store::update(&state.db, comment_id, user_id, &req.content)
        .await?
        .ok_or(ApiError::NotFound)?;
    notify_admin(
        &state.config,
        comment_message(
            &state.config.frontend_url,
            "Update Comment",
            &classification,
            &category,
            &slug,
            &comment.username,
            Some(&comment.content),
        ),
    );
    Ok(Json(comment))
}

/// DELETE /comments/{classification}/{category}/{slug}/{comment_id}
/// Auth: Bearer JWT. Removes only the caller's own comment.
/// Response: 204 No Content, 401, 404 (unknown or not owned).
pub async fn remove(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug, comment_id)): Path<(String, String, String, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let user_id = auth::extract_user_id(&state.config, &headers)?;

    match store::delete(&state.db, comment_id, user_id).await? {
        Some(comment) => {
            notify_admin(
                &state.config,
                comment_message(
                    &state.config.frontend_url,
                    "Delete Comment",
                    &classification,
                    &category,
                    &slug,
                    &comment.username,
                    None,
                ),
            );
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(ApiError::NotFound),
    }
}

/// PATCH /comments/{classification}/{category}/{slug}/{comment_id}/admin
/// Admin-only (ADMIN_EMAILS allowlist). Request: { reply }. Sets the reply.
/// Response: 200 CommentResponse, 401, 403 non-admin, 404, 400 empty.
pub async fn update_reply(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_classification, _category, _slug, comment_id)): Path<(String, String, String, Uuid)>,
    Json(req): Json<AdminReplyRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    auth::require_admin(&state.config, &headers)?;

    if req.reply.trim().is_empty() {
        return Err(ApiError::BadRequest("reply is required".into()));
    }

    let comment = store::upsert_reply(&state.db, comment_id, &req.reply)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(comment))
}

/// DELETE /comments/{classification}/{category}/{slug}/{comment_id}/admin
/// Admin-only. Clears the reply. Response: 204, 401, 403 non-admin, 404.
pub async fn delete_reply(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_classification, _category, _slug, comment_id)): Path<(String, String, String, Uuid)>,
) -> Result<StatusCode, ApiError> {
    auth::require_admin(&state.config, &headers)?;

    if store::delete_reply(&state.db, comment_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}
