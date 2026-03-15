use aws_sdk_s3::primitives::ByteStream;
use axum::extract::{Multipart, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::auth;
use crate::config::AppState;
use crate::stores::postgres as pg;
use crate::types::{ApiError, AssetResponse};

// POST /api/uploads
pub async fn upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    auth::extract_user_id(&state.config, &headers)?;

    let mut assets: Vec<AssetResponse> = Vec::new();

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
        if let Some(existing) = pg::find_asset_by_sha256(&state.db, &sha256).await? {
            assets.push(AssetResponse::from(&existing));
            continue;
        }

        let kind = kind_from_mime(&mime_type);
        let ext = file_name
            .rsplit('.')
            .next()
            .unwrap_or("bin");
        let object_key = format!("{}/{}.{}", state.config.s3_prefix, Uuid::new_v4(), ext);

        // Upload to S3
        state
            .s3
            .put_object()
            .bucket(&state.config.s3_bucket)
            .key(&object_key)
            .body(ByteStream::from(data.to_vec()))
            .content_type(&mime_type)
            .send()
            .await
            .map_err(|e| ApiError::S3(e.to_string()))?;

        let row = pg::insert_asset(
            &state.db,
            &state.config.s3_bucket,
            &object_key,
            &file_name,
            &mime_type,
            data.len() as i64,
            &sha256,
            &kind,
        )
        .await?;

        assets.push(AssetResponse::from(&row));
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
