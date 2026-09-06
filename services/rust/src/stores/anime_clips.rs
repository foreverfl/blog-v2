use sqlx::PgPool;

use crate::types::{ApiError, ClipRow};

/// Insert a clip row, refreshing the existing row on the same r2_key so
/// re-uploading a clip stays idempotent.
///
/// @return the stored row as it now exists in the database.
pub async fn upsert(
    pool: &PgPool,
    r2_key: &str,
    series_slug: &str,
    episode: &str,
    start_sec: f32,
    duration_sec: f32,
    jellyfin_item: Option<&str>,
    is_opening: bool,
) -> Result<ClipRow, ApiError> {
    let row = sqlx::query_as::<_, ClipRow>(
        r#"
        INSERT INTO anime.clips (r2_key, series_slug, episode, start_sec, duration_sec, jellyfin_item, is_opening)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (r2_key) DO UPDATE SET
            series_slug = EXCLUDED.series_slug,
            episode = EXCLUDED.episode,
            start_sec = EXCLUDED.start_sec,
            duration_sec = EXCLUDED.duration_sec,
            jellyfin_item = EXCLUDED.jellyfin_item,
            is_opening = EXCLUDED.is_opening
        RETURNING id, r2_key, series_slug, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, view_count, last_viewed_at, created_at
        "#,
    )
    .bind(r2_key)
    .bind(series_slug)
    .bind(episode)
    .bind(start_sec)
    .bind(duration_sec)
    .bind(jellyfin_item)
    .bind(is_opening)
    .fetch_one(pool)
    .await?;

    Ok(row)
}
