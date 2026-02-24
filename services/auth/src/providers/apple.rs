use serde::Deserialize;

use super::ProviderParams;
use crate::types::{AuthError, OAuthUserInfo};

pub fn auth_url(client_id: &str, redirect_uri: &str, state: &str) -> String {
    let mut url = url::Url::parse("https://appleid.apple.com/auth/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", "name email")
        .append_pair("state", state)
        .append_pair("response_mode", "query");
    url.to_string()
}

#[derive(Deserialize)]
struct TokenResponse {
    id_token: String,
}

#[derive(Deserialize)]
struct IdTokenPayload {
    #[allow(dead_code)]
    sub: String,
    email: Option<String>,
}

pub async fn authenticate(
    params: &ProviderParams<'_>,
    code: &str,
) -> Result<OAuthUserInfo, AuthError> {
    let token_res: TokenResponse = params
        .http
        .post("https://appleid.apple.com/auth/token")
        .form(&[
            ("code", code),
            ("client_id", params.client.client_id.as_str()),
            ("client_secret", params.client.client_secret.as_str()),
            ("redirect_uri", params.redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    // Decode id_token JWT payload (trusted from Apple's token endpoint over HTTPS)
    let parts: Vec<&str> = token_res.id_token.split('.').collect();
    if parts.len() != 3 {
        return Err(AuthError::OAuth("invalid id_token format".into()));
    }

    use base64::Engine;
    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|e| AuthError::OAuth(format!("id_token decode error: {e}")))?;

    let claims: IdTokenPayload = serde_json::from_slice(&payload_bytes)
        .map_err(|e| AuthError::OAuth(format!("id_token parse error: {e}")))?;

    let email = claims
        .email
        .ok_or_else(|| AuthError::OAuth("email not available from Apple".into()))?;

    // Apple only sends user name on first authorization; use email prefix as fallback
    let name = email.split('@').next().unwrap_or("").to_string();

    Ok(OAuthUserInfo {
        email,
        name,
        photo: None,
        provider: "apple".into(),
    })
}