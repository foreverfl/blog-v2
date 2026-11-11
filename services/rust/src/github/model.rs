use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// One repo of a day's ranking: a github_repo_snapshots row joined to its github_repo_kinds row.
///
/// kind is null and languages empty for a repo that has not been classified yet.
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
}

/// Body of GET /github/overall.
#[derive(Debug, Serialize)]
pub struct RankingResponse {
    pub date: NaiveDate,
    pub repos: Vec<RepoRow>,
}
