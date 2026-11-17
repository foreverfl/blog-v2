use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// One repo of a day's ranking: a github_repo_snapshots row joined to its github_repo_kinds row.
///
/// kind is null and languages empty for a repo that has not been classified yet;
/// description_ko and description_ja are null until its github_repo_descriptions row is written.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RepoRow {
    pub rank: i16,
    pub full_name: String,
    pub url: String,
    pub description: Option<String>,
    pub description_ko: Option<String>,
    pub description_ja: Option<String>,
    pub stars: i32,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub repo_created_at: DateTime<Utc>,
    pub stars_today: Option<i32>,
    pub kind: Option<String>,
    pub languages: Vec<String>,
}

/// A RepoRow together with the snapshot_date it was read from.
#[derive(Debug, sqlx::FromRow)]
pub struct DatedRepoRow {
    pub snapshot_date: NaiveDate,
    #[sqlx(flatten)]
    pub repo: RepoRow,
}

#[derive(Debug, Deserialize)]
pub struct ListOverallQuery {
    pub date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub lang: Option<String>,
}

/// Body of GET /github/overall.
#[derive(Debug, Serialize)]
pub struct RankingResponse {
    pub date: NaiveDate,
    pub repos: Vec<RepoRow>,
}

/// How many repos of a day's overall ranking are written mainly in one language.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LanguageCount {
    pub name: String,
    pub count: i64,
}

/// A LanguageCount together with the snapshot_date it was counted on.
#[derive(Debug, sqlx::FromRow)]
pub struct DatedLanguageCount {
    pub snapshot_date: NaiveDate,
    #[sqlx(flatten)]
    pub language: LanguageCount,
}

/// Body of GET /github/languages.
#[derive(Debug, Serialize)]
pub struct LanguagesResponse {
    pub date: NaiveDate,
    pub languages: Vec<LanguageCount>,
}
