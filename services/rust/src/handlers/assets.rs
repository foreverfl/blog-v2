use aws_sdk_s3::primitives::ByteStream;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use sha2::{Digest, Sha256};
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

// POST /assets (also mounted at /uploads until the editor migrates)
//
// Request: Authorization: Bearer <API_SECRET or user JWT>,
//          multipart/form-data with one or more `file` fields.
// Response: 201 array of assets (SHA-256 deduplicated). 400 bad multipart or
//           file over the size limit, 401 missing/bad token.
pub async fn upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let mut assets: Vec<AssetResponse> = Vec::new();
    let base = state.config.upload_base_url.as_deref();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?
    {
        let file_name = field
            .file_name()
            .unwrap_or("unnamed")
            .to_string();
        let mime_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let data = field
            .bytes()
            .await
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        if data.len() > state.config.max_upload_size {
            return Err(ApiError::BadRequest(format!(
                "file '{}' exceeds max upload size of {} bytes",
                file_name, state.config.max_upload_size
            )));
        }

        let sha256 = {
            let mut hasher = Sha256::new();
            hasher.update(&data);
            hex::encode(hasher.finalize())
        };

        // Deduplicate by SHA-256
        if let Some(existing) = asset_store::find_by_sha256(&state.db, &sha256).await? {
            assets.push(AssetResponse::with_base(&existing, base));
            continue;
        }

        let kind = kind_from_mime(&mime_type);
        let data_len = data.len() as i64;
        let ext = file_name
            .rsplit('.')
            .next()
            .unwrap_or("bin");
        let object_key = format!("{}.{}", Uuid::new_v4(), ext);

        // Upload to S3
        state
            .s3
            .put_object()
            .bucket(&state.config.s3_bucket_blog_posts_assets)
            .key(&object_key)
            .body(ByteStream::from(data))
            .content_type(&mime_type)
            .send()
            .await
            .map_err(|e| ApiError::S3(e.to_string()))?;

        // Save to DB
        let row = asset_store::insert(
            &state.db,
            &state.config.s3_bucket_blog_posts_assets,
            &object_key,
            &file_name,
            &mime_type,
            data_len,
            &sha256,
            &kind,
        )
        .await?;

        assets.push(AssetResponse::with_base(&row, base));
    }

    Ok((StatusCode::CREATED, Json(assets)))
}

fn kind_from_mime(mime: &str) -> String {
    if mime.starts_with("image/") {
        "image".into()
    } else if mime.starts_with("video/") {
        "video".into()
    } else if mime.starts_with("audio/") {
        "audio".into()
    } else {
        "document".into()
    }
}
