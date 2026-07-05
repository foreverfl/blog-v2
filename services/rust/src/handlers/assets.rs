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
use crate::types::{
    ApiError, AssetResponse, ListAssetsQuery, ListAssetsResponse, ListBucketsResponse,
    UpdateAssetRequest, UploadQuery,
};

// GET /assets
//
// Request: Authorization: Bearer <API_SECRET or user JWT>,
//          query ?bucket= (logical name) &page= &per_page=.
// Response: 200 { items: [asset], total, page, per_page }. 401 missing/bad token.
pub async fn list_assets(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ListAssetsQuery>,
) -> Result<Json<ListAssetsResponse>, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let bucket = query
        .bucket
        .as_deref()
        .map(|logical| state.config.physical_bucket(logical));

    let (rows, total) = asset_store::list(&state.db, bucket.as_deref(), page, per_page).await?;
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

// GET /assets/buckets
//
// Request: Authorization: Bearer <API_SECRET or user JWT>.
// Response: 200 { buckets: [logical names for this env], default }.
//           401 missing/bad token.
// ponytail: hits R2 ListBuckets on every call — cache only if it gets slow.
pub async fn list_buckets(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ListBucketsResponse>, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let output = state
        .s3
        .list_buckets()
        .send()
        .await
        .map_err(|e| ApiError::S3(e.to_string()))?;

    let buckets = output
        .buckets()
        .iter()
        .filter_map(|bucket| bucket.name())
        .filter_map(|name| state.config.logical_bucket(name))
        .map(String::from)
        .collect();
    let default_bucket = &state.config.s3_bucket_blog_posts_assets;
    let default = state
        .config
        .logical_bucket(default_bucket)
        .unwrap_or(default_bucket)
        .to_string();

    Ok(Json(ListBucketsResponse { buckets, default }))
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

// PATCH /assets/{id}
//
// Request: Authorization: Bearer <API_SECRET or user JWT>, path id (uuid),
//          JSON with any subset of { file_name, status }. object_key and the
//          stored bytes are immutable.
// Response: 200 with the updated asset. 400 malformed uuid/body,
//           401 missing/bad token, 404 unknown id.
pub async fn update_asset(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAssetRequest>,
) -> Result<Json<AssetResponse>, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let row = asset_store::update(&state.db, id, req.file_name.as_deref(), req.status.as_deref())
        .await?
        .ok_or(ApiError::NotFound)?;
    let base = state.config.upload_base_url.as_deref();

    Ok(Json(AssetResponse::with_base(&row, base)))
}

// DELETE /assets/{id}
//
// Request: Authorization: Bearer <API_SECRET or user JWT>, path id (uuid).
// Response: 204 No Content. 400 malformed uuid, 401 missing/bad token,
//           404 unknown id.
pub async fn delete_asset(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let row = asset_store::get_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    // External object first, DB row second: a failed S3 delete keeps the DB
    // record intact, while the reverse would leave an untracked R2 object.
    // ponytail: sha256-dedup'd assets may still be referenced by posts —
    // deletion is unconditional until the orphan-detection spike adds a guard.
    state
        .s3
        .delete_object()
        .bucket(&row.bucket)
        .key(&row.object_key)
        .send()
        .await
        .map_err(|e| ApiError::S3(e.to_string()))?;

    asset_store::delete(&state.db, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Every object (key, size) in a bucket, following ListObjectsV2
/// continuation tokens until the listing is exhausted. Sync needs the full
/// inventory — stopping at one page would misread absent-from-page as
/// absent-from-bucket. page_size is 1000 in production; tests shrink it to
/// force the pagination path.
async fn collect_bucket_inventory(
    s3: &aws_sdk_s3::Client,
    bucket: &str,
    page_size: i32,
) -> Result<Vec<(String, i64)>, ApiError> {
    let mut inventory = Vec::new();
    let mut continuation_token: Option<String> = None;

    loop {
        let output = s3
            .list_objects_v2()
            .bucket(bucket)
            .max_keys(page_size)
            .set_continuation_token(continuation_token.take())
            .send()
            .await
            .map_err(|e| ApiError::S3(e.to_string()))?;

        for object in output.contents() {
            if let Some(key) = object.key() {
                inventory.push((key.to_string(), object.size().unwrap_or(0)));
            }
        }

        match output.next_continuation_token() {
            Some(token) => continuation_token = Some(token.to_string()),
            None => break,
        }
    }

    Ok(inventory)
}

/// Resolve the upload target: logical `?bucket=` → physical name verified
/// against the live R2 bucket list; the default bucket when omitted.
async fn resolve_upload_bucket(
    state: &AppState,
    logical: Option<&str>,
) -> Result<String, ApiError> {
    let Some(logical) = logical else {
        return Ok(state.config.s3_bucket_blog_posts_assets.clone());
    };

    let physical = state.config.physical_bucket(logical);
    let output = state
        .s3
        .list_buckets()
        .send()
        .await
        .map_err(|e| ApiError::S3(e.to_string()))?;
    let exists = output
        .buckets()
        .iter()
        .filter_map(|bucket| bucket.name())
        .any(|name| name == physical);

    if exists {
        Ok(physical)
    } else {
        Err(ApiError::BadRequest(format!("unknown bucket '{logical}'")))
    }
}

// POST /assets
//
// Request: Authorization: Bearer <API_SECRET or user JWT>,
//          optional ?bucket= (logical name, default bucket when omitted),
//          multipart/form-data with one or more `file` fields.
// Response: 201 array of assets (SHA-256 deduplicated). 400 bad multipart,
//           file over the size limit, or unknown bucket. 401 missing/bad token.
pub async fn upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<UploadQuery>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let bucket = resolve_upload_bucket(&state, query.bucket.as_deref()).await?;
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
            .bucket(&bucket)
            .key(&object_key)
            .body(ByteStream::from(data))
            .content_type(&mime_type)
            .send()
            .await
            .map_err(|e| ApiError::S3(e.to_string()))?;

        // Save to DB
        let row = asset_store::insert(
            &state.db,
            &bucket,
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

#[cfg(test)]
mod tests {
    use super::collect_bucket_inventory;

    // Integration check against real R2 — needs AWS_* and S3_ENDPOINT in the
    // env, so it is #[ignore]d in normal runs:
    //   cargo test collects_full_inventory -- --ignored
    #[tokio::test]
    #[ignore]
    async fn collects_full_inventory() {
        let endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT required");
        let bucket =
            std::env::var("S3_BUCKET_BLOG_POSTS_ASSETS").expect("bucket env required");
        let aws_config =
            aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let s3_config = aws_sdk_s3::config::Builder::from(&aws_config)
            .endpoint_url(endpoint)
            .force_path_style(true)
            .build();
        let s3 = aws_sdk_s3::Client::from_conf(s3_config);

        // page_size=1 forces one continuation round-trip per object; the
        // result must match the single-page run exactly, order included.
        let paged = collect_bucket_inventory(&s3, &bucket, 1).await.unwrap();
        let single = collect_bucket_inventory(&s3, &bucket, 1000).await.unwrap();

        assert!(!single.is_empty(), "dev bucket should not be empty");
        assert_eq!(paged, single);
    }
}
