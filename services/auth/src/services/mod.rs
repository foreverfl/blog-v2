use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::RngExt;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::stores::redis as redis_store;
use crate::types::{AuthError, Claims};

pub fn create_access_token(
    config: &AppConfig,
    user_id: Uuid,
    email: &str,
) -> Result<String, AuthError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        iat: now,
        exp: now + config.access_token_ttl,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| AuthError::Internal(e.to_string()))
}

pub fn validate_access_token(config: &AppConfig, token: &str) -> Result<Claims, AuthError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
        _ => AuthError::InvalidToken,
    })
}

pub async fn create_refresh_token(
    redis: &mut redis_store::RedisConn,
    user_id: Uuid,
    ttl: u64,
) -> Result<String, AuthError> {
    let bytes: [u8; 32] = rand::rng().random();
    let token = hex::encode(bytes);
    let hash = hash_token(&token);
    redis_store::store_refresh_token(redis, &hash, user_id, ttl).await?;
    Ok(token)
}

pub async fn validate_refresh_token(
    redis: &mut redis_store::RedisConn,
    token: &str,
) -> Result<Uuid, AuthError> {
    let hash = hash_token(token);
    redis_store::get_refresh_token_user(redis, &hash)
        .await?
        .ok_or(AuthError::InvalidToken)
}

pub async fn revoke_refresh_token(
    redis: &mut redis_store::RedisConn,
    token: &str,
) -> Result<(), AuthError> {
    let hash = hash_token(token);
    redis_store::delete_refresh_token(redis, &hash).await
}

pub fn generate_state() -> String {
    let bytes: [u8; 16] = rand::rng().random();
    hex::encode(bytes)
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}