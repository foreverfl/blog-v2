use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, ContentPayload, PostContentRow};

pub async fn insert(
    tx: &mut sqlx::PgConnection,
    post_id: Uuid,
    c: &ContentPayload,
) -> Result<(), ApiError> {
    let metadata = c
        .metadata
        .clone()
        .unwrap_or_else(|| serde_json::json!({}));

    sqlx::query(
        r#"
        INSERT INTO post_contents (post_id, lang, content_type, title, excerpt, body_markdown, body_json, body_text, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(post_id)
    .bind(&c.lang)
    .bind(&c.content_type)
    .bind(&c.title)
    .bind(&c.excerpt)
    .bind(&c.body_markdown)
    .bind(&c.body_json)
    .bind(&c.body_text)
    .bind(&metadata)
    .execute(tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            ApiError::Conflict("duplicate lang in contents".into())
        }
        other => ApiError::Database(other),
    })?;

    Ok(())
}

pub async fn upsert(
    tx: &mut sqlx::PgConnection,
    post_id: Uuid,
    c: &ContentPayload,
) -> Result<(), ApiError> {
    let metadata = c
        .metadata
        .clone()
        .unwrap_or_else(|| serde_json::json!({}));

    sqlx::query(
        r#"
        INSERT INTO post_contents (post_id, lang, content_type, title, excerpt, body_markdown, body_json, body_text, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (post_id, lang) DO UPDATE SET
            content_type = EXCLUDED.content_type,
            title = EXCLUDED.title,
            excerpt = EXCLUDED.excerpt,
            body_markdown = EXCLUDED.body_markdown,
            body_json = EXCLUDED.body_json,
            body_text = EXCLUDED.body_text,
            metadata = EXCLUDED.metadata,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(post_id)
    .bind(&c.lang)
    .bind(&c.content_type)
    .bind(&c.title)
    .bind(&c.excerpt)
    .bind(&c.body_markdown)
    .bind(&c.body_json)
    .bind(&c.body_text)
    .bind(&metadata)
    .execute(tx)
    .await?;

    Ok(())
}

pub async fn get_by_post(
    pool: &PgPool,
    post_id: Uuid,
    lang: Option<&str>,
) -> Result<Vec<PostContentRow>, ApiError> {
    let rows = match lang {
        Some(lang) => {
            sqlx::query_as::<_, PostContentRow>(
                r#"
                SELECT id, post_id, lang, content_type, title, excerpt, body_markdown, body_json, body_text, metadata, created_at, updated_at
                FROM post_contents WHERE post_id = $1 AND lang = $2
                "#,
            )
            .bind(post_id)
            .bind(lang)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, PostContentRow>(
                r#"
                SELECT id, post_id, lang, content_type, title, excerpt, body_markdown, body_json, body_text, metadata, created_at, updated_at
                FROM post_contents WHERE post_id = $1
                "#,
            )
            .bind(post_id)
            .fetch_all(pool)
            .await?
        }
    };

    Ok(rows)
}

pub async fn upsert_sync(
    pool: &PgPool,
    post_id: Uuid,
    lang: &str,
    content_type: &str,
    title: Option<&str>,
    body_markdown: &str,
    metadata: &serde_json::Value,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO post_contents (post_id, lang, content_type, title, body_markdown, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (post_id, lang) DO UPDATE SET
            content_type = EXCLUDED.content_type,
            title = EXCLUDED.title,
            body_markdown = EXCLUDED.body_markdown,
            metadata = EXCLUDED.metadata,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(post_id)
    .bind(lang)
    .bind(content_type)
    .bind(title)
    .bind(body_markdown)
    .bind(metadata)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn upsert_json(
    pool: &PgPool,
    post_id: Uuid,
    lang: &str,
    title: Option<&str>,
    excerpt: Option<&str>,
    body_text: Option<&str>,
    metadata: &serde_json::Value,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO post_contents (post_id, lang, content_type, title, excerpt, body_text, metadata)
        VALUES ($1, $2, 'json', $3, $4, $5, $6)
        ON CONFLICT (post_id, lang) DO UPDATE SET
            content_type = EXCLUDED.content_type,
            title = EXCLUDED.title,
            excerpt = EXCLUDED.excerpt,
            body_text = EXCLUDED.body_text,
            metadata = EXCLUDED.metadata,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(post_id)
    .bind(lang)
    .bind(title)
    .bind(excerpt)
    .bind(body_text)
    .bind(metadata)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn upsert_batch_json(
    pool: &PgPool,
    post_id: Uuid,
    lang: &str,
    body_json: &serde_json::Value,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO post_contents (post_id, lang, content_type, body_json)
        VALUES ($1, $2, 'json', $3)
        ON CONFLICT (post_id, lang) DO UPDATE SET
            content_type = EXCLUDED.content_type,
            body_json = EXCLUDED.body_json,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(post_id)
    .bind(lang)
    .bind(body_json)
    .execute(pool)
    .await?;

    Ok(())
}
