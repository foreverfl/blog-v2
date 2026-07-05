use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use uuid::Uuid;

use crate::auth;
use crate::config::AppState;
use crate::stores::assets as asset_store;
use crate::types::{ApiError, AssetResponse, ListAssetsQuery, ListAssetsResponse};

// GET /assets
//
// Request: Authorization: Bearer <API_SECRET or user JWT>, query ?page= &per_page=.
// Response: 200 { items: [asset], total, page, per_page }. 401 missing/bad token.
pub async fn list_assets(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ListAssetsQuery>,
) -> Result<Json<ListAssetsResponse>, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let (rows, total) = asset_store::list(&state.db, page, per_page).await?;
    let base = state.config.upload_base_url.as_deref();
    let items = rows
        .iter()
        .map(|row| AssetResponse::with_base(row, base))
        .collect();

    Ok(Json(ListAssetsResponse {
        items,
        total,
        page,
        per_page,
    }))
}

// GET /assets/{id}
//
// Request: Authorization: Bearer <API_SECRET or user JWT>, path id (uuid).
// Response: 200 with the asset (url included). 400 malformed uuid,
//           401 missing/bad token, 404 unknown id.
pub async fn get_asset(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<AssetResponse>, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let row = asset_store::get_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    let base = state.config.upload_base_url.as_deref();

    Ok(Json(AssetResponse::with_base(&row, base)))
}
