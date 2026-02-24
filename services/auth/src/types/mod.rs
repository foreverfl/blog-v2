use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

// Database row
#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub auth_provider: String,
    pub username: String,
    pub photo: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

// API response
#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub photo: Option<String>,
    pub auth_provider: String,
}

impl From<UserRow> for UserDto {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            username: row.username,
            photo: row.photo,
            auth_provider: row.auth_provider,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

// OAuth user info from provider
#[derive(Debug)]
pub struct OAuthUserInfo {
    pub email: String,
    pub name: String,
    pub photo: Option<String>,
    pub provider: String,
}

// Errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("unsupported provider: {0}")]
    UnsupportedProvider(String),

    #[error("provider not configured: {0}")]
    ProviderNotConfigured(String),

    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("invalid token")]
    InvalidToken,

    #[error("expired token")]
    ExpiredToken,

    #[error("invalid OAuth state")]
    InvalidState,

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("internal error")]
    Internal(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AuthError::UnsupportedProvider(_) | AuthError::ProviderNotConfigured(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            AuthError::OAuth(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AuthError::InvalidToken | AuthError::ExpiredToken | AuthError::InvalidState => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            AuthError::Database(e) => {
                tracing::error!("database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
            AuthError::Redis(e) => {
                tracing::error!("redis error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
            AuthError::Internal(e) => {
                tracing::error!("internal error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
        };
        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}