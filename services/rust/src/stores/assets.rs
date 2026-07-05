use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, AssetRow, PostAssetRow};

pub async fn find_by_sha256(
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

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<AssetRow>, ApiError> {
    let row = sqlx::query_as::<_, AssetRow>(
        r#"
        SELECT id, bucket, object_key, file_name, mime_type, size_bytes, sha256, width, height, duration_ms, kind, status, metadata, created_at, updated_at
        FROM assets WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list(
    pool: &PgPool,
    bucket: Option<&str>,
    page: i64,
    per_page: i64,
) -> Result<(Vec<AssetRow>, i64), ApiError> {
    let offset = (page - 1) * per_page;

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM assets WHERE ($1::text IS NULL OR bucket = $1)")
            .bind(bucket)
            .fetch_one(pool)
            .await?;

    let rows = sqlx::query_as::<_, AssetRow>(
        r#"
        SELECT id, bucket, object_key, file_name, mime_type, size_bytes, sha256, width, height, duration_ms, kind, status, metadata, created_at, updated_at
        FROM assets
        WHERE ($1::text IS NULL OR bucket = $1)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(bucket)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((rows, total.0))
}

pub async fn insert(
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

/// Partial update: None keeps the current value via COALESCE.
/// Returns None when no row matched.
pub async fn update(
    pool: &PgPool,
    id: Uuid,
    file_name: Option<&str>,
    status: Option<&str>,
) -> Result<Option<AssetRow>, ApiError> {
    let row = sqlx::query_as::<_, AssetRow>(
        r#"
        UPDATE assets SET
            file_name = COALESCE($2, file_name),
            status = COALESCE($3, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, bucket, object_key, file_name, mime_type, size_bytes, sha256, width, height, duration_ms, kind, status, metadata, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(file_name)
    .bind(status)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), ApiError> {
    sqlx::query("DELETE FROM assets WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
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
