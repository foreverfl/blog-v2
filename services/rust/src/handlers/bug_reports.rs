use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use chrono::{FixedOffset, Utc};
use serde::Deserialize;

use crate::config::AppState;
use crate::services::{discord, turnstile};
use crate::types::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BugReportRequest {
    pub turnstile_token: String,
    pub title: String,
    pub content: String,
}

/// POST /bug-reports
/// Request:  { turnstileToken, title, content }
/// Response: 202 Accepted (Turnstile-verified and dispatched to Discord; nothing persisted)
pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<BugReportRequest>,
) -> Result<StatusCode, ApiError> {
    let client_ip = client_ip(&headers);

    let passed = turnstile::verify(
        &state.config.turnstile_secret,
        &req.turnstile_token,
        client_ip.as_deref(),
    )
    .await?;
    if !passed {
        return Err(ApiError::Forbidden("turnstile verification failed".into()));
    }

    let now_kst = Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    let ip = client_ip.as_deref().unwrap_or("unknown");
    let message = format!(
        "[{}] {ip}\n# Bug Report\n\n## Title\n{}\n\n## Content\n{}",
        now_kst.format("%Y-%m-%d %H:%M:%S"),
        req.title,
        req.content,
    );

    discord::send_dm(
        &state.config.discord_bot_token,
        &state.config.discord_user_id,
        &message,
    )
    .await?;

    Ok(StatusCode::ACCEPTED)
}

/// Client IP from the X-Forwarded-For header (first hop set by the proxy).
fn client_ip(headers: &HeaderMap) -> Option<String> {
    let forwarded = headers.get("x-forwarded-for")?.to_str().ok()?;
    forwarded.split(',').next().map(|ip| ip.trim().to_string())
}
