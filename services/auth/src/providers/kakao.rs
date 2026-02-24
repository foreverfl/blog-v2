use serde::Deserialize;

use super::ProviderParams;
use crate::types::{AuthError, OAuthUserInfo};

pub fn auth_url(client_id: &str, redirect_uri: &str, state: &str) -> String {
    let mut url = url::Url::parse("https://kauth.kakao.com/oauth/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", "profile_nickname profile_image account_email")
        .append_pair("state", state);
    url.to_string()
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct KakaoUser {
    kakao_account: Option<KakaoAccount>,
}

#[derive(Deserialize)]
struct KakaoAccount {
    email: Option<String>,
    profile: Option<KakaoProfile>,
}

#[derive(Deserialize)]
struct KakaoProfile {
    nickname: Option<String>,
    profile_image_url: Option<String>,
}

pub async fn authenticate(
    params: &ProviderParams<'_>,
    code: &str,
) -> Result<OAuthUserInfo, AuthError> {
    let token_res: TokenResponse = params
        .http
        .post("https://kauth.kakao.com/oauth/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", params.client.client_id.as_str()),
            ("client_secret", params.client.client_secret.as_str()),
            ("redirect_uri", params.redirect_uri.as_str()),
            ("code", code),
        ])
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    let user: KakaoUser = params
        .http
        .get("https://kapi.kakao.com/v2/user/me")
        .bearer_auth(&token_res.access_token)
        .send()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?
        .json()
        .await
        .map_err(|e| AuthError::OAuth(e.to_string()))?;

    let account = user
        .kakao_account
        .ok_or_else(|| AuthError::OAuth("kakao account info not available".into()))?;
    let email = account
        .email
        .ok_or_else(|| AuthError::OAuth("email not available from Kakao".into()))?;
    let profile = account.profile;

    Ok(OAuthUserInfo {
        email,
        name: profile
            .as_ref()
            .and_then(|p| p.nickname.clone())
            .unwrap_or_default(),
        photo: profile.and_then(|p| p.profile_image_url),
        provider: "kakao".into(),
    })
}