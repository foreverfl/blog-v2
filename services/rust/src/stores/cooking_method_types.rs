use sqlx::PgPool;

use crate::types::{ApiError, CookingMethodType};

pub async fn list(pool: &PgPool) -> Result<Vec<CookingMethodType>, ApiError> {
    let types = sqlx::query_as::<_, CookingMethodType>(
        r#"
        SELECT code, name_ko, name_ja, name_en
        FROM recipe.cooking_method_types
        ORDER BY code
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(types)
}
