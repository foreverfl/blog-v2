use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── JWT ──

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

// ── Database rows ──

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostRow {
    pub id: Uuid,
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub body: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub indexed: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostContentRow {
    pub id: Uuid,
    pub post_id: Uuid,
    pub lang: String,
    pub content_type: String,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub body_markdown: Option<String>,
    pub body_json: Option<serde_json::Value>,
    pub body_text: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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

// ── Request types ──

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub body: Option<String>,
    #[serde(default)]
    pub contents: Vec<ContentPayload>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub classification: Option<String>,
    pub category: Option<String>,
    pub slug: Option<String>,
    pub body: Option<String>,
    #[serde(default)]
    pub contents: Vec<ContentPayload>,
}

#[derive(Debug, Deserialize)]
pub struct ContentPayload {
    pub lang: String,
    pub content_type: String,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub body_markdown: Option<String>,
    pub body_json: Option<serde_json::Value>,
    pub body_text: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct LangQuery {
    pub lang: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    pub lang: Option<String>,
    pub classification: Option<String>,
    pub category: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ── Lightweight row for list queries ──

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostSummaryRow {
    pub id: Uuid,
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub body: Option<String>,
    pub created_at: NaiveDateTime,
    pub title: Option<String>,
}

// ── Response types ──

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub body: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub indexed: bool,
    pub contents: Vec<ContentResponse>,
    pub assets: Vec<PostAssetResponse>,
}

#[derive(Debug, Serialize)]
pub struct ContentResponse {
    pub id: Uuid,
    pub lang: String,
    pub content_type: String,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub body_markdown: Option<String>,
    pub body_json: Option<serde_json::Value>,
    pub body_text: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

impl From<&PostRow> for PostResponse {
    fn from(row: &PostRow) -> Self {
        Self {
            id: row.id,
            classification: row.classification.clone(),
            category: row.category.clone(),
            slug: row.slug.clone(),
            body: row.body.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
            indexed: row.indexed,
            contents: vec![],
            assets: vec![],
        }
    }
}

impl From<&PostContentRow> for ContentResponse {
    fn from(row: &PostContentRow) -> Self {
        Self {
            id: row.id,
            lang: row.lang.clone(),
            content_type: row.content_type.clone(),
            title: row.title.clone(),
            excerpt: row.excerpt.clone(),
            body_markdown: row.body_markdown.clone(),
            body_json: row.body_json.clone(),
            body_text: row.body_text.clone(),
            metadata: row.metadata.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
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

// ── List response ──

#[derive(Debug, Serialize)]
pub struct PostSummaryResponse {
    pub id: Uuid,
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub title: Option<String>,
}

impl From<&PostSummaryRow> for PostSummaryResponse {
    fn from(row: &PostSummaryRow) -> Self {
        Self {
            id: row.id,
            classification: row.classification.clone(),
            category: row.category.clone(),
            slug: row.slug.clone(),
            image: row.body.clone(),
            created_at: row.created_at,
            title: row.title.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListPostsResponse {
    pub posts: Vec<PostSummaryResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ── Errors ──

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("invalid token")]
    InvalidToken,

    #[error("expired token")]
    ExpiredToken,

    #[error("not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("s3 error: {0}")]
    S3(String),

    #[error("internal error: {0}")]
    #[allow(dead_code)]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::InvalidToken | ApiError::ExpiredToken => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::Database(e) => {
                tracing::error!("database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
            ApiError::S3(e) => {
                tracing::error!("s3 error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
            ApiError::Internal(e) => {
                tracing::error!("internal error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
        };
        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}
