use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ApiError, DietProfile, UpsertDietProfileRequest};

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

/// Create or replace one user's profile. The body carries the whole profile, so
/// an omitted optional field clears the stored one.
#[tracing::instrument(name = "diet_profiles.upsert", skip(pool, req))]
pub async fn upsert(
    pool: &PgPool,
    user_id: Uuid,
    req: &UpsertDietProfileRequest,
) -> Result<DietProfile, ApiError> {
    sqlx::query_as::<_, DietProfile>(
        r#"
        INSERT INTO public.diet_profiles (user_id, height_cm, target_weight_kg, bmr_kcal)
        -- float8 in, numeric in the column: cast so the bind type is unambiguous
        VALUES ($1, $2::float8::numeric, $3::float8::numeric, $4)
        ON CONFLICT (user_id) DO UPDATE SET
            height_cm        = EXCLUDED.height_cm,
            target_weight_kg = EXCLUDED.target_weight_kg,
            bmr_kcal         = EXCLUDED.bmr_kcal,
            updated_at       = now()
        RETURNING height_cm::float8        AS height_cm,
                  target_weight_kg::float8 AS target_weight_kg,
                  bmr_kcal,
                  updated_at
        "#,
    )
    .bind(user_id)
    .bind(req.height_cm)
    .bind(req.target_weight_kg)
    .bind(req.bmr_kcal)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_check_violation() => {
            ApiError::BadRequest("height_cm and bmr_kcal must be greater than 0".into())
        }
        other => ApiError::Database(other),
    })
}

/// The most recent weight this user wrote down, if they ever have.
/// Days without a weight are skipped rather than treated as the latest.
#[tracing::instrument(name = "diet_profiles.latest_weight", skip(pool))]
pub async fn latest_weight(pool: &PgPool, user_id: Uuid) -> Result<Option<f64>, ApiError> {
    sqlx::query_scalar::<_, f64>(
        r#"
        SELECT weight_kg::float8
        FROM public.diet_daily_logs
        WHERE user_id = $1 AND weight_kg IS NOT NULL
        ORDER BY log_date DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(ApiError::from)
}
