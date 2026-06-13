use axum::http::{header, HeaderMap};
use jsonwebtoken::{decode, DecodingKey, Validation};
use subtle::ConstantTimeEq;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::types::{ApiError, Claims};

/// Verify an `Authorization: Bearer <secret>` header against a shared secret.
/// Constant-time comparison to avoid timing leaks. Returns InvalidToken on mismatch.
pub fn verify_bearer_secret(headers: &HeaderMap, secret: &str) -> Result<(), ApiError> {
    let bearer = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .unwrap_or("");

    if bearer.as_bytes().ct_eq(secret.as_bytes()).unwrap_u8() == 1 {
        Ok(())
    } else {
        Err(ApiError::InvalidToken)
    }
}

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
