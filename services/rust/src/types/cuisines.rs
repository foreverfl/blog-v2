use serde::Serialize;

/// A cuisine classification (recipe.cuisines).
///
/// `code` is an ISO 3166-1 alpha-2 country code for countries, or an ad-hoc
/// code for region/fusion extras. Doubles as the DB row and the API response.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Cuisine {
    pub code: String,
    pub name_ko: String,
    pub name_ja: String,
    pub name_en: String,
}
