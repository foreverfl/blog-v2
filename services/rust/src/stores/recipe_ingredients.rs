use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, CreateIngredientRequest, Ingredient};

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
