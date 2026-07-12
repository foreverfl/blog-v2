use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, CommentResponse};

const SELECT_COLUMNS: &str =
    "c.id, u.email, u.username, u.photo, c.content, c.created_at, c.reply, c.replied_at";

/// All comments for a post, oldest first, joined with their authors.
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
#[allow(dead_code)] // wired into PATCH /comments/{id}
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

/// Delete a comment the user owns. Returns whether a row was removed.
#[allow(dead_code)] // wired into DELETE /comments/{id}
pub async fn delete(pool: &PgPool, comment_id: Uuid, user_id: Uuid) -> Result<bool, ApiError> {
    let result = sqlx::query(
        r#"
        DELETE FROM comments
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(comment_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
