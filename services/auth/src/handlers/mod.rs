use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json, Redirect};

use crate::config::AppState;
use crate::providers::Provider;
use crate::services;
use crate::stores::{postgres as pg, redis as redis_store};
use crate::types::{AuthError, CallbackQuery, TokenResponse, UserDto};

// GET /auth/login/:provider
pub async fn login(
    State(state): State<AppState>,
    Path(provider_name): Path<String>,
) -> Result<Redirect, AuthError> {
    let provider = Provider::from_name(&provider_name)?;
    let oauth_state = services::generate_state();

    redis_store::store_oauth_state(
        &mut state.redis.clone(),
        &oauth_state,
        &provider_name,
        600, // 10 min
    )
    .await?;

    let url = provider.auth_url(&state.config, &oauth_state)?;
    Ok(Redirect::temporary(&url))
}

// GET /auth/callback/:provider
pub async fn callback(
    State(state): State<AppState>,
    Path(provider_name): Path<String>,
    Query(query): Query<CallbackQuery>,
) -> Result<impl IntoResponse, AuthError> {
    // Verify CSRF state
    let stored_provider = redis_store::get_and_delete_oauth_state(
        &mut state.redis.clone(),
        &query.state,
    )
    .await?
    .ok_or(AuthError::InvalidState)?;

    if stored_provider != provider_name {
        return Err(AuthError::InvalidState);
    }

    let provider = Provider::from_name(&provider_name)?;

    // Exchange code for user info
    let user_info = provider
        .authenticate(&state.config, &state.http, &query.code)
        .await?;

    // Upsert user in Postgres
    let user = pg::upsert_user(&state.db, &user_info).await?;

    // Generate tokens
    let access_token = services::create_access_token(&state.config, user.id, &user.email)?;
    let refresh_token = services::create_refresh_token(
        &mut state.redis.clone(),
        user.id,
        state.config.refresh_token_ttl,
    )
    .await?;

    // Set refresh token as HTTP-only cookie, redirect to frontend with access token
    let cookie = format!(
        "refresh_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
        refresh_token, state.config.refresh_token_ttl,
    );
    let redirect_url = format!(
        "{}/auth/callback?access_token={}&expires_in={}",
        state.config.frontend_url, access_token, state.config.access_token_ttl,
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());
    headers.insert(header::LOCATION, redirect_url.parse().unwrap());

    Ok((StatusCode::TEMPORARY_REDIRECT, headers))
}

// POST /auth/refresh
pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<TokenResponse>, AuthError> {
    let refresh_token =
        extract_cookie(&headers, "refresh_token").ok_or(AuthError::InvalidToken)?;

    let user_id =
        services::validate_refresh_token(&mut state.redis.clone(), &refresh_token).await?;

    let user = pg::find_user_by_id(&state.db, user_id)
        .await?
        .ok_or(AuthError::InvalidToken)?;

    let access_token = services::create_access_token(&state.config, user.id, &user.email)?;

    Ok(Json(TokenResponse {
        access_token,
        token_type: "Bearer".into(),
        expires_in: state.config.access_token_ttl,
    }))
}

// POST /auth/logout
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AuthError> {
    if let Some(refresh_token) = extract_cookie(&headers, "refresh_token") {
        services::revoke_refresh_token(&mut state.redis.clone(), &refresh_token).await?;
    }

    let clear_cookie = "refresh_token=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax";
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, clear_cookie.parse().unwrap());

    Ok((
        StatusCode::OK,
        resp_headers,
        Json(serde_json::json!({"message": "logged out"})),
    ))
}

// GET /auth/me
pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserDto>, AuthError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AuthError::InvalidToken)?;

    let claims = services::validate_access_token(&state.config, token)?;

    let user = pg::find_user_by_id(&state.db, claims.sub)
        .await?
        .ok_or(AuthError::InvalidToken)?;

    Ok(Json(user.into()))
}

fn extract_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|s| s.split(';'))
        .find_map(|pair| {
            let mut parts = pair.trim().splitn(2, '=');
            let key = parts.next()?.trim();
            let val = parts.next()?.trim();
            if key == name {
                Some(val.to_string())
            } else {
                None
            }
        })
}