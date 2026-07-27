use sqlx::PgPool;

use crate::types::{ApiError, Cuisine};

#[tracing::instrument(name = "recipe_cuisines.list", skip(pool))]
pub async fn list(pool: &PgPool) -> Result<Vec<Cuisine>, ApiError> {
    let cuisines = sqlx::query_as::<_, Cuisine>(
        r#"
        SELECT code, name_ko, name_ja, name_en
        FROM recipe.cuisines
        ORDER BY code
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(cuisines)
}
