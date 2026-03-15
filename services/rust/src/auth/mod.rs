use axum::http::{header, HeaderMap};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::types::{ApiError, Claims};

pub fn extract_user_id(config: &AppConfig, headers: &HeaderMap) -> Result<Uuid, ApiError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::InvalidToken)?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::ExpiredToken,
        _ => ApiError::InvalidToken,
    })?;

    Ok(claims.sub)
}
