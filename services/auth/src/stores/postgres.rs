use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{AuthError, OAuthUserInfo, UserRow};

pub async fn upsert_user(pool: &PgPool, info: &OAuthUserInfo) -> Result<UserRow, AuthError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (email, auth_provider, username, photo)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (email) DO UPDATE SET
            auth_provider = EXCLUDED.auth_provider,
            username = EXCLUDED.username,
            photo = EXCLUDED.photo
        RETURNING id, email, auth_provider, username, photo, created_at
        "#,
    )
    .bind(&info.email)
    .bind(&info.provider)
    .bind(&info.name)
    .bind(&info.photo)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserRow>, AuthError> {
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, email, auth_provider, username, photo, created_at FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}