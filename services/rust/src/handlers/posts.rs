use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;

use crate::auth;
use crate::config::AppState;
use crate::repositories::posts as repo;
use crate::types::{
    ApiError, CreatePostRequest, LangQuery, ListPostsQuery, ListPostsResponse, MarkIndexedRequest,
    PostResponse, UpdatePostRequest,
};

// GET /posts/recent — exclude trends
pub async fn list_posts(
    State(state): State<AppState>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<ListPostsResponse>, ApiError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let resp = repo::list_posts(&state.db, query.lang.as_deref(), page, per_page).await?;
    Ok(Json(resp))
}

// GET /posts/{classification}
pub async fn list_by_classification(
    State(state): State<AppState>,
    Path(classification): Path<String>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<ListPostsResponse>, ApiError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let resp = repo::list(
        &state.db,
        query.lang.as_deref(),
        Some(&classification),
        None,
        page,
        per_page,
    )
    .await?;
    Ok(Json(resp))
}

// GET /posts/{classification}/{category}
pub async fn list_by_category(
    State(state): State<AppState>,
    Path((classification, category)): Path<(String, String)>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<ListPostsResponse>, ApiError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let resp = repo::list(
        &state.db,
        query.lang.as_deref(),
        Some(&classification),
        Some(&category),
        page,
        per_page,
    )
    .await?;
    Ok(Json(resp))
}

// GET /posts/{classification}/{category}/{slug}
pub async fn get_by_slug(
    State(state): State<AppState>,
    Path((classification, category, slug)): Path<(String, String, String)>,
    Query(query): Query<LangQuery>,
) -> Result<Json<PostResponse>, ApiError> {
    let resp =
        repo::get_by_slug(&state.db, &classification, &category, &slug, query.lang.as_deref())
            .await?;
    Ok(Json(resp))
}

// POST /posts
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

    let resp = repo::create(&state.db, &req).await?;
    Ok((StatusCode::CREATED, Json(resp)))
}

// PUT /posts/{classification}/{category}/{slug}
pub async fn update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug)): Path<(String, String, String)>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>, ApiError> {
    auth::extract_user_id(&state.config, &headers)?;

    let resp = repo::update_by_slug(&state.db, &classification, &category, &slug, &req).await?;
    Ok(Json(resp))
}

// DELETE /posts/{classification}/{category}/{slug}
pub async fn delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((classification, category, slug)): Path<(String, String, String)>,
) -> Result<StatusCode, ApiError> {
    auth::extract_user_id(&state.config, &headers)?;

    let deleted = repo::delete_by_slug(&state.db, &classification, &category, &slug).await?;
    if !deleted {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

// GET /posts/unindexed
pub async fn list_unindexed(
    State(state): State<AppState>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<ListPostsResponse>, ApiError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let resp = repo::list_unindexed(&state.db, page, per_page).await?;
    Ok(Json(resp))
}

// PUT /posts/mark-indexed
pub async fn mark_indexed(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<MarkIndexedRequest>,
) -> Result<Json<Vec<PostResponse>>, ApiError> {
    auth::extract_user_id(&state.config, &headers)?;

    let resp = repo::mark_indexed(&state.db, &req.ids).await?;
    Ok(Json(resp))
}
