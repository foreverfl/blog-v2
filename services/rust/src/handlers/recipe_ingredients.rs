use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use uuid::Uuid;

use crate::auth;
use crate::config::AppState;
use crate::stores::recipe_ingredients as store;
use crate::types::{ApiError, CreateIngredientRequest, Ingredient, UpdateIngredientRequest};

// POST /recipe/ingredients
//
// Request: Authorization: Bearer <API_SECRET>, JSON { slug, name_ko, name_ja, name_en, category? }
// Response: 201 with the created ingredient (generated id). 401 missing/bad secret,
//           409 duplicate slug, 400 invalid slug format.
pub async fn create_ingredient(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateIngredientRequest>,
) -> Result<(StatusCode, Json<Ingredient>), ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;
    let ingredient = store::create(&state.db, &req).await?;
    Ok((StatusCode::CREATED, Json(ingredient)))
}

// GET /recipe/ingredients/{id}
//
// Request: path id (uuid). Read-only, no auth.
// Response: 200 with the ingredient. 400 malformed uuid, 404 unknown id.
pub async fn get_ingredient(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Ingredient>, ApiError> {
    let ingredient = store::get(&state.db, id).await?;
    Ok(Json(ingredient))
}

// PATCH /recipe/ingredients/{id}
//
// Request: Authorization: Bearer <API_SECRET>, path id (uuid),
//          JSON with any subset of { slug, name_ko, name_ja, name_en, category }.
// Response: 200 with the updated ingredient. 401 missing/bad secret, 404 unknown id,
//           409 duplicate slug, 400 invalid slug format.
pub async fn update_ingredient(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateIngredientRequest>,
) -> Result<Json<Ingredient>, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;
    let ingredient = store::update(&state.db, id, &req).await?;
    Ok(Json(ingredient))
}

// DELETE /recipe/ingredients/{id}
//
// Request: Authorization: Bearer <API_SECRET>, path id (uuid).
// Response: 204 No Content on success. 401 missing/bad secret, 404 unknown id.
pub async fn delete_ingredient(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;
    store::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
