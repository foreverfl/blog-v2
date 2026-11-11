use chrono::NaiveDate;
use sqlx::PgPool;

use super::model::DatedRepoRow;

/// Lists one category's repos of a day by rank, skipping `offset` rows and returning at most `limit`;
/// the latest day when `date` is None.
#[tracing::instrument(name = "github.list_ranking", skip(pool))]
pub async fn list_ranking(
    pool: &PgPool,
    category: &str,
    date: Option<NaiveDate>,
    limit: i64,
    offset: i64,
) -> Result<Vec<DatedRepoRow>, sqlx::Error> {
    sqlx::query_as::<_, DatedRepoRow>(
        r#"
        SELECT s.snapshot_date, s.rank, s.full_name, s.url, s.description, s.stars,
               s.language, s.topics, s.repo_created_at, s.stars_today,
               k.kind, COALESCE(k.languages, '{}') AS languages
        FROM collector.github_repo_snapshots s
        LEFT JOIN collector.github_repo_kinds k ON k.full_name = s.full_name
        WHERE s.category = $1
          AND s.snapshot_date = COALESCE($2::date,
              (SELECT max(snapshot_date) FROM collector.github_repo_snapshots WHERE category = $1))
        ORDER BY s.rank
        LIMIT $3
        OFFSET $4
        "#,
    )
    .bind(category)
    .bind(date)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
