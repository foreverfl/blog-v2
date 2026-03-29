use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{
    ApiError, CreatePostRequest, PostRow, PostSummaryRow, UpdatePostRequest,
};

pub async fn create(
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
        super::contents::insert(&mut *tx, post.id, content).await?;
    }

    tx.commit().await?;
    Ok(post)
}

pub async fn list(
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

pub async fn list_excluding_classification(
    pool: &PgPool,
    lang: Option<&str>,
    exclude_classification: &str,
    page: i64,
    per_page: i64,
) -> Result<(Vec<PostSummaryRow>, i64), ApiError> {
    let offset = (page - 1) * per_page;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM posts p
        WHERE p.classification != $1
        "#,
    )
    .bind(exclude_classification)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, PostSummaryRow>(
        r#"
        SELECT p.id, p.classification, p.category, p.slug, p.body, p.created_at,
               pc.title
        FROM posts p
        LEFT JOIN post_contents pc ON pc.post_id = p.id AND ($1::text IS NULL OR pc.lang = $1)
        WHERE p.classification != $2
        ORDER BY p.created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(lang)
    .bind(exclude_classification)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((rows, total.0))
}

pub async fn get_by_slug(
    pool: &PgPool,
    classification: &str,
    category: &str,
    slug: &str,
) -> Result<Option<PostRow>, ApiError> {
    let row = sqlx::query_as::<_, PostRow>(
        r#"
        SELECT id, classification, category, slug, body, created_at, updated_at, indexed
        FROM posts
        WHERE classification = $1 AND category = $2 AND slug = $3
        LIMIT 1
        "#,
    )
    .bind(classification)
    .bind(category)
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_unindexed(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<(Vec<PostSummaryRow>, i64), ApiError> {
    let offset = (page - 1) * per_page;

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM posts WHERE indexed = false")
            .fetch_one(pool)
            .await?;

    let rows = sqlx::query_as::<_, PostSummaryRow>(
        r#"
        SELECT p.id, p.classification, p.category, p.slug, p.body, p.created_at,
               NULL::text AS title
        FROM posts p
        WHERE p.indexed = false
        ORDER BY p.created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((rows, total.0))
}

pub async fn mark_indexed(
    pool: &PgPool,
    ids: &[Uuid],
) -> Result<Vec<PostRow>, ApiError> {
    let rows = sqlx::query_as::<_, PostRow>(
        r#"
        UPDATE posts
        SET indexed = true, updated_at = CURRENT_TIMESTAMP
        WHERE id = ANY($1)
        RETURNING id, classification, category, slug, body, created_at, updated_at, indexed
        "#,
    )
    .bind(ids)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn update(
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
        super::contents::upsert(&mut *tx, id, content).await?;
    }

    tx.commit().await?;
    Ok(post)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, ApiError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn upsert(
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
