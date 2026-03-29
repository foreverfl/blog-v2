use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::assets::PostAssetResponse;
use super::contents::{ContentPayload, ContentResponse};

// ── Database rows ──

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostRow {
    pub id: Uuid,
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub indexed: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostSummaryRow {
    pub id: Uuid,
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub title: Option<String>,
}

// ── Request types ──

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub image: Option<String>,
    #[serde(default)]
    pub contents: Vec<ContentPayload>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub classification: Option<String>,
    pub category: Option<String>,
    pub slug: Option<String>,
    pub image: Option<String>,
    #[serde(default)]
    pub contents: Vec<ContentPayload>,
}

#[derive(Debug, Deserialize)]
pub struct MarkIndexedRequest {
    pub ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct LangQuery {
    pub lang: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    pub lang: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ── Response types ──

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub classification: String,
    pub category: String,
    pub slug: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub indexed: bool,
    pub contents: Vec<ContentResponse>,
    pub assets: Vec<PostAssetResponse>,
}

impl From<&PostRow> for PostResponse {
    fn from(row: &PostRow) -> Self {
        Self {
            id: row.id,
            classification: row.classification.clone(),
            category: row.category.clone(),
            slug: row.slug.clone(),
            image: row.image.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
            indexed: row.indexed,
            contents: vec![],
            assets: vec![],
        }
    }
}

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
            image: row.image.clone(),
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