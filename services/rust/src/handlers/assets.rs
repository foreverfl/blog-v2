use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::Json;

use crate::auth;
use crate::config::AppState;
use crate::stores::assets as asset_store;
use crate::types::{ApiError, AssetResponse, ListAssetsQuery, ListAssetsResponse};

// GET /assets
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
