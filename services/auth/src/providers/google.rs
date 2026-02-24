use serde::Deserialize;

use super::ProviderParams;
use crate::types::{AuthError, OAuthUserInfo};

pub fn auth_url(client_id: &str, redirect_uri: &str, state: &str) -> String {
    let mut url = url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", "openid email profile")
        .append_pair("state", state)
        .append_pair("access_type", "offline");
    url.to_string()
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct UserInfo {
    email: String,
    name: Option<String>,
    picture: Option<String>,
}

pub async fn authenticate(
    params: &ProviderParams<'_>,
    code: &str,
) -> Result<OAuthUserInfo, AuthError> {
    let token_res: TokenResponse = params
        .http
        .post("https://oauth2.googleapis.com/token")
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

    let user: UserInfo = params
        .http
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&token_res.access_token)
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    Ok(OAuthUserInfo {
        email: user.email,
        name: user.name.unwrap_or_default(),
        photo: user.picture,
        provider: "google".into(),
    })
}