use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, CommentResponse};

const SELECT_COLUMNS: &str =
    "c.id, u.email, u.username, u.photo, c.content, c.created_at, c.reply, c.replied_at";

/// All comments for a post, oldest first, joined with their authors.
#[tracing::instrument(name = "comments.list_for_post", skip(pool))]
pub async fn list_for_post(
    pool: &PgPool,
    post_id: Uuid,
) -> Result<Vec<CommentResponse>, ApiError> {
    let comments = sqlx::query_as::<_, CommentResponse>(&format!(
        r#"
        SELECT {SELECT_COLUMNS}
        FROM comments c
        JOIN users u ON c.user_id = u.id
        WHERE c.post_id = $1
        ORDER BY c.created_at ASC
        "#
    ))
    .bind(post_id)
    .fetch_all(pool)
    .await?;

    Ok(comments)
}

/// Insert a comment (snapshotting the author's avatar) and return it joined
/// with the author, matching the list-item shape.
#[tracing::instrument(name = "comments.create", skip(pool, content))]
pub async fn create(
    pool: &PgPool,
    post_id: Uuid,
    user_id: Uuid,
    content: &str,
) -> Result<CommentResponse, ApiError> {
    let comment = sqlx::query_as::<_, CommentResponse>(&format!(
        r#"
        WITH new_comment AS (
            INSERT INTO comments (post_id, user_id, photo, content)
            VALUES ($1, $2, (SELECT photo FROM users WHERE id = $2), $3)
            RETURNING id, user_id, content, created_at, reply, replied_at
        )
        SELECT {SELECT_COLUMNS}
        FROM new_comment c
        JOIN users u ON c.user_id = u.id
        "#
    ))
    .bind(post_id)
    .bind(user_id)
    .bind(content)
    .fetch_one(pool)
    .await?;

    Ok(comment)
}

/// Update a comment the user owns. Returns `None` when the comment does not
/// exist or belongs to someone else (indistinguishable on purpose).
#[tracing::instrument(name = "comments.update", skip(pool, content))]
pub async fn update(
    pool: &PgPool,
    comment_id: Uuid,
    user_id: Uuid,
    content: &str,
) -> Result<Option<CommentResponse>, ApiError> {
    let comment = sqlx::query_as::<_, CommentResponse>(&format!(
        r#"
        WITH updated AS (
            UPDATE comments
            SET content = $3, updated_at = now()
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, content, created_at, reply, replied_at
        )
        SELECT {SELECT_COLUMNS}
        FROM updated c
        JOIN users u ON c.user_id = u.id
        "#
    ))
    .bind(comment_id)
    .bind(user_id)
    .bind(content)
    .fetch_optional(pool)
    .await?;

    Ok(comment)
}

/// Delete a comment the user owns. Returns the deleted comment (so the caller
/// can notify), or `None` when it's missing or owned by someone else.
#[tracing::instrument(name = "comments.delete", skip(pool))]
pub async fn delete(
    pool: &PgPool,
    comment_id: Uuid,
    user_id: Uuid,
) -> Result<Option<CommentResponse>, ApiError> {
    let comment = sqlx::query_as::<_, CommentResponse>(&format!(
        r#"
        WITH deleted AS (
            DELETE FROM comments
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, content, created_at, reply, replied_at
        )
        SELECT {SELECT_COLUMNS}
        FROM deleted c
        JOIN users u ON c.user_id = u.id
        "#
    ))
    .bind(comment_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(comment)
}

/// Set (or replace) the admin reply on any comment. Admin-only — no ownership
/// check. Returns the updated comment, or `None` if the comment is missing.
#[tracing::instrument(name = "comments.upsert_reply", skip(pool, reply))]
pub async fn upsert_reply(
    pool: &PgPool,
    comment_id: Uuid,
    reply: &str,
) -> Result<Option<CommentResponse>, ApiError> {
    let comment = sqlx::query_as::<_, CommentResponse>(&format!(
        r#"
        WITH updated AS (
            UPDATE comments
            SET reply = $2, replied_at = now()
            WHERE id = $1
            RETURNING id, user_id, content, created_at, reply, replied_at
        )
        SELECT {SELECT_COLUMNS}
        FROM updated c
        JOIN users u ON c.user_id = u.id
        "#
    ))
    .bind(comment_id)
    .bind(reply)
    .fetch_optional(pool)
    .await?;

    Ok(comment)
}

/// Clear the admin reply on a comment. Returns whether a comment was updated.
#[tracing::instrument(name = "comments.delete_reply", skip(pool))]
pub async fn delete_reply(pool: &PgPool, comment_id: Uuid) -> Result<bool, ApiError> {
    let result = sqlx::query(
        r#"
        UPDATE comments
        SET reply = NULL, replied_at = NULL
        WHERE id = $1
        "#,
    )
    .bind(comment_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
