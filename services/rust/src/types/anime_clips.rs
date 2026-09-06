use chrono::{DateTime, Utc};
use serde::Serialize;

// ── Database rows ──

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ClipRow {
    pub id: i64,
    pub r2_key: String,
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
}

// ── Query types ──

#[derive(Debug, serde::Deserialize)]
pub struct ListClipsQuery {
    pub viewed: Option<bool>,
    pub limit: Option<i64>,
}
