use aws_sdk_s3::primitives::ByteStream;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use std::collections::HashSet;

use crate::auth;
use crate::config::AppState;
use crate::stores::assets as asset_store;
use crate::types::{
    ApiError, AssetResponse, ListAssetsQuery, ListAssetsResponse, ListBucketsResponse,
    SyncAssetsResponse, SyncQuery, UpdateAssetRequest, UploadQuery,
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
    let items = rows
        .iter()
        .map(|row| AssetResponse::from(row))
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
// Hits R2 ListBuckets on every call — cache only if it gets slow.
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

    Ok(Json(AssetResponse::from(&row)))
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

    Ok(Json(AssetResponse::from(&row)))
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
    // sha256-dedup'd assets may still be referenced by posts — deletion is
    // unconditional; stale references are handled manually via the manager.
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

/// Every object (key, size) in a bucket, following ListObjectsV2 continuation
/// tokens. page_size is 1000 in production; tests shrink it to force paging.
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

/// Resolve a logical `?bucket=` to its physical name, verified against the
/// live R2 bucket list; the default bucket when omitted.
async fn resolve_bucket(
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

    let bucket = resolve_bucket(&state, query.bucket.as_deref()).await?;
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

        if let Some(existing) = asset_store::find_by_sha256(&state.db, &sha256).await? {
            assets.push(AssetResponse::from(&existing));
            continue;
        }

        let kind = kind_from_mime(&mime_type);
        let data_len = data.len() as i64;
        let ext = file_name
            .rsplit('.')
            .next()
            .unwrap_or("bin");
        let object_key = format!("{}.{}", Uuid::new_v4(), ext);

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

        assets.push(AssetResponse::from(&row));
    }

    Ok((StatusCode::CREATED, Json(assets)))
}

// POST /assets/sync
//
// Request: Authorization: Bearer <API_SECRET or user JWT>,
//          query ?bucket= (logical name, default bucket when omitted).
// Response: 200 { bucket, inserted, deleted }.
//           400 unknown bucket, 401 missing/bad token.
//
// R2 is the source of truth: objects missing from the DB are inserted
// (sha256 NULL — excluded from upload dedup), rows whose object is gone are
// deleted. Idempotent — a second run reports {inserted: 0, deleted: 0}.
pub async fn sync_assets(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SyncQuery>,
) -> Result<Json<SyncAssetsResponse>, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    let bucket = resolve_bucket(&state, query.bucket.as_deref()).await?;

    let inventory = collect_bucket_inventory(&state.s3, &bucket, 1000).await?;
    let db_keys: HashSet<String> = asset_store::list_object_keys(&state.db, &bucket)
        .await?
        .into_iter()
        .collect();
    let r2_keys: HashSet<&str> = inventory.iter().map(|(key, _)| key.as_str()).collect();

    let to_insert: Vec<&(String, i64)> = inventory
        .iter()
        .filter(|(key, _)| !db_keys.contains(key))
        .collect();
    let to_delete: Vec<String> = db_keys
        .iter()
        .filter(|key| !r2_keys.contains(key.as_str()))
        .cloned()
        .collect();

    for (object_key, size_bytes) in &to_insert {
        let mime_type = mime_from_extension(object_key);
        let kind = kind_from_mime(&mime_type);
        asset_store::insert_synced(
            &state.db,
            &bucket,
            object_key,
            object_key,
            &mime_type,
            *size_bytes,
            &kind,
        )
        .await?;
    }
    let deleted = asset_store::delete_missing(&state.db, &bucket, &to_delete).await?;

    Ok(Json(SyncAssetsResponse {
        bucket,
        inserted: to_insert.len(),
        deleted: deleted as usize,
    }))
}

/// Extension sniffing only — ListObjectsV2 carries no content-type and a
/// HEAD request per object is not worth it for sync.
fn mime_from_extension(key: &str) -> String {
    let extension = key.rsplit('.').next().map(|ext| ext.to_ascii_lowercase());
    match extension.as_deref() {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("mp3") => "audio/mpeg",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
    .into()
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
