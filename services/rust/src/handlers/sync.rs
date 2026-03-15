use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

use crate::config::AppState;
use crate::stores::postgres as pg;
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
pub struct SyncResult {
    pub synced: usize,
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

pub async fn sync_from_github(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    // Verify sync secret from X-Sync-Secret header (constant-time comparison)
    let secret = headers
        .get("X-Sync-Secret")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if secret.as_bytes().ct_eq(state.config.sync_secret.as_bytes()).unwrap_u8() != 1 {
        return Err(ApiError::InvalidToken);
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ApiError::Internal(format!("failed to build http client: {e}")))?;

    // 1. Fetch repo tree recursively
    let tree_url = format!(
        "https://api.github.com/repos/{}/git/trees/{}?recursive=1",
        GITHUB_REPO, GITHUB_BRANCH
    );
    let tree: GitHubTree = client
        .get(&tree_url)
        .header("User-Agent", "blog-sync")
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to fetch github tree: {e}")))?
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to parse github tree: {e}")))?;

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

    let mut result = SyncResult {
        synced: 0,
        skipped: 0,
        errors: vec![],
    };

    // 3. Process each file
    for entry in &content_files {
        let parsed = match parse_file_path(&entry.path) {
            Some(p) => p,
            None => {
                result.skipped += 1;
                continue;
            }
        };

        // Fetch raw file content
        let raw_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}",
            GITHUB_REPO, GITHUB_BRANCH, entry.path
        );
        let raw_content = match client
            .get(&raw_url)
            .header("User-Agent", "blog-sync")
            .send()
            .await
        {
            Ok(resp) => match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    result
                        .errors
                        .push(format!("{}: failed to read body: {e}", entry.path));
                    continue;
                }
            },
            Err(e) => {
                result
                    .errors
                    .push(format!("{}: failed to fetch: {e}", entry.path));
                continue;
            }
        };

        // Parse frontmatter and body
        let (frontmatter, body) = parse_frontmatter(&raw_content);

        // Upsert post
        let post = match pg::upsert_post(
            &state.db,
            &parsed.classification,
            &parsed.category,
            &parsed.slug,
        )
        .await
        {
            Ok(p) => p,
            Err(e) => {
                result
                    .errors
                    .push(format!("{}: failed to upsert post: {e}", entry.path));
                continue;
            }
        };

        // Upsert content
        let metadata = serde_json::json!({
            "date": frontmatter.date,
            "image": frontmatter.image,
            "source_path": entry.path,
        });

        if let Err(e) = pg::upsert_sync_content(
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
            result
                .errors
                .push(format!("{}: failed to upsert content: {e}", entry.path));
            continue;
        }

        result.synced += 1;
    }

    tracing::info!(
        synced = result.synced,
        skipped = result.skipped,
        errors = result.errors.len(),
        "github sync complete"
    );

    Ok((StatusCode::OK, Json(result)))
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

/// POST /sync/json — placeholder for JSON-based sync (not yet implemented)
pub async fn sync_json(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let secret = headers
        .get("X-Sync-Secret")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if secret.as_bytes().ct_eq(state.config.sync_secret.as_bytes()).unwrap_u8() != 1 {
        return Err(ApiError::InvalidToken);
    }

    Ok((
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({"error": "sync/json is not yet implemented"})),
    ))
}
