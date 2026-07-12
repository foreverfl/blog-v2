use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth;
use crate::config::AppState;
use crate::stores::{contents as content_store, jobs as job_store, posts as post_store};
use crate::types::ApiError;

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
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
    auth::verify_bearer_secret(&headers, &state.config.api_secret)?;

    // Parse YYMMDD → NaiveDate
    let from_date = chrono::NaiveDate::parse_from_str(&query.from, "%y%m%d")
        .map_err(|e| ApiError::BadRequest(format!("invalid date format (expected YYMMDD): {e}")))?;
    // Use JST (UTC+9) to match the pipeline's TZ=Asia/Tokyo date resolution
    let today = (chrono::Utc::now() + chrono::Duration::hours(9)).date_naive();

    tracing::info!(from = %from_date, today_jst = %today, "import/json date check");

    if from_date > today {
        return Err(ApiError::BadRequest(format!(
            "from date {} is in the future (today JST = {})", from_date, today
        )));
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

    tracing::info!(from = %from_date, to = %today, count = dates.len(), "importing date range");

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
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    auth::verify_secret_or_user(&state.config, &headers)?;

    match job_store::get(&state.redis, &job_id, &["json"]).await? {
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
    tracing::info!(batch_id = %batch_id, url = %json_url, "fetching batch from R2");

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
