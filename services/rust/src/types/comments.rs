use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// A comment joined with its author, as returned to the frontend.
/// `photo`/`username`/`email` come from the user row; `photo` is the author's
/// current avatar (not the snapshot stored on the comment).
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CommentResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub photo: Option<String>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub reply: Option<String>,
    pub replied_at: Option<DateTime<Utc>>,
}
