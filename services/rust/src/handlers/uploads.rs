use aws_sdk_s3::primitives::ByteStream;
use axum::extract::{Multipart, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::auth;
use crate::config::{AppConfig, AppState};
use crate::stores::assets as asset_store;
use crate::types::{ApiError, AssetResponse};

/// Authorize an upload via `UPLOAD_SECRET` Bearer token or a user JWT.
/// Falls back to JWT-only when no secret is configured.
fn authorize_upload(config: &AppConfig, headers: &HeaderMap) -> Result<(), ApiError> {
    if let Some(secret) = config.upload_secret.as_deref() {
        let bearer = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "));
        if bearer == Some(secret) {
            return Ok(());
        }
    }
    auth::extract_user_id(config, headers).map(|_| ())
}

// POST /api/uploads
pub async fn upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    authorize_upload(&state.config, &headers)?;

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
