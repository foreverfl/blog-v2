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

/// Lists the top `limit` overall repos of one day by rank; the latest day when
/// `date` is None. NotFound when that day has no rows.
#[tracing::instrument(name = "github.list_overall", skip(pool))]
pub async fn list_overall(
    pool: &PgPool,
    date: Option<NaiveDate>,
    limit: i64,
) -> Result<RankingResponse, ApiError> {
    let rows = sqlx::query_as::<_, DatedRow>(
        r#"
        SELECT s.snapshot_date, s.rank, s.full_name, s.url, s.description, s.stars,
               s.language, s.topics, s.repo_created_at, s.stars_today,
               k.kind, COALESCE(k.languages, '{}') AS languages
        FROM collector.github_repo_snapshots s
        LEFT JOIN collector.github_repo_kinds k ON k.full_name = s.full_name
        WHERE s.category = 'overall'
          AND s.snapshot_date = COALESCE($1, (SELECT max(snapshot_date) FROM collector.github_repo_snapshots WHERE category = 'overall'))
        ORDER BY s.rank
        LIMIT $2
        "#,
    )
    .bind(date)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let date = rows.first().ok_or(ApiError::NotFound)?.snapshot_date;
    let repos = rows.into_iter().map(|r| r.repo).collect();
    Ok(RankingResponse { date, repos })
}
