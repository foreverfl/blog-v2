use sqlx::PgPool;
use uuid::Uuid;

use crate::stores::{assets as asset_store, contents as content_store, posts as post_store};
use crate::types::{
    ApiError, AssetResponse, ContentResponse, CreatePostRequest, ListPostsResponse,
    PostAssetResponse, PostResponse, PostSummaryResponse, UpdatePostRequest,
};

pub async fn list(
    pool: &PgPool,
    lang: Option<&str>,
    classification: Option<&str>,
    category: Option<&str>,
    page: i64,
    per_page: i64,
) -> Result<ListPostsResponse, ApiError> {
    let (rows, total) =
        post_store::list(pool, lang, classification, category, page, per_page).await?;

    Ok(ListPostsResponse {
        posts: rows.iter().map(PostSummaryResponse::from).collect(),
        total,
        page,
        per_page,
    })
}

pub async fn get_by_slug(
    pool: &PgPool,
    classification: &str,
    category: &str,
    slug: &str,
    lang: Option<&str>,
) -> Result<PostResponse, ApiError> {
    let post = post_store::get_by_slug(pool, classification, category, slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    let contents = content_store::get_by_post(pool, post.id, lang).await?;
    let post_assets = asset_store::get_post_assets(pool, post.id).await?;

    let mut resp = PostResponse::from(&post);
    resp.contents = contents.iter().map(ContentResponse::from).collect();
    resp.assets = post_assets
        .iter()
        .map(|(pa, asset)| PostAssetResponse {
            id: pa.id,
            asset_id: pa.asset_id,
            lang: pa.lang.clone(),
            role: pa.role.clone(),
            sort_order: pa.sort_order,
            asset: AssetResponse::from(asset),
        })
        .collect();

    Ok(resp)
}

pub async fn create(
    pool: &PgPool,
    req: &CreatePostRequest,
) -> Result<PostResponse, ApiError> {
    let post = post_store::create(pool, req).await?;
    let contents = content_store::get_by_post(pool, post.id, None).await?;

    let mut resp = PostResponse::from(&post);
    resp.contents = contents.iter().map(ContentResponse::from).collect();
    Ok(resp)
}

pub async fn update_by_slug(
    pool: &PgPool,
    classification: &str,
    category: &str,
    slug: &str,
    req: &UpdatePostRequest,
) -> Result<PostResponse, ApiError> {
    let post = post_store::get_by_slug(pool, classification, category, slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    let updated = post_store::update(pool, post.id, req).await?;
    let contents = content_store::get_by_post(pool, updated.id, None).await?;

    let mut resp = PostResponse::from(&updated);
    resp.contents = contents.iter().map(ContentResponse::from).collect();
    Ok(resp)
}

pub async fn delete_by_slug(
    pool: &PgPool,
    classification: &str,
    category: &str,
    slug: &str,
) -> Result<bool, ApiError> {
    let post = post_store::get_by_slug(pool, classification, category, slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    post_store::delete(pool, post.id).await
}

pub async fn list_unindexed(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<ListPostsResponse, ApiError> {
    let (rows, total) = post_store::list_unindexed(pool, page, per_page).await?;

    Ok(ListPostsResponse {
        posts: rows.iter().map(PostSummaryResponse::from).collect(),
        total,
        page,
        per_page,
    })
}

pub async fn mark_indexed(
    pool: &PgPool,
    ids: &[Uuid],
) -> Result<Vec<PostResponse>, ApiError> {
    let posts = post_store::mark_indexed(pool, ids).await?;
    Ok(posts.iter().map(PostResponse::from).collect())
}
