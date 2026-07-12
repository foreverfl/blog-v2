use sqlx::PgPool;
use uuid::Uuid;

use crate::types::ApiError;

/// Number of likes on a post.
pub async fn count(pool: &PgPool, post_id: Uuid) -> Result<i64, ApiError> {
    let (count,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM likes
        WHERE post_id = $1
        "#,
    )
    .bind(post_id)
    .fetch_one(pool)
    .await?;

    Ok(count)
}

/// Whether a user has liked a post.
pub async fn has_liked(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<bool, ApiError> {
    let (liked,): (bool,) = sqlx::query_as(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM likes WHERE post_id = $1 AND user_id = $2
        )
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(liked)
}

/// Add a like (idempotent — a repeat like is a no-op).
pub async fn add(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO likes (post_id, user_id)
        VALUES ($1, $2)
        ON CONFLICT (post_id, user_id) DO NOTHING
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Remove a like.
pub async fn remove(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        DELETE FROM likes
        WHERE post_id = $1 AND user_id = $2
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
