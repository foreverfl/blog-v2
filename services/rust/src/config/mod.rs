use std::env;
use std::sync::Arc;

pub const ASSET_URL_PATTERN: &str = "https://assets-{bucket}.mogumogu.dev";

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<AppConfig>,
    pub s3: aws_sdk_s3::Client,
    pub redis: redis::Client,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub frontend_url: String,
    pub s3_bucket_blog_posts_assets: String,
    pub max_upload_size: usize,
    pub api_secret: String,
    pub github_token: Option<String>,
    pub openai_api_key: String,
    pub redis_url: String,
    pub bucket_prefix: String,
    pub turnstile_secret: String,
    pub discord_comments_webhook: Option<String>,
    pub discord_bug_reports_webhook: Option<String>,
    pub admin_emails: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL required"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET required"),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
            s3_bucket_blog_posts_assets: env::var("S3_BUCKET_BLOG_POSTS_ASSETS")
                .expect("S3_BUCKET_BLOG_POSTS_ASSETS required"),
            max_upload_size: env::var("MAX_UPLOAD_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50 * 1024 * 1024), // 50 MB
            api_secret: env::var("API_SECRET").expect("API_SECRET required"),
            github_token: env::var("GITHUB_TOKEN").ok(),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required"),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into()),
            bucket_prefix: env::var("BUCKET_PREFIX").unwrap_or_default(),
            turnstile_secret: env::var("TURNSTILE_SECRET_KEY")
                .expect("TURNSTILE_SECRET_KEY required"),
            // compose injects these as "" when unset — treat empty as absent
            discord_comments_webhook: env::var("DISCORD_COMMENTS_WEBHOOK")
                .ok()
                .filter(|url| !url.is_empty()),
            discord_bug_reports_webhook: env::var("DISCORD_BUG_REPORTS_WEBHOOK")
                .ok()
                .filter(|url| !url.is_empty()),
            admin_emails: env::var("ADMIN_EMAILS")
                .unwrap_or_default()
                .split(',')
                .map(|email| email.trim().to_string())
                .filter(|email| !email.is_empty())
                .collect(),
        }
    }

    /// Map a logical bucket name (what the API and UI use) to the physical
    /// R2 bucket for this environment: "dev-" prefix locally, none in prod.
    pub fn physical_bucket(&self, logical: &str) -> String {
        format!("{}{}", self.bucket_prefix, logical)
    }

    /// Reverse mapping: the logical name of a physical bucket, or None when
    /// the bucket does not belong to this environment (hidden from listings).
    pub fn logical_bucket<'a>(&self, physical: &'a str) -> Option<&'a str> {
        if self.bucket_prefix.is_empty() {
            if physical.starts_with("dev-") {
                None
            } else {
                Some(physical)
            }
        } else {
            physical.strip_prefix(&self.bucket_prefix)
        }
    }
}

#[cfg(test)]
mod tests {
    fn config_with_prefix(prefix: &str) -> super::AppConfig {
        super::AppConfig {
            database_url: String::new(),
            jwt_secret: String::new(),
            frontend_url: String::new(),
            s3_bucket_blog_posts_assets: String::new(),
            max_upload_size: 0,
            api_secret: String::new(),
            github_token: None,
            openai_api_key: String::new(),
            redis_url: String::new(),
            bucket_prefix: prefix.into(),
            turnstile_secret: String::new(),
            discord_comments_webhook: None,
            discord_bug_reports_webhook: None,
            admin_emails: Vec::new(),
        }
    }

    #[test]
    fn local_prefix_maps_and_filters() {
        let config = config_with_prefix("dev-");
        assert_eq!(config.physical_bucket("blog-posts-assets"), "dev-blog-posts-assets");
        assert_eq!(config.logical_bucket("dev-blog-posts-assets"), Some("blog-posts-assets"));
        assert_eq!(config.logical_bucket("blog-posts-images"), None);
    }

    #[test]
    fn prod_empty_prefix_passes_through_and_hides_dev() {
        let config = config_with_prefix("");
        assert_eq!(config.physical_bucket("blog-posts-images"), "blog-posts-images");
        assert_eq!(config.logical_bucket("blog-posts-images"), Some("blog-posts-images"));
        assert_eq!(config.logical_bucket("dev-blog-posts-assets"), None);
    }
}