use serde::Deserialize;

use super::ProviderParams;
use crate::types::{AuthError, OAuthUserInfo};

pub fn auth_url(client_id: &str, redirect_uri: &str, state: &str) -> String {
    let mut url = url::Url::parse("https://access.line.me/oauth2/v2.1/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", "profile openid email")
        .append_pair("state", state);
    url.to_string()
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
}

#[derive(Deserialize)]
struct LineProfile {
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "pictureUrl")]
    picture_url: Option<String>,
}

#[derive(Deserialize)]
struct IdTokenPayload {
    email: Option<String>,
}

pub async fn authenticate(
    params: &ProviderParams<'_>,
    code: &str,
) -> Result<OAuthUserInfo, AuthError> {
    let token_res: TokenResponse = params
        .http
        .post("https://api.line.me/oauth2/v2.1/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", params.client.client_id.as_str()),
            ("client_secret", params.client.client_secret.as_str()),
            ("redirect_uri", params.redirect_uri.as_str()),
        ])
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    let profile: LineProfile = params
        .http
        .get("https://api.line.me/v2/profile")
        .bearer_auth(&token_res.access_token)
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    // Extract email from id_token JWT payload
    let email = token_res
        .id_token
        .and_then(|t: String| {
            let parts: Vec<&str> = t.split('.').collect();
            if parts.len() != 3 {
                return None;
            }
            use base64::Engine;
            let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(parts[1])
                .ok()?;
            let claims: IdTokenPayload = serde_json::from_slice(&payload).ok()?;
            claims.email
        })
        .ok_or_else(|| AuthError::OAuth("email not available from LINE".into()))?;

    Ok(OAuthUserInfo {
        email,
        name: profile.display_name,
        photo: profile.picture_url,
        provider: "line".into(),
    })
}