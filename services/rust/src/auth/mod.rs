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

/// Authorize via the shared `API_SECRET` Bearer token or a user JWT.
pub fn verify_secret_or_user(config: &AppConfig, headers: &HeaderMap) -> Result<(), ApiError> {
    if verify_bearer_secret(headers, &config.api_secret).is_ok() {
        return Ok(());
    }
    extract_user_id(config, headers).map(|_| ())
}

fn decode_claims(config: &AppConfig, headers: &HeaderMap) -> Result<Claims, ApiError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::InvalidToken)?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::ExpiredToken,
        _ => ApiError::InvalidToken,
    })
}

pub fn extract_user_id(config: &AppConfig, headers: &HeaderMap) -> Result<Uuid, ApiError> {
    Ok(decode_claims(config, headers)?.sub)
}

/// Require the caller to be an admin: a valid JWT whose email is in the
/// `ADMIN_EMAILS` allowlist. 401 on a bad token, 403 on a non-admin user.
pub fn require_admin(config: &AppConfig, headers: &HeaderMap) -> Result<(), ApiError> {
    let claims = decode_claims(config, headers)?;
    if config.admin_emails.iter().any(|email| email == &claims.email) {
        Ok(())
    } else {
        Err(ApiError::Forbidden("admin only".into()))
    }
}
