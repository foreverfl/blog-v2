use chrono::NaiveDate;
use sqlx::PgPool;

use crate::types::ApiError;

use super::model::{RankingResponse, RepoRow};

#[derive(sqlx::FromRow)]
struct DatedRow {
    snapshot_date: NaiveDate,
    #[sqlx(flatten)]
    repo: RepoRow,
}

/// Lists `limit` overall repos of one day by rank after skipping `offset`; the
/// latest day when `date` is None. NotFound only when offset is 0 and that day
/// has no rows; a page past the end comes back with empty repos.
#[tracing::instrument(name = "github.list_overall", skip(pool))]
pub async fn list_overall(
    pool: &PgPool,
    date: Option<NaiveDate>,
    limit: i64,
    offset: i64,
) -> Result<RankingResponse, ApiError> {
    let rows = sqlx::query_as::<_, DatedRow>(
        r#"
        SELECT s.snapshot_date, s.rank, s.full_name, s.url, s.description, s.stars,
               s.language, s.topics, s.repo_created_at, s.stars_today,
               k.kind, COALESCE(k.languages, '{}') AS languages,
               d.description_ko, d.description_ja
        FROM collector.github_repo_snapshots s
        LEFT JOIN collector.github_repo_kinds k ON k.full_name = s.full_name
        LEFT JOIN collector.github_repo_descriptions d ON d.full_name = s.full_name
        WHERE s.category = 'overall'
          AND s.snapshot_date = COALESCE($1, (SELECT max(snapshot_date) FROM collector.github_repo_snapshots WHERE category = 'overall'))
        ORDER BY s.rank
        LIMIT $2
        OFFSET $3
        "#,
    )
    .bind(date)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let Some(first) = rows.first() else {
        return empty_page(pool, date, offset).await;
    };
    let date = first.snapshot_date;
    let repos = rows.into_iter().map(|r| r.repo).collect();
    Ok(RankingResponse { date, repos })
}

/// A page past the end: 200 with no repos, unless offset is 0 (the day has no
/// rows at all) or there is no overall day to name.
async fn empty_page(
    pool: &PgPool,
    date: Option<NaiveDate>,
    offset: i64,
) -> Result<RankingResponse, ApiError> {
    if offset == 0 {
        return Err(ApiError::NotFound);
    }
    let date = match date {
        Some(date) => date,
        None => sqlx::query_scalar::<_, Option<NaiveDate>>(
            "SELECT max(snapshot_date) FROM collector.github_repo_snapshots WHERE category = 'overall'",
        )
        .fetch_one(pool)
        .await?
        .ok_or(ApiError::NotFound)?,
    };
    Ok(RankingResponse {
        date,
        repos: Vec::new(),
    })
}
