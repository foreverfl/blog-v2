mod apple;
mod github;
mod google;
mod kakao;
mod line;

use crate::config::{AppConfig, OAuthClientConfig};
use crate::types::{AuthError, OAuthUserInfo};

pub struct ProviderParams<'a> {
    pub client: &'a OAuthClientConfig,
    pub redirect_uri: String,
    pub http: &'a reqwest::Client,
}

pub enum Provider {
    Google,
    Github,
    Apple,
    Line,
    Kakao,
}

impl Provider {
    pub fn from_name(s: &str) -> Result<Self, AuthError> {
        match s {
            "google" => Ok(Self::Google),
            "github" => Ok(Self::Github),
            "apple" => Ok(Self::Apple),
            "line" => Ok(Self::Line),
            "kakao" => Ok(Self::Kakao),
            other => Err(AuthError::UnsupportedProvider(other.into())),
        }
    }

    pub fn auth_url(&self, config: &AppConfig, state: &str) -> Result<String, AuthError> {
        let client: OAuthClientConfig = self.client_config(config)?;
        let redirect_uri = config.redirect_uri(self.name());

        Ok(match self {
            Self::Google => google::auth_url(&client.client_id, &redirect_uri, state),
            Self::Github => github::auth_url(&client.client_id, &redirect_uri, state),
            Self::Apple => apple::auth_url(&client.client_id, &redirect_uri, state),
            Self::Line => line::auth_url(&client.client_id, &redirect_uri, state),
            Self::Kakao => kakao::auth_url(&client.client_id, &redirect_uri, state),
        })
    }

    pub async fn authenticate(
        &self,
        config: &AppConfig,
        http: &reqwest::Client,
        code: &str,
    ) -> Result<OAuthUserInfo, AuthError> {
        let client = self.client_config(config)?;
        let redirect_uri = config.redirect_uri(self.name());
        let params = ProviderParams {
            client: &client,
            redirect_uri,
            http,
        };

        match self {
            Self::Google => google::authenticate(&params, code).await,
            Self::Github => github::authenticate(&params, code).await,
            Self::Apple => apple::authenticate(&params, code).await,
            Self::Line => line::authenticate(&params, code).await,
            Self::Kakao => kakao::authenticate(&params, code).await,
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Google => "google",
            Self::Github => "github",
            Self::Apple => "apple",
            Self::Line => "line",
            Self::Kakao => "kakao",
        }
    }

    fn client_config(&self, config: &AppConfig) -> Result<OAuthClientConfig, AuthError> {
        let cfg = match self {
            Self::Google => &config.providers.google,
            Self::Github => &config.providers.github,
            Self::Apple => &config.providers.apple,
            Self::Line => &config.providers.line,
            Self::Kakao => &config.providers.kakao,
        };
        cfg.clone()
            .ok_or_else(|| AuthError::ProviderNotConfigured(self.name().into()))
    }
}