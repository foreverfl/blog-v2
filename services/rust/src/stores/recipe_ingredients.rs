use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, CreateIngredientRequest, Ingredient, UpdateIngredientRequest};

pub async fn create(
    pool: &PgPool,
    req: &CreateIngredientRequest,
) -> Result<Ingredient, ApiError> {
    sqlx::query_as::<_, Ingredient>(
        r#"
        INSERT INTO recipe.ingredients (slug, name_ko, name_ja, name_en, category)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, slug, name_ko, name_ja, name_en, category
        "#,
    )
    .bind(&req.slug)
    .bind(&req.name_ko)
    .bind(&req.name_ja)
    .bind(&req.name_en)
    .bind(&req.category)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            ApiError::Conflict(format!("ingredient with slug '{}' already exists", req.slug))
        }
        sqlx::Error::Database(ref db_err) if db_err.is_check_violation() => {
            ApiError::BadRequest(format!("invalid slug format: '{}'", req.slug))
        }
        other => ApiError::Database(other),
    })
}

/// Partially update an ingredient by id. Omitted (NULL) fields keep their
/// current value via COALESCE. Returns NotFound when no row matched.
pub async fn update(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateIngredientRequest,
) -> Result<Ingredient, ApiError> {
    sqlx::query_as::<_, Ingredient>(
        r#"
        UPDATE recipe.ingredients SET
            slug    = COALESCE($2, slug),
            name_ko = COALESCE($3, name_ko),
            name_ja = COALESCE($4, name_ja),
            name_en = COALESCE($5, name_en),
            category = COALESCE($6, category)
        WHERE id = $1
        RETURNING id, slug, name_ko, name_ja, name_en, category
        "#,
    )
    .bind(id)
    .bind(&req.slug)
    .bind(&req.name_ko)
    .bind(&req.name_ja)
    .bind(&req.name_en)
    .bind(&req.category)
    .fetch_optional(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            let slug = req.slug.as_deref().unwrap_or_default();
            ApiError::Conflict(format!("ingredient with slug '{slug}' already exists"))
        }
        sqlx::Error::Database(ref db_err) if db_err.is_check_violation() => {
            let slug = req.slug.as_deref().unwrap_or_default();
            ApiError::BadRequest(format!("invalid slug format: '{slug}'"))
        }
        other => ApiError::Database(other),
    })?
    .ok_or(ApiError::NotFound)
}

/// Delete an ingredient by id. Returns NotFound when no row matched.
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), ApiError> {
    let result = sqlx::query("DELETE FROM recipe.ingredients WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(())
}
