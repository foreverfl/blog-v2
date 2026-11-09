use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// One repo of a day's ranking: a collector.github_repo_snapshots row without
/// snapshot_date / category, plus kind and languages from github_repo_kinds.
///
/// kind is null and languages empty until the repo has been classified.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RepoRow {
    pub rank: i16,
    pub full_name: String,
    pub url: String,
    pub description: Option<String>,
    pub stars: i32,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub repo_created_at: DateTime<Utc>,
    pub stars_today: Option<i32>,
    pub kind: Option<String>,
    pub languages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListOverallQuery {
    pub date: Option<String>,
    pub limit: Option<i32>,
}

/// Body of GET /github/overall: the day the rows belong to and the rows.
#[derive(Debug, Serialize)]
pub struct RankingResponse {
    pub date: NaiveDate,
    pub repos: Vec<RepoRow>,
}
