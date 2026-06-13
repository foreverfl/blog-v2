use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;

use crate::auth;
use crate::config::AppState;
use crate::stores::recipe_ingredients as store;
use crate::types::{ApiError, CreateIngredientRequest, Ingredient};

// POST /recipe/ingredients
//
// Request: Authorization: Bearer <IMPORT_SECRET>, JSON { slug, name_ko, name_ja, name_en, category? }
// Response: 201 with the created ingredient (generated id). 401 missing/bad secret,
//           409 duplicate slug, 400 invalid slug format.
pub async fn create_ingredient(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateIngredientRequest>,
) -> Result<(StatusCode, Json<Ingredient>), ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.import_secret)?;
    let ingredient = store::create(&state.db, &req).await?;
    Ok((StatusCode::CREATED, Json(ingredient)))
}
