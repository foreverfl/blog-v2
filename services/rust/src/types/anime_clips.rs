use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::config::AppState;
use crate::services::signed_url;
use crate::types::ApiError;

// ── Database rows ──

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ClipRow {
    pub id: i64,
    pub r2_key: Option<String>, // NULL once cleanup removed the R2 object
    pub series_slug: String,
    pub series_title: Option<String>, // Jellyfin's name for the series, NULL until filled
    pub episode: String,
    pub start_sec: f32,
    pub duration_sec: f32,
    pub jellyfin_item: Option<String>,
    pub is_opening: bool,
    pub liked: bool,
    pub liked_at: Option<DateTime<Utc>>, // NULL until liked, back to NULL on unlike
    pub view_count: i32,
    pub last_viewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    // Not columns — filled by with_url() before the row goes out.
    #[sqlx(default)]
    pub url: Option<String>,
    #[sqlx(default)]
    pub thumbnail_url: Option<String>,
}

/// A clip's thumbnail key: the video's own key with the extension swapped
/// to .webp, so the pair always sits in the same feed//liked/ folder.
///
/// @return e.g. "feed/slug-s01e01-13.mp4" -> "feed/slug-s01e01-13.webp"
pub fn thumbnail_key(r2_key: &str) -> String {
    match r2_key.rsplit_once('.') {
        Some((stem, _)) => format!("{stem}.webp"),
        None => format!("{r2_key}.webp"),
    }
}

impl ClipRow {
    /// Fill `url` and `thumbnail_url` with temporary signed links to the
    /// clip's R2 objects (the bucket is private; signing is offline crypto).
    ///
    /// The thumbnail link is signed without checking the object exists — a
    /// clip from before thumbnails 404s there until the backfill runs.
    ///
    /// @param state - carries the S3 client and the clips bucket name
    /// @return self with both urls set, or both None when the media was cleared
    pub async fn with_url(mut self, state: &AppState) -> Result<Self, ApiError> {
        let bucket = &state.config.s3_bucket_anime_clips;
        (self.url, self.thumbnail_url) = match self.r2_key.as_deref() {
            Some(key) => (
                Some(signed_url::get_object(&state.s3, bucket, key).await?),
                Some(signed_url::get_object(&state.s3, bucket, &thumbnail_key(key)).await?),
            ),
            None => (None, None),
        };
        Ok(self)
    }
}

// ── Query types ──

#[derive(Debug, serde::Deserialize)]
pub struct ListClipsQuery {
    pub viewed: Option<bool>,
    pub liked: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PatchClipBody {
    pub jellyfin_item: Option<String>,
    pub series_title: Option<String>,
}

// What the player saw when playback went wrong. Every field but `event` is
// optional — a browser that cannot read one still sends the rest.
#[derive(Debug, serde::Deserialize)]
pub struct PlaybackEventBody {
    pub event: String, // stall | recovered | error | muted
    pub buffer_left_sec: Option<f64>,
    pub downlink: Option<f64>,
    pub effective_type: Option<String>,
    pub stall_ms: Option<i64>,
    pub error_code: Option<i32>,
    pub url_age_sec: Option<i64>,
    pub muted: Option<bool>,
    pub volume: Option<f64>,
    pub session_id: Option<String>,
}
