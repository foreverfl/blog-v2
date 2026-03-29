use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::config::AppState;
use crate::types::ApiError;

const OPENAI_CHAT_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-4o-mini";

#[derive(Debug, Deserialize)]
pub struct TranslateQuery {
    pub origin: String,
}

#[derive(Debug, Deserialize)]
pub struct TranslateRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslatedEntry {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct TranslateResponse {
    pub en: TranslatedEntry,
    pub ja: TranslatedEntry,
    pub ko: TranslatedEntry,
}

#[derive(Debug, Deserialize)]
struct SingleTranslation {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: Option<String>,
}

/// POST /posts/translate?origin=en
pub async fn translate(
    State(state): State<AppState>,
    Query(query): Query<TranslateQuery>,
    Json(req): Json<TranslateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let origin = query.origin.as_str();
    let target_langs: Vec<&str> = ["en", "ja", "ko"]
        .into_iter()
        .filter(|&l| l != origin)
        .collect();

    if target_langs.len() != 2 {
        return Err(ApiError::BadRequest(
            "origin must be one of: en, ja, ko".into(),
        ));
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| ApiError::Internal(format!("failed to build http client: {e}")))?;

    // Fire two translation calls in parallel
    let (r1, r2) = tokio::join!(
        translate_one(&client, &state.config.openai_api_key, origin, target_langs[0], &req.title, &req.content),
        translate_one(&client, &state.config.openai_api_key, origin, target_langs[1], &req.title, &req.content),
    );

    let t1 = r1?;
    let t2 = r2?;

    let original = TranslatedEntry {
        title: req.title,
        content: req.content,
    };

    let mut result = TranslateResponse {
        en: TranslatedEntry { title: String::new(), content: String::new() },
        ja: TranslatedEntry { title: String::new(), content: String::new() },
        ko: TranslatedEntry { title: String::new(), content: String::new() },
    };

    // Place original and translations into the right slots
    for (lang, entry) in [(origin, original), (target_langs[0], t1), (target_langs[1], t2)] {
        match lang {
            "en" => result.en = entry,
            "ja" => result.ja = entry,
            "ko" => result.ko = entry,
            _ => {}
        }
    }

    Ok(Json(result))
}

async fn translate_one(
    client: &reqwest::Client,
    api_key: &str,
    origin: &str,
    target: &str,
    title: &str,
    content: &str,
) -> Result<TranslatedEntry, ApiError> {
    let system_prompt = format!(
        "You are a professional translator for a technical blog. \
         Translate the given title and markdown content from \"{origin}\" to \"{target}\". \
         Preserve all markdown formatting, code blocks, links, and structure. \
         Do NOT translate code, variable names, URLs, or file paths. \
         Return a JSON object with \"title\" and \"content\" fields."
    );

    let user_prompt = format!(
        "Translate the following from \"{origin}\" to \"{target}\".\n\n\
         Title: {title}\n\n---\n\n{content}"
    );

    let body = serde_json::json!({
        "model": MODEL,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_prompt }
        ],
        "temperature": 0.3,
        "response_format": {
            "type": "json_schema",
            "json_schema": {
                "name": "translation",
                "strict": true,
                "schema": {
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "content": { "type": "string" }
                    },
                    "required": ["title", "content"],
                    "additionalProperties": false
                }
            }
        }
    });

    let resp = client
        .post(OPENAI_CHAT_URL)
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&body)
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("openai request failed ({target}): {e}")))?;

    let status = resp.status();
    if !status.is_success() {
        let err_body = resp.text().await.unwrap_or_default();
        return Err(ApiError::Internal(format!(
            "openai returned {status} ({target}): {err_body}"
        )));
    }

    let completion: ChatCompletion = resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to parse openai response ({target}): {e}")))?;

    let raw = completion
        .choices
        .first()
        .and_then(|c| c.message.content.as_deref())
        .ok_or_else(|| ApiError::Internal(format!("empty openai response ({target})")))?;

    let entry: SingleTranslation = serde_json::from_str(raw)
        .map_err(|e| ApiError::Internal(format!("failed to parse translation ({target}): {e}")))?;

    Ok(TranslatedEntry {
        title: entry.title,
        content: entry.content,
    })
}