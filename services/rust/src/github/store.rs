use chrono::NaiveDate;
use sqlx::PgPool;

use super::model::{DatedLanguageCount, DatedRepoRow, KindCount};

/// Lists one category's repos of a day by rank, only those whose `language` matches when given,
/// skipping `offset` rows and returning at most `limit`; the latest day when `date` is None.
#[tracing::instrument(name = "github.list_ranking", skip(pool))]
pub async fn list_ranking(
    pool: &PgPool,
    category: &str,
    date: Option<NaiveDate>,
    language: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<DatedRepoRow>, sqlx::Error> {
    sqlx::query_as::<_, DatedRepoRow>(
        r#"
        SELECT s.snapshot_date, s.rank, s.full_name, s.url, s.description,
               d.description_ko, d.description_ja, s.stars,
               s.language, s.topics, s.repo_created_at, s.stars_today,
               k.kind, COALESCE(k.languages, '{}') AS languages
        FROM collector.github_repo_snapshots s
        LEFT JOIN collector.github_repo_kinds k ON k.full_name = s.full_name
        LEFT JOIN collector.github_repo_descriptions d ON d.full_name = s.full_name
        WHERE s.category = $1
          AND s.snapshot_date = COALESCE($2::date,
              (SELECT max(snapshot_date) FROM collector.github_repo_snapshots WHERE category = $1))
          AND ($3::text IS NULL OR s.language = $3)
        ORDER BY s.rank
        LIMIT $4
        OFFSET $5
        "#,
    )
    .bind(category)
    .bind(date)
    .bind(language)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// Counts the latest day's overall repos per language, most first, leaving out repos with no language;
/// empty when there are no snapshots.
#[tracing::instrument(name = "github.count_languages", skip(pool))]
pub async fn count_languages(pool: &PgPool) -> Result<Vec<DatedLanguageCount>, sqlx::Error> {
    sqlx::query_as::<_, DatedLanguageCount>(
        r#"
        SELECT snapshot_date, language AS name, count(*) AS count
        FROM collector.github_repo_snapshots
        WHERE category = 'overall'
          AND language IS NOT NULL
          AND snapshot_date =
              (SELECT max(snapshot_date) FROM collector.github_repo_snapshots WHERE category = 'overall')
        GROUP BY snapshot_date, language
        ORDER BY count(*) DESC, language
        "#,
    )
    .fetch_all(pool)
    .await
}

/// Counts classified repos per kind, most first; empty when nothing is classified yet.
#[tracing::instrument(name = "github.count_kinds", skip(pool))]
pub async fn count_kinds(pool: &PgPool) -> Result<Vec<KindCount>, sqlx::Error> {
    sqlx::query_as::<_, KindCount>(
        r#"
        SELECT kind AS name, count(*) AS count
        FROM collector.github_repo_kinds
        GROUP BY kind
        ORDER BY count(*) DESC, kind
        "#,
    )
    .fetch_all(pool)
    .await
}
