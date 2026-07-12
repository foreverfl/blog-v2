use serde::Serialize;

/// Like count for a post plus whether the current user has liked it.
#[derive(Debug, Serialize)]
pub struct LikeStatus {
    pub like_count: i64,
    pub liked: bool,
}
