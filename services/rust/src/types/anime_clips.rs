use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::config::ASSET_URL_PATTERN;

// ── Database rows ──

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ClipRow {
    pub id: i64,
    pub r2_key: Option<String>, // NULL once cleanup removed the R2 object
    pub series_slug: String,
    pub episode: String,
    pub start_sec: f32,
    pub duration_sec: f32,
    pub jellyfin_item: Option<String>,
    pub is_opening: bool,
    pub liked: bool,
    pub view_count: i32,
    pub last_viewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    // Not a column — filled by with_url() before the row goes out.
    #[sqlx(default)]
    pub url: Option<String>,
}

impl ClipRow {
    /// Fill `url` from r2_key and the physical clips bucket.
    ///
    /// @param bucket - physical bucket name (e.g. "dev-anime-clips")
    /// @return self with url set, or url None when the media was cleared.
    pub fn with_url(mut self, bucket: &str) -> Self {
        self.url = self.r2_key.as_ref().map(|key| {
            format!("{}/{}", ASSET_URL_PATTERN.replace("{bucket}", bucket), key)
        });
        self
    }
}

// ── Query types ──

#[derive(Debug, serde::Deserialize)]
pub struct ListClipsQuery {
    pub viewed: Option<bool>,
    pub limit: Option<i64>,
}
