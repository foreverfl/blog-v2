use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<AppConfig>,
    pub s3: aws_sdk_s3::Client,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub frontend_url: String,
    pub s3_bucket: String,
    pub s3_prefix: String,
    pub max_upload_size: usize,
    pub import_secret: String,
    pub github_token: Option<String>,
    pub openai_api_key: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL required"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET required"),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
            s3_bucket: env::var("S3_BUCKET").unwrap_or_else(|_| "blog-assets".into()),
            s3_prefix: env::var("S3_PREFIX").unwrap_or_else(|_| "uploads".into()),
            max_upload_size: env::var("MAX_UPLOAD_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50 * 1024 * 1024), // 50 MB
            import_secret: env::var("IMPORT_SECRET").expect("IMPORT_SECRET required"),
            github_token: env::var("GITHUB_TOKEN").ok(),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required"),
        }
    }
}