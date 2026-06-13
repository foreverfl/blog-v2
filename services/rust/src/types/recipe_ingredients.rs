use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An ingredient (recipe.ingredients). Doubles as the DB row and the API response.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Ingredient {
    pub id: Uuid,
    pub slug: String,
    pub name_ko: String,
    pub name_ja: String,
    pub name_en: String,
    pub category: Option<String>,
}

/// POST /recipe/ingredients request body.
#[derive(Debug, Deserialize)]
pub struct CreateIngredientRequest {
    pub slug: String,
    pub name_ko: String,
    pub name_ja: String,
    pub name_en: String,
    pub category: Option<String>,
}

/// PATCH /recipe/ingredients/{id} request body. Omitted fields stay unchanged
/// (each binds NULL and the query keeps the existing value via COALESCE).
#[derive(Debug, Deserialize)]
pub struct UpdateIngredientRequest {
    pub slug: Option<String>,
    pub name_ko: Option<String>,
    pub name_ja: Option<String>,
    pub name_en: Option<String>,
    pub category: Option<String>,
}
