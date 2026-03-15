use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::auth;
use crate::config::AppState;
use crate::stores::postgres as pg;
use crate::types::{
    ApiError, ContentResponse, CreatePostRequest, LangQuery, ListPostsQuery, ListPostsResponse,
    PostAssetResponse, PostResponse, PostSummaryResponse, UpdatePostRequest,
};

// GET /api/posts
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<ListPostsResponse>, ApiError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let (rows, total) = pg::list_posts(
        &state.db,
        query.lang.as_deref(),
        query.classification.as_deref(),
        query.category.as_deref(),
        page,
        per_page,
    )
    .await?;

    Ok(Json(ListPostsResponse {
        posts: rows.iter().map(PostSummaryResponse::from).collect(),
        total,
        page,
        per_page,
    }))
}

// POST /api/posts
pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, ApiError> {
    auth::extract_user_id(&state.config, &headers)?;

    if req.classification.is_empty() || req.category.is_empty() || req.slug.is_empty() {
        return Err(ApiError::BadRequest(
            "classification, category, and slug are required".into(),
        ));
    }

    let post = pg::create_post(&state.db, &req).await?;
    let contents = pg::get_contents(&state.db, post.id, None).await?;

    let mut resp = PostResponse::from(&post);
    resp.contents = contents.iter().map(ContentResponse::from).collect();

    Ok((StatusCode::CREATED, Json(resp)))
}

// GET /api/posts/:id
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(query): Query<LangQuery>,
) -> Result<Json<PostResponse>, ApiError> {
    let post = pg::get_post(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let contents = pg::get_contents(&state.db, post.id, query.lang.as_deref()).await?;
    let post_assets = pg::get_post_assets(&state.db, post.id).await?;

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
            asset: asset.into(),
        })
        .collect();

    Ok(Json(resp))
}

// PUT /api/posts/:id
pub async fn update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>, ApiError> {
    auth::extract_user_id(&state.config, &headers)?;

    let post = pg::update_post(&state.db, id, &req).await?;
    let contents = pg::get_contents(&state.db, post.id, None).await?;

    let mut resp = PostResponse::from(&post);
    resp.contents = contents.iter().map(ContentResponse::from).collect();

    Ok(Json(resp))
}

// DELETE /api/posts/:id
pub async fn delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    auth::extract_user_id(&state.config, &headers)?;

    let deleted = pg::delete_post(&state.db, id).await?;
    if !deleted {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
