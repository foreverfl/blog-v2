use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Database rows ──

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

// ── Request types ──

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

// ── Response types ──

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