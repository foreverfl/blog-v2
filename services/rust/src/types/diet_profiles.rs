use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user's body profile (public.diet_profiles). One row per user, and only the
/// inputs — BMI and the rest are derived on read, never stored.
/// The numeric columns arrive as float8; see the store's SELECT.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct DietProfile {
    pub height_cm: f64,
    pub target_weight_kg: Option<f64>,
    pub bmr_kcal: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

/// PUT /diet/profile request body. The whole profile is sent every time, so an
/// omitted optional field clears the stored value rather than keeping it.
#[derive(Debug, Deserialize)]
pub struct UpsertDietProfileRequest {
    pub height_cm: f64,
    pub target_weight_kg: Option<f64>,
    pub bmr_kcal: Option<i32>,
}

/// GET /diet/profile/stats query. `weight` previews a value the user is still
/// typing; without it the latest recorded weight is used.
#[derive(Debug, Deserialize)]
pub struct DietProfileStatsQuery {
    pub weight: Option<f64>,
}

/// GET /diet/profile/stats response. Everything here is computed on read from
/// the profile plus one weight — none of it is stored.
#[derive(Debug, Serialize)]
pub struct DietProfileStats {
    pub weight_kg: f64,
    pub bmi: f64,
    pub target_weight_kg: Option<f64>,
    pub remaining_kg: Option<f64>,
    pub remaining_kcal: Option<f64>,
    pub walk_hours: Option<f64>,
    pub walk_km: Option<f64>,
    pub run_hours: Option<f64>,
    pub run_km: Option<f64>,
}
