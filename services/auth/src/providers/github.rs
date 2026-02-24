use serde::Deserialize;

use super::ProviderParams;
use crate::types::{AuthError, OAuthUserInfo};

pub fn auth_url(client_id: &str, redirect_uri: &str, state: &str) -> String {
    let mut url = url::Url::parse("https://github.com/login/oauth/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", "read:user user:email")
        .append_pair("state", state);
    url.to_string()
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GitHubUser {
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub async fn authenticate(
    params: &ProviderParams<'_>,
    code: &str,
) -> Result<OAuthUserInfo, AuthError> {
    let token_res: TokenResponse = params
        .http
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
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

    let user: GitHubUser = params
        .http
        .get("https://api.github.com/user")
        .bearer_auth(&token_res.access_token)
        .header("User-Agent", "blog-auth-api")
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    let emails: Vec<GitHubEmail> = params
        .http
        .get("https://api.github.com/user/emails")
        .bearer_auth(&token_res.access_token)
        .header("User-Agent", "blog-auth-api")
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    let email = emails
        .into_iter()
        .find(|e| e.primary && e.verified)
        .map(|e| e.email)
        .ok_or_else(|| AuthError::OAuth("no verified primary email".into()))?;

    Ok(OAuthUserInfo {
        email,
        name: user.name.unwrap_or(user.login),
        photo: user.avatar_url,
        provider: "github".into(),
    })
}