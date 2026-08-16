use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, DietProfile};

/// Fetch one user's profile. Returns NotFound when they have not made one yet.
/// The numeric columns are cast to float8 so sqlx maps them without pulling in
/// a decimal crate — one tenth of a kilogram survives the trip fine.
#[tracing::instrument(name = "diet_profiles.get", skip(pool))]
pub async fn get(pool: &PgPool, user_id: Uuid) -> Result<DietProfile, ApiError> {
    sqlx::query_as::<_, DietProfile>(
        r#"
        SELECT height_cm::float8              AS height_cm,
               target_weight_kg::float8       AS target_weight_kg,
               bmr_kcal,
               updated_at
        FROM public.diet_profiles
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)
}
