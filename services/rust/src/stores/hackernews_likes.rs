use sqlx::PgPool;
use uuid::Uuid;

use crate::types::ApiError;

pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<i64>, ApiError> {
    let rows: Vec<(i64,)> = sqlx::query_as(
        r#"
        SELECT hn_id
        FROM hackernews_likes
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn insert(pool: &PgPool, user_id: Uuid, hn_id: i64) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO hackernews_likes (user_id, hn_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, hn_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(hn_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, user_id: Uuid, hn_id: i64) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        DELETE FROM hackernews_likes
        WHERE user_id = $1 AND hn_id = $2
        "#,
    )
    .bind(user_id)
    .bind(hn_id)
    .execute(pool)
    .await?;

    Ok(())
}
