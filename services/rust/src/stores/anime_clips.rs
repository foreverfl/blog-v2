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
    series_title: Option<&str>,
) -> Result<ClipRow, ApiError> {
    let row = sqlx::query_as::<_, ClipRow>(
        r#"
        INSERT INTO anime.clips (r2_key, series_slug, episode, start_sec, duration_sec, jellyfin_item, is_opening, series_title)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (r2_key) DO UPDATE SET
            series_slug = EXCLUDED.series_slug,
            episode = EXCLUDED.episode,
            start_sec = EXCLUDED.start_sec,
            duration_sec = EXCLUDED.duration_sec,
            jellyfin_item = EXCLUDED.jellyfin_item,
            is_opening = EXCLUDED.is_opening,
            series_title = EXCLUDED.series_title
        RETURNING id, r2_key, series_slug, series_title, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, liked_at, view_count, last_viewed_at, created_at
        "#,
    )
    .bind(r2_key)
    .bind(series_slug)
    .bind(episode)
    .bind(start_sec)
    .bind(duration_sec)
    .bind(jellyfin_item)
    .bind(is_opening)
    .bind(series_title)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

/// List liked clips for the profile grid, newest like first, one page at a
/// time (offset paging is fine here — only one person ever likes clips).
///
/// @return up to `limit` rows starting at `offset`, liked_at descending.
pub async fn list_liked(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<ClipRow>, ApiError> {
    let rows = sqlx::query_as::<_, ClipRow>(
        r#"
        SELECT id, r2_key, series_slug, series_title, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, liked_at, view_count, last_viewed_at, created_at
        FROM anime.clips
        WHERE liked
        ORDER BY liked_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// List clips for the feed, in random order (seeding cuts whole episodes,
/// so id order would replay an episode front to back).
///
/// @param viewed - Some(false) = only unviewed (view_count = 0), Some(true) =
///                 only viewed ones ("the ones already seen"), None = all.
/// @return up to `limit` rows, randomly ordered.
pub async fn list(pool: &PgPool, viewed: Option<bool>, limit: i64) -> Result<Vec<ClipRow>, ApiError> {
    let rows = sqlx::query_as::<_, ClipRow>(
        r#"
        SELECT id, r2_key, series_slug, series_title, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, liked_at, view_count, last_viewed_at, created_at
        FROM anime.clips
        WHERE $1::boolean IS NULL OR ($1 AND view_count > 0) OR (NOT $1 AND view_count = 0)
        ORDER BY random()
        LIMIT $2
        "#,
    )
    .bind(viewed)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Count one viewing of a clip: view_count + 1, last_viewed_at = now.
///
/// @return the updated row, or None when the id does not exist.
pub async fn record_view(pool: &PgPool, id: i64) -> Result<Option<ClipRow>, ApiError> {
    let row = sqlx::query_as::<_, ClipRow>(
        r#"
        UPDATE anime.clips
        SET view_count = view_count + 1, last_viewed_at = now()
        WHERE id = $1
        RETURNING id, r2_key, series_slug, series_title, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, liked_at, view_count, last_viewed_at, created_at
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Fill a clip's jellyfin_item and series_title.
///
/// @param jellyfin_item - None leaves the stored value alone (same for series_title),
///                        so a title backfill cannot wipe an item id.
/// @return the updated row, or None when the id does not exist.
pub async fn patch_metadata(
    pool: &PgPool,
    id: i64,
    jellyfin_item: Option<&str>,
    series_title: Option<&str>,
) -> Result<Option<ClipRow>, ApiError> {
    let row = sqlx::query_as::<_, ClipRow>(
        r#"
        UPDATE anime.clips
        SET jellyfin_item = COALESCE($2, jellyfin_item), series_title = COALESCE($3, series_title)
        WHERE id = $1
        RETURNING id, r2_key, series_slug, series_title, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, liked_at, view_count, last_viewed_at, created_at
        "#,
    )
    .bind(id)
    .bind(jellyfin_item)
    .bind(series_title)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Fetch one clip by id.
///
/// @return the row, or None when the id does not exist.
pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<ClipRow>, ApiError> {
    let row = sqlx::query_as::<_, ClipRow>(
        r#"
        SELECT id, r2_key, series_slug, series_title, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, liked_at, view_count, last_viewed_at, created_at
        FROM anime.clips
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Set the liked flag, the r2_key it moved to, and when the like happened
/// (now() on like, back to NULL on unlike).
///
/// @return the updated row.
pub async fn set_liked(pool: &PgPool, id: i64, liked: bool, r2_key: &str) -> Result<ClipRow, ApiError> {
    let row = sqlx::query_as::<_, ClipRow>(
        r#"
        UPDATE anime.clips
        SET liked = $2, r2_key = $3, liked_at = CASE WHEN $2 THEN now() ELSE NULL END
        WHERE id = $1
        RETURNING id, r2_key, series_slug, series_title, episode, start_sec, duration_sec, jellyfin_item, is_opening, liked, liked_at, view_count, last_viewed_at, created_at
        "#,
    )
    .bind(id)
    .bind(liked)
    .bind(r2_key)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

/// Delete a clip row outright, after its R2 object was deleted.
///
/// @return nothing; deleting an already-gone id is a silent no-op.
pub async fn delete_row(pool: &PgPool, id: i64) -> Result<(), ApiError> {
    sqlx::query("DELETE FROM anime.clips WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
