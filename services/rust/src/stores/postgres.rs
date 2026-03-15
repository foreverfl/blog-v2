use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{
    ApiError, AssetRow, ContentPayload, CreatePostRequest, PostAssetRow, PostContentRow, PostRow,
    PostSummaryRow, UpdatePostRequest,
};

// ── Posts ──

pub async fn create_post(
    pool: &PgPool,
    req: &CreatePostRequest,
) -> Result<PostRow, ApiError> {
    let mut tx = pool.begin().await?;

    let post = sqlx::query_as::<_, PostRow>(
        r#"
        INSERT INTO posts (classification, category, slug, body)
        VALUES ($1, $2, $3, $4)
        RETURNING id, classification, category, slug, body, created_at, updated_at, indexed
        "#,
    )
    .bind(&req.classification)
    .bind(&req.category)
    .bind(&req.slug)
    .bind(&req.body)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            ApiError::Conflict("post with this classification/category/slug already exists".into())
        }
        other => ApiError::Database(other),
    })?;

    for content in &req.contents {
        insert_content(&mut *tx, post.id, content).await?;
    }

    tx.commit().await?;
    Ok(post)
}

pub async fn list_posts(
    pool: &PgPool,
    lang: Option<&str>,
    classification: Option<&str>,
    category: Option<&str>,
    page: i64,
    per_page: i64,
) -> Result<(Vec<PostSummaryRow>, i64), ApiError> {
    let offset = (page - 1) * per_page;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM posts p
        WHERE ($1::text IS NULL OR p.classification = $1)
          AND ($2::text IS NULL OR p.category = $2)
        "#,
    )
    .bind(classification)
    .bind(category)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, PostSummaryRow>(
        r#"
        SELECT p.id, p.classification, p.category, p.slug, p.body, p.created_at,
               pc.title
        FROM posts p
        LEFT JOIN post_contents pc ON pc.post_id = p.id AND ($1::text IS NULL OR pc.lang = $1)
        WHERE ($2::text IS NULL OR p.classification = $2)
          AND ($3::text IS NULL OR p.category = $3)
        ORDER BY p.created_at DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(lang)
    .bind(classification)
    .bind(category)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((rows, total.0))
}

pub async fn get_post(pool: &PgPool, id: Uuid) -> Result<Option<PostRow>, ApiError> {
    let row = sqlx::query_as::<_, PostRow>(
        "SELECT id, classification, category, slug, body, created_at, updated_at, indexed FROM posts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn update_post(
    pool: &PgPool,
    id: Uuid,
    req: &UpdatePostRequest,
) -> Result<PostRow, ApiError> {
    let mut tx = pool.begin().await?;

    let post = sqlx::query_as::<_, PostRow>(
        r#"
        UPDATE posts SET
            classification = COALESCE($2, classification),
            category = COALESCE($3, category),
            slug = COALESCE($4, slug),
            body = COALESCE($5, body),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, classification, category, slug, body, created_at, updated_at, indexed
        "#,
    )
    .bind(id)
    .bind(&req.classification)
    .bind(&req.category)
    .bind(&req.slug)
    .bind(&req.body)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            ApiError::Conflict("post with this classification/category/slug already exists".into())
        }
        other => ApiError::Database(other),
    })?
    .ok_or(ApiError::NotFound)?;

    for content in &req.contents {
        upsert_content(&mut *tx, id, content).await?;
    }

    tx.commit().await?;
    Ok(post)
}

pub async fn delete_post(pool: &PgPool, id: Uuid) -> Result<bool, ApiError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

// ── Post contents ──

async fn insert_content(
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

async fn upsert_content(
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

pub async fn get_contents(
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

// ── Sync ──

pub async fn upsert_post(
    pool: &PgPool,
    classification: &str,
    category: &str,
    slug: &str,
) -> Result<PostRow, ApiError> {
    let row = sqlx::query_as::<_, PostRow>(
        r#"
        INSERT INTO posts (classification, category, slug)
        VALUES ($1, $2, $3)
        ON CONFLICT (classification, category, slug) DO UPDATE SET
            updated_at = CURRENT_TIMESTAMP
        RETURNING id, classification, category, slug, body, created_at, updated_at, indexed
        "#,
    )
    .bind(classification)
    .bind(category)
    .bind(slug)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn upsert_sync_content(
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

// ── Assets ──

pub async fn find_asset_by_sha256(
    pool: &PgPool,
    sha256: &str,
) -> Result<Option<AssetRow>, ApiError> {
    let row = sqlx::query_as::<_, AssetRow>(
        r#"
        SELECT id, bucket, object_key, file_name, mime_type, size_bytes, sha256, width, height, duration_ms, kind, status, metadata, created_at, updated_at
        FROM assets WHERE sha256 = $1
        "#,
    )
    .bind(sha256)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn insert_asset(
    pool: &PgPool,
    bucket: &str,
    object_key: &str,
    file_name: &str,
    mime_type: &str,
    size_bytes: i64,
    sha256: &str,
    kind: &str,
) -> Result<AssetRow, ApiError> {
    let row = sqlx::query_as::<_, AssetRow>(
        r#"
        INSERT INTO assets (bucket, object_key, file_name, mime_type, size_bytes, sha256, kind)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (sha256) WHERE sha256 IS NOT NULL DO UPDATE SET
            updated_at = CURRENT_TIMESTAMP
        RETURNING id, bucket, object_key, file_name, mime_type, size_bytes, sha256, width, height, duration_ms, kind, status, metadata, created_at, updated_at
        "#,
    )
    .bind(bucket)
    .bind(object_key)
    .bind(file_name)
    .bind(mime_type)
    .bind(size_bytes)
    .bind(sha256)
    .bind(kind)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_post_assets(
    pool: &PgPool,
    post_id: Uuid,
) -> Result<Vec<(PostAssetRow, AssetRow)>, ApiError> {
    let rows = sqlx::query_as::<_, PostAssetRow>(
        r#"
        SELECT id, post_id, asset_id, lang, role, sort_order, created_at
        FROM post_assets WHERE post_id = $1 ORDER BY sort_order
        "#,
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::with_capacity(rows.len());
    for pa in rows {
        let asset = sqlx::query_as::<_, AssetRow>(
            r#"
            SELECT id, bucket, object_key, file_name, mime_type, size_bytes, sha256, width, height, duration_ms, kind, status, metadata, created_at, updated_at
            FROM assets WHERE id = $1
            "#,
        )
        .bind(pa.asset_id)
        .fetch_one(pool)
        .await?;
        result.push((pa, asset));
    }

    Ok(result)
}
