use chrono::{DateTime, Utc};
use serde::Serialize;

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
