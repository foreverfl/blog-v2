use aws_sdk_s3::primitives::ByteStream;
use axum::body::Bytes;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use tracing::Instrument;

use crate::auth;
use crate::config::AppState;
use crate::stores::anime_clips as clip_store;
use crate::types::{ApiError, ListClipsQuery};

// POST /anime/clips
//
// Request: Authorization: Bearer <API_SECRET>, multipart/form-data with a
//          `file` field (the clip video) and text fields: series_slug,
//          episode (SxxEyy), start_sec, duration_sec, and optionally
//          jellyfin_item and is_opening ("true"/"false").
// Response: 201 with the stored clip row. Re-posting the same clip overwrites
//           the R2 object and refreshes the row (idempotent).
//           400 missing/unknown field or bad multipart, 401 missing/bad token.
pub async fn upload_clip(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;

    let mut file: Option<(String, String, Bytes)> = None; // (file_name, mime, data)
    let mut series_slug: Option<String> = None;
    let mut episode: Option<String> = None;
    let mut start_sec: Option<f32> = None;
    let mut duration_sec: Option<f32> = None;
    let mut jellyfin_item: Option<String> = None;
    let mut is_opening = false;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let file_name = field.file_name().unwrap_or("clip.mp4").to_string();
            let mime = field.content_type().unwrap_or("video/mp4").to_string();
            let data = field
                .bytes()
                .await
                .map_err(|e| ApiError::BadRequest(e.to_string()))?;
            file = Some((file_name, mime, data));
            continue;
        }
        let text = field
            .text()
            .await
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;
        match name.as_str() {
            "series_slug" => series_slug = Some(text),
            "episode" => episode = Some(text),
            "start_sec" => start_sec = text.parse().ok(),
            "duration_sec" => duration_sec = text.parse().ok(),
            "jellyfin_item" => jellyfin_item = Some(text),
            "is_opening" => is_opening = text == "true",
            _ => return Err(ApiError::BadRequest(format!("unknown field '{name}'"))),
        }
    }

    let (file_name, mime, data) = file.ok_or_else(|| missing("file"))?;
    let series_slug = series_slug.ok_or_else(|| missing("series_slug"))?;
    let episode = episode.ok_or_else(|| missing("episode"))?;
    let start_sec = start_sec.ok_or_else(|| missing("start_sec"))?;
    let duration_sec = duration_sec.ok_or_else(|| missing("duration_sec"))?;

    if data.len() > state.config.max_upload_size {
        return Err(ApiError::BadRequest(format!(
            "file '{}' exceeds max upload size of {} bytes",
            file_name, state.config.max_upload_size
        )));
    }

    let ext = file_name.rsplit('.').next().unwrap_or("mp4");
    // Deterministic key: re-cutting the same spot lands on the same object.
    let r2_key = format!(
        "feed/{}-{}-{}.{}",
        series_slug,
        episode.to_lowercase(),
        start_sec.round() as u32,
        ext
    );

    state
        .s3
        .put_object()
        .bucket(&state.config.s3_bucket_anime_clips)
        .key(&r2_key)
        .body(ByteStream::from(data))
        .content_type(&mime)
        .send()
        .instrument(tracing::info_span!("s3.put_object"))
        .await
        .map_err(|e| ApiError::S3(e.to_string()))?;

    let row = clip_store::upsert(
        &state.db,
        &r2_key,
        &series_slug,
        &episode,
        start_sec,
        duration_sec,
        jellyfin_item.as_deref(),
        is_opening,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(row)))
}

fn missing(field: &str) -> ApiError {
    ApiError::BadRequest(format!("missing field '{field}'"))
}

// GET /anime/clips
//
// Request: Authorization: Bearer <API_SECRET>, optional query
//          ?viewed=false (only unviewed — view_count = 0; true = only viewed)
//          &limit= (default 100, max 1000).
// Response: 200 array of clip rows in random order.
//           401 missing/bad token.
pub async fn list_clips(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ListClipsQuery>,
) -> Result<Json<Vec<crate::types::ClipRow>>, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;

    let limit = query.limit.unwrap_or(100).clamp(1, 1000);
    let rows = clip_store::list(&state.db, query.viewed, limit).await?;

    Ok(Json(rows))
}

// POST /anime/clips/{id}/view
//
// Request: Authorization: Bearer <API_SECRET>, path id (bigint).
// Response: 200 with the updated clip row (view_count + 1, last_viewed_at set).
//           400 non-numeric id, 401 missing/bad token, 404 unknown id.
pub async fn view_clip(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<Json<crate::types::ClipRow>, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;

    let row = clip_store::record_view(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(row))
}

// POST /anime/clips/{id}/like
//
// Request: Authorization: Bearer <API_SECRET>, path id (bigint).
// Response: 200 with the updated row — liked TRUE and the R2 object moved
//           feed/ → liked/ (r2_key updated). Already-liked is a no-op 200.
//           400 non-numeric id, 401 missing/bad token, 404 unknown id.
pub async fn like_clip(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<Json<crate::types::ClipRow>, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;
    Ok(Json(move_and_flag(&state, id, true).await?))
}

// DELETE /anime/clips/{id}/like
//
// Request: Authorization: Bearer <API_SECRET>, path id (bigint).
// Response: 200 with the updated row — liked FALSE and the R2 object moved
//           back liked/ → feed/. Not-liked is a no-op 200.
//           400 non-numeric id, 401 missing/bad token, 404 unknown id.
pub async fn unlike_clip(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<Json<crate::types::ClipRow>, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;
    Ok(Json(move_and_flag(&state, id, false).await?))
}

/// Flip the liked flag, moving the R2 object between feed/ and liked/
/// (copy + delete — R2 has no rename). No-op when already in the wanted state.
///
/// @return the updated (or unchanged) row.
async fn move_and_flag(
    state: &AppState,
    id: i64,
    liked: bool,
) -> Result<crate::types::ClipRow, ApiError> {
    let row = clip_store::get_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    if row.liked == liked {
        return Ok(row);
    }

    let Some(current_key) = row.r2_key.clone() else {
        return Err(ApiError::BadRequest("clip has no media".into()));
    };

    let (from_prefix, to_prefix) = if liked {
        ("feed/", "liked/")
    } else {
        ("liked/", "feed/")
    };
    let new_key = format!(
        "{}{}",
        to_prefix,
        current_key.strip_prefix(from_prefix).unwrap_or(&current_key)
    );

    let bucket = &state.config.s3_bucket_anime_clips;
    state
        .s3
        .copy_object()
        .bucket(bucket)
        .copy_source(format!("{bucket}/{current_key}"))
        .key(&new_key)
        .send()
        .instrument(tracing::info_span!("s3.copy_object"))
        .await
        .map_err(|e| ApiError::S3(e.to_string()))?;
    state
        .s3
        .delete_object()
        .bucket(bucket)
        .key(&current_key)
        .send()
        .instrument(tracing::info_span!("s3.delete_object"))
        .await
        .map_err(|e| ApiError::S3(e.to_string()))?;

    clip_store::set_liked(&state.db, id, liked, &new_key).await
}

// DELETE /anime/clips/{id}/media
//
// Request: Authorization: Bearer <API_SECRET>, path id (bigint).
// Response: 200 with the updated row — the R2 object deleted, r2_key NULL,
//           the row itself kept (view history survives, the clip can be
//           re-cut). Already-cleared is a no-op 200.
//           400 liked clip (kept forever) or non-numeric id,
//           401 missing/bad token, 404 unknown id.
pub async fn clear_clip_media(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<Json<crate::types::ClipRow>, ApiError> {
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;

    let row = clip_store::get_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    if row.liked {
        return Err(ApiError::BadRequest("clip is liked - media is kept".into()));
    }
    let Some(key) = row.r2_key.clone() else {
        return Ok(Json(row));
    };

    state
        .s3
        .delete_object()
        .bucket(&state.config.s3_bucket_anime_clips)
        .key(&key)
        .send()
        .instrument(tracing::info_span!("s3.delete_object"))
        .await
        .map_err(|e| ApiError::S3(e.to_string()))?;

    Ok(Json(clip_store::clear_media(&state.db, id).await?))
}
