use sqlx::PgPool;

use crate::types::{ApiError, SauceUsageType};

#[tracing::instrument(name = "recipe_sauce_usage_types.list", skip(pool))]
pub async fn list(pool: &PgPool) -> Result<Vec<SauceUsageType>, ApiError> {
    let types = sqlx::query_as::<_, SauceUsageType>(
        r#"
        SELECT code, name_ko, name_ja, name_en
        FROM recipe.sauce_usage_types
        ORDER BY code
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(types)
}
