use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

// ── Database rows ──

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AssetRow {
    pub id: Uuid,
    pub bucket: String,
    pub object_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_ms: Option<i32>,
    pub kind: String,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostAssetRow {
    pub id: Uuid,
    pub post_id: Uuid,
    pub asset_id: Uuid,
    pub lang: Option<String>,
    pub role: String,
    pub sort_order: i32,
    pub created_at: NaiveDateTime,
}

// ── Response types ──

#[derive(Debug, Serialize)]
pub struct AssetResponse {
    pub id: Uuid,
    pub bucket: String,
    pub object_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_ms: Option<i32>,
    pub kind: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<&AssetRow> for AssetResponse {
    fn from(row: &AssetRow) -> Self {
        Self {
            id: row.id,
            bucket: row.bucket.clone(),
            object_key: row.object_key.clone(),
            file_name: row.file_name.clone(),
            mime_type: row.mime_type.clone(),
            size_bytes: row.size_bytes,
            sha256: row.sha256.clone(),
            width: row.width,
            height: row.height,
            duration_ms: row.duration_ms,
            kind: row.kind.clone(),
            status: row.status.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PostAssetResponse {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub lang: Option<String>,
    pub role: String,
    pub sort_order: i32,
    pub asset: AssetResponse,
}