use crate::types::ApiError;

const SITEVERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

#[derive(serde::Deserialize)]
struct SiteverifyResponse {
    success: bool,
}

/// Verify a Cloudflare Turnstile token against the siteverify API.
/// `remote_ip` is the client IP — optional, but recommended by Cloudflare.
/// Returns whether the token is valid for the given secret.
pub async fn verify(
    secret: &str,
    token: &str,
    remote_ip: Option<&str>,
) -> Result<bool, ApiError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ApiError::Internal(format!("failed to build http client: {e}")))?;

    let mut payload = serde_json::json!({ "secret": secret, "response": token });
    if let Some(ip) = remote_ip {
        payload["remoteip"] = serde_json::Value::String(ip.to_string());
    }

    let resp = client
        .post(SITEVERIFY_URL)
        .json(&payload)
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("turnstile request failed: {e}")))?;

    let body: SiteverifyResponse = resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("turnstile decode failed: {e}")))?;

    Ok(body.success)
}
