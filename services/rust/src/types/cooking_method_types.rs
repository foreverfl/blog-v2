use serde::Serialize;

/// A cooking method type (recipe.cooking_method_types).
///
/// `code` is an ad-hoc slug (e.g. 'grill', 'pan_fry', 'steam').
/// Doubles as the DB row and the API response.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CookingMethodType {
    pub code: String,
    pub name_ko: String,
    pub name_ja: String,
    pub name_en: String,
}
