use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::aio::MultiplexedConnection,
    pub config: Arc<AppConfig>,
    pub http: reqwest::Client,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub access_token_ttl: i64,
    pub refresh_token_ttl: u64,
    pub frontend_url: String,
    pub server_url: String,
    pub providers: ProviderConfigs,
}

#[derive(Clone, Debug)]
pub struct ProviderConfigs {
    pub google: Option<OAuthClientConfig>,
    pub github: Option<OAuthClientConfig>,
    pub apple: Option<OAuthClientConfig>,
    pub line: Option<OAuthClientConfig>,
    pub kakao: Option<OAuthClientConfig>,
}

#[derive(Clone, Debug)]
pub struct OAuthClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL required"),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".into()),
            access_token_ttl: env::var("ACCESS_TOKEN_TTL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(900), // 15 min
            refresh_token_ttl: env::var("REFRESH_TOKEN_TTL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(604_800), // 7 days
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
            server_url: env::var("SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:8001".into()),
            providers: ProviderConfigs {
                google: Self::load_provider("GOOGLE"),
                github: Self::load_provider("GITHUB"),
                apple: Self::load_provider("APPLE"),
                line: Self::load_provider("LINE"),
                kakao: Self::load_provider("KAKAO"),
            },
        }
    }

    fn load_provider(prefix: &str) -> Option<OAuthClientConfig> {
        let client_id = env::var(format!("{prefix}_CLIENT_ID")).ok()?;
        let client_secret = env::var(format!("{prefix}_CLIENT_SECRET")).ok()?;
        Some(OAuthClientConfig {
            client_id,
            client_secret,
        })
    }

    pub fn redirect_uri(&self, provider: &str) -> String {
        format!("{}/auth/callback/{}", self.server_url, provider)
    }
}