use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

use crate::config::AppState;
use crate::stores::{contents as content_store, jobs as job_store, posts as post_store};
use crate::types::ApiError;

const GITHUB_REPO: &str = "foreverfl/blog";
const GITHUB_BRANCH: &str = "main";
const CONTENTS_PREFIX: &str = "contents/";

#[derive(Debug, Deserialize)]
struct GitHubTree {
    tree: Vec<GitHubTreeEntry>,
}

#[derive(Debug, Deserialize)]
struct GitHubTreeEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

struct ParsedFile {
    classification: String,
    category: String,
    slug: String,
    lang: String,
    content_type: String,
}

struct ParsedFrontmatter {
    title: Option<String>,
    date: Option<String>,
    image: Option<String>,
}

/// POST /import/mdx — import markdown/mdx files from GitHub and upsert into DB.
/// Returns 202 immediately with a job_id. Results stored in Redis (TTL 24h).
pub async fn import_mdx_from_github(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let secret = headers
        .get("X-Import-Secret")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if secret
        .as_bytes()
        .ct_eq(state.config.import_secret.as_bytes())
        .unwrap_u8()
        != 1
    {
        return Err(ApiError::InvalidToken);
    }

    let job_id = uuid::Uuid::new_v4().to_string();

    job_store::set(&state.redis, "mdx", &job_id, &serde_json::json!({"status": "processing"})).await?;

    // Spawn background task
    let bg_state = state.clone();
    let bg_job_id = job_id.clone();
    tokio::spawn(async move {
        let result = run_mdx_import(&bg_state).await;

        tracing::info!(
            imported = result.imported,
            skipped = result.skipped,
            errors = result.errors.len(),
            "mdx import complete"
        );

        let error_count = result.errors.len();
        let errors_preview: Vec<&str> = result.errors.iter().take(50).map(|s| s.as_str()).collect();
        let payload = serde_json::json!({
            "status": "completed",
            "imported": result.imported,
            "skipped": result.skipped,
            "error_count": error_count,
            "errors": errors_preview,
            "files_processed": result.files_processed,
        });
        job_store::set_silent(&bg_state.redis, "mdx", &bg_job_id, &payload).await;
    });

    Ok((
        StatusCode::ACCEPTED,
        Json(serde_json::json!({ "job_id": job_id, "status": "processing" })),
    ))
}

#[derive(Debug, Serialize)]
struct MdxFileDetail {
    path: String,
    classification: String,
    category: String,
    slug: String,
    lang: String,
    status: String, // "imported", "skipped", "error"
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct MdxImportResult {
    imported: usize,
    skipped: usize,
    errors: Vec<String>,
    files_processed: Vec<MdxFileDetail>,
}

async fn run_mdx_import(state: &AppState) -> MdxImportResult {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return MdxImportResult {
                imported: 0,
                skipped: 0,
                errors: vec![format!("failed to build http client: {e}")],
                files_processed: vec![],
            };
        }
    };

    let github_token = state.config.github_token.as_deref();

    // 1. Fetch repo tree recursively
    let tree_url = format!(
        "https://api.github.com/repos/{}/git/trees/{}?recursive=1",
        GITHUB_REPO, GITHUB_BRANCH
    );
    let mut req = client
        .get(&tree_url)
        .header("User-Agent", "blog-import");
    if let Some(token) = github_token {
        req = req.header("Authorization", format!("Bearer {token}"));
    }
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            return MdxImportResult {
                imported: 0,
                skipped: 0,
                errors: vec![format!("failed to fetch github tree: {e}")],
                files_processed: vec![],
            };
        }
    };
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return MdxImportResult {
            imported: 0,
            skipped: 0,
            errors: vec![format!("github tree api returned {status}: {body}")],
            files_processed: vec![],
        };
    }
    let tree: GitHubTree = match resp.json().await {
        Ok(t) => t,
        Err(e) => {
            return MdxImportResult {
                imported: 0,
                skipped: 0,
                errors: vec![format!("failed to parse github tree: {e}")],
                files_processed: vec![],
            };
        }
    };

    // 2. Filter markdown/mdx files under contents/
    let content_files: Vec<&GitHubTreeEntry> = tree
        .tree
        .iter()
        .filter(|e| {
            e.entry_type == "blob"
                && e.path.starts_with(CONTENTS_PREFIX)
                && (e.path.ends_with(".md") || e.path.ends_with(".mdx"))
        })
        .collect();

    let mut result = MdxImportResult {
        imported: 0,
        skipped: 0,
        errors: vec![],
        files_processed: vec![],
    };

    // 3. Process each file
    for entry in &content_files {
        let parsed = match parse_file_path(&entry.path) {
            Some(p) => p,
            None => {
                tracing::info!(path = %entry.path, "skipped: could not parse file path");
                result.skipped += 1;
                result.files_processed.push(MdxFileDetail {
                    path: entry.path.clone(),
                    classification: String::new(),
                    category: String::new(),
                    slug: String::new(),
                    lang: String::new(),
                    status: "skipped".into(),
                    error: Some("could not parse file path".into()),
                });
                continue;
            }
        };

        let detail_base = |status: &str, error: Option<String>| MdxFileDetail {
            path: entry.path.clone(),
            classification: parsed.classification.clone(),
            category: parsed.category.clone(),
            slug: parsed.slug.clone(),
            lang: parsed.lang.clone(),
            status: status.into(),
            error,
        };

        // Fetch raw file content
        let raw_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}",
            GITHUB_REPO, GITHUB_BRANCH, entry.path
        );
        let mut req = client
            .get(&raw_url)
            .header("User-Agent", "blog-import");
        if let Some(token) = github_token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        let raw_content = match req.send().await {
            Ok(resp) => match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    let msg = format!("{}: failed to read body: {e}", entry.path);
                    result.errors.push(msg.clone());
                    result.files_processed.push(detail_base("error", Some(msg)));
                    continue;
                }
            },
            Err(e) => {
                let msg = format!("{}: failed to fetch: {e}", entry.path);
                result.errors.push(msg.clone());
                result.files_processed.push(detail_base("error", Some(msg)));
                continue;
            }
        };

        // Parse frontmatter and body
        let (frontmatter, body) = parse_frontmatter(&raw_content);

        // Upsert post
        let post = match post_store::upsert(
            &state.db,
            &parsed.classification,
            &parsed.category,
            &parsed.slug,
            frontmatter.image.as_deref(),
        )
        .await
        {
            Ok(p) => p,
            Err(e) => {
                let msg = format!("{}: failed to upsert post: {e}", entry.path);
                result.errors.push(msg.clone());
                result.files_processed.push(detail_base("error", Some(msg)));
                continue;
            }
        };

        // Upsert content
        let metadata = serde_json::json!({
            "date": frontmatter.date,
            "image": frontmatter.image,
            "source_path": entry.path,
        });

        if let Err(e) = content_store::upsert_sync(
            &state.db,
            post.id,
            &parsed.lang,
            &parsed.content_type,
            frontmatter.title.as_deref(),
            &body,
            &metadata,
        )
        .await
        {
            let msg = format!("{}: failed to upsert content: {e}", entry.path);
            result.errors.push(msg.clone());
            result.files_processed.push(detail_base("error", Some(msg)));
            continue;
        }

        result.imported += 1;
        result.files_processed.push(detail_base("imported", None));
    }

    result
}

/// Parse a file path like `contents/development/devnotes/001-some-slug-ko.mdx`
/// into classification, category, slug, lang, content_type.
fn parse_file_path(path: &str) -> Option<ParsedFile> {
    let rel = path.strip_prefix(CONTENTS_PREFIX)?;
    let parts: Vec<&str> = rel.splitn(3, '/').collect();
    if parts.len() != 3 {
        return None;
    }

    let classification = parts[0].to_string();
    let category = parts[1].to_string();
    let filename = parts[2];

    // Determine content_type from extension
    let (stem, content_type) = if let Some(s) = filename.strip_suffix(".mdx") {
        (s, "mdx")
    } else if let Some(s) = filename.strip_suffix(".md") {
        (s, "markdown")
    } else {
        return None;
    };

    // Extract lang from the last `-xx` suffix (2-letter lang code)
    let (slug, lang) = if stem.len() > 3 {
        let last_dash = stem.rfind('-')?;
        let lang_candidate = &stem[last_dash + 1..];
        if lang_candidate.len() == 2 && lang_candidate.chars().all(|c| c.is_ascii_lowercase()) {
            (
                stem[..last_dash].to_string(),
                lang_candidate.to_string(),
            )
        } else {
            // No lang suffix, default to "en"
            (stem.to_string(), "en".to_string())
        }
    } else {
        (stem.to_string(), "en".to_string())
    };

    Some(ParsedFile {
        classification,
        category,
        slug,
        lang,
        content_type: content_type.to_string(),
    })
}

/// Parse YAML frontmatter from markdown content.
/// Returns (frontmatter, body_without_frontmatter).
fn parse_frontmatter(content: &str) -> (ParsedFrontmatter, String) {
    let mut frontmatter = ParsedFrontmatter {
        title: None,
        date: None,
        image: None,
    };

    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (frontmatter, content.to_string());
    }

    // Find closing ---
    if let Some(end) = trimmed[3..].find("\n---") {
        let fm_block = &trimmed[3..end + 3];
        let body = &trimmed[end + 3 + 4..]; // skip \n---

        for line in fm_block.lines() {
            let line = line.trim();
            if let Some(val) = line.strip_prefix("title:") {
                frontmatter.title = Some(strip_quotes(val.trim()));
            } else if let Some(val) = line.strip_prefix("date:") {
                frontmatter.date = Some(strip_quotes(val.trim()));
            } else if let Some(val) = line.strip_prefix("image:") {
                frontmatter.image = Some(strip_quotes(val.trim()));
            }
        }

        return (frontmatter, body.trim_start().to_string());
    }

    (frontmatter, content.to_string())
}

fn strip_quotes(s: &str) -> String {
    s.trim_matches('"').trim_matches('\'').to_string()
}

const HN_JSON_BASE: &str = "https://blog_workers.forever-fl.workers.dev/hackernews";
const HN_IMAGE_BASE: &str = "https://blog_workers.forever-fl.workers.dev/hackernews-images";

#[derive(Debug, Deserialize)]
pub struct ImportJsonQuery {
    /// Start date in YYMMDD format, e.g. 250324. Imports from this date to today.
    pub from: String,
}

#[derive(Debug, Deserialize)]
struct HnItem {
    id: String,
    #[serde(rename = "hnId")]
    hn_id: Option<u64>,
    title: HnI18n,
    url: Option<String>,
    score: Option<i64>,
    by: Option<String>,
    time: Option<i64>,
    content: Option<String>,
    summary: HnI18n,
}

#[derive(Debug, Deserialize)]
struct HnI18n {
    en: Option<String>,
    ko: Option<String>,
    ja: Option<String>,
}

/// POST /import/json?from=250324 — import HN items from Cloudflare R2, from date to today.
/// Returns 202 immediately with a job_id. Results stored in Redis (TTL 24h).
pub async fn import_json(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ImportJsonQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let secret = headers
        .get("X-Import-Secret")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if secret
        .as_bytes()
        .ct_eq(state.config.import_secret.as_bytes())
        .unwrap_u8()
        != 1
    {
        return Err(ApiError::InvalidToken);
    }

    // Parse YYMMDD → NaiveDate
    let from_date = chrono::NaiveDate::parse_from_str(&query.from, "%y%m%d")
        .map_err(|e| ApiError::BadRequest(format!("invalid date format (expected YYMMDD): {e}")))?;
    let today = chrono::Utc::now().date_naive();

    if from_date > today {
        return Err(ApiError::BadRequest("from date is in the future".into()));
    }

    let job_id = uuid::Uuid::new_v4().to_string();

    job_store::set(&state.redis, "json", &job_id, &serde_json::json!({"status": "processing", "from": query.from})).await?;

    // Spawn background task
    let bg_state = state.clone();
    let bg_from = query.from.clone();
    let bg_job_id = job_id.clone();
    tokio::spawn(async move {
        let result = run_json_import(&bg_state, from_date, today).await;

        tracing::info!(
            imported = result.imported,
            skipped = result.skipped,
            errors = result.errors.len(),
            from = %bg_from,
            "json import complete"
        );

        let error_count = result.errors.len();
        let errors_preview: Vec<&str> = result.errors.iter().take(20).map(|s| s.as_str()).collect();
        let payload = serde_json::json!({
            "status": "completed",
            "from": bg_from,
            "imported": result.imported,
            "skipped": result.skipped,
            "error_count": error_count,
            "errors": errors_preview,
        });
        job_store::set_silent(&bg_state.redis, "json", &bg_job_id, &payload).await;
    });

    Ok((
        StatusCode::ACCEPTED,
        Json(serde_json::json!({ "job_id": job_id, "status": "processing" })),
    ))
}

async fn run_json_import(
    state: &AppState,
    from_date: chrono::NaiveDate,
    today: chrono::NaiveDate,
) -> ImportResult {
    use futures::stream::{self, StreamExt};

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return ImportResult {
                imported: 0,
                skipped: 0,
                errors: vec![format!("failed to build http client: {e}")],
            };
        }
    };

    let mut dates = vec![];
    let mut current = from_date;
    while current <= today {
        dates.push(current);
        current += chrono::Duration::days(1);
    }

    let batch_results: Vec<BatchResult> = stream::iter(dates)
        .map(|date| {
            let client = client.clone();
            async move {
                let batch_id = date.format("%y%m%d").to_string();
                fetch_batch(&client, &batch_id).await
            }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    let mut result = ImportResult {
        imported: 0,
        skipped: 0,
        errors: vec![],
    };

    for batch in batch_results {
        match batch {
            BatchResult::Items(batch_id, items) => {
                import_hn_batch(&state.db, &batch_id, &items, &mut result).await;
                tracing::info!(batch_id = %batch_id, items = items.len(), "batch processed");
            }
            BatchResult::Skip => {}
            BatchResult::Error(msg) => {
                result.errors.push(msg);
            }
        }
    }

    result
}

/// GET /import/jobs/{job_id} — check import job status
pub async fn get_import_job(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    match job_store::get(&state.redis, &job_id, &["mdx", "json"]).await? {
        Some(value) => Ok(Json(value)),
        None => Err(ApiError::NotFound),
    }
}

enum BatchResult {
    Items(String, Vec<HnItem>),
    Skip,
    Error(String),
}

async fn fetch_batch(client: &reqwest::Client, batch_id: &str) -> BatchResult {
    let json_url = format!("{}/{}.json", HN_JSON_BASE, batch_id);

    let resp = match client.get(&json_url).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(batch_id = %batch_id, "failed to fetch: {e:?}");
            return BatchResult::Error(format!("{batch_id}: fetch failed: {e:?}"));
        }
    };

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        tracing::debug!(batch_id = %batch_id, "no data, skipping");
        return BatchResult::Skip;
    }

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return BatchResult::Error(format!("{batch_id}: fetch returned {status}: {body}"));
    }

    match resp.json::<Vec<HnItem>>().await {
        Ok(items) => BatchResult::Items(batch_id.to_string(), items),
        Err(e) => BatchResult::Error(format!("{batch_id}: failed to parse json: {e}")),
    }
}

/// Strip null bytes that PostgreSQL text columns reject.
fn sanitize_json(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::String(s) => serde_json::Value::String(s.replace('\0', "")),
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(sanitize_json).collect())
        }
        serde_json::Value::Object(map) => {
            serde_json::Value::Object(map.iter().map(|(k, v)| (k.clone(), sanitize_json(v))).collect())
        }
        other => other.clone(),
    }
}

/// Import one batch (date) as a single post with items in body_json per language.
async fn import_hn_batch(
    db: &sqlx::PgPool,
    batch_id: &str,
    items: &[HnItem],
    result: &mut ImportResult,
) {
    // slug = date (e.g. "260313")
    let image_url = format!("{}/{}.webp", HN_IMAGE_BASE, batch_id);
    let post = match post_store::upsert(db, "trends", "hackernews", batch_id, Some(&image_url)).await {
        Ok(p) => p,
        Err(e) => {
            result
                .errors
                .push(format!("{batch_id}: failed to upsert post: {e}"));
            return;
        }
    };

    // Generate per-language titles
    let title_fn = |lang: &str| -> String {
        match lang {
            "ko" => format!("데일리 해커뉴스"),
            "ja" => format!("デイリーハッカーニュース"),
            _ => format!("Daily Hacker News"),
        }
    };

    // Build per-language item arrays
    for lang in &["en", "ko", "ja"] {
        let lang_items: Vec<serde_json::Value> = items
            .iter()
            .filter_map(|item| {
                let title = match lang {
                    &"en" => item.title.en.as_deref(),
                    &"ko" => item.title.ko.as_deref(),
                    &"ja" => item.title.ja.as_deref(),
                    _ => None,
                }?;

                let summary = match lang {
                    &"en" => item.summary.en.as_deref(),
                    &"ko" => item.summary.ko.as_deref(),
                    &"ja" => item.summary.ja.as_deref(),
                    _ => None,
                };

                let hn_id = item.hn_id.map(|id| id.to_string()).unwrap_or_else(|| item.id.clone());
                let image = item.hn_id.map(|id| format!("{}/{}.webp", HN_IMAGE_BASE, id));

                Some(serde_json::json!({
                    "hn_id": hn_id,
                    "title": title,
                    "summary": summary,
                    "content": item.content,
                    "url": item.url,
                    "score": item.score,
                    "by": item.by,
                    "time": item.time,
                    "image": image,
                }))
            })
            .collect();

        if lang_items.is_empty() {
            continue;
        }

        let body_json = sanitize_json(&serde_json::Value::Array(lang_items));

        let title = title_fn(lang);
        if let Err(e) = content_store::upsert_batch_json(
            db,
            post.id,
            lang,
            &title,
            &body_json,
        )
        .await
        {
            result
                .errors
                .push(format!("{batch_id}/{lang}: failed to upsert content: {e}"));
        }
    }

    result.imported += 1;
}
