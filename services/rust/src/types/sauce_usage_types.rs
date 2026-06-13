use serde::Serialize;

/// A sauce usage type (recipe.sauce_usage_types).
///
/// `code` is an ad-hoc slug (e.g. 'dip', 'marinade', 'glaze').
/// Doubles as the DB row and the API response.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SauceUsageType {
    pub code: String,
    pub name_ko: String,
    pub name_ja: String,
    pub name_en: String,
}
