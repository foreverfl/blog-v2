-- V12: Create anime schema + clips — metadata for clips cut from the anime library

-- Anime domain gets its own schema (more tables will follow: series, episodes, ...).
CREATE SCHEMA IF NOT EXISTS anime;

-- One clip file in R2 = one row. Views are a counter, not per-view rows —
-- single-user, so "how many times" is enough; restock check counts view_count = 0.
CREATE TABLE IF NOT EXISTS anime.clips (
    id             bigserial PRIMARY KEY,
    r2_key         text NOT NULL UNIQUE,           -- feed/... or liked/... (moved on like)
    series_slug    text NOT NULL,                  -- source series folder slug
    episode        text NOT NULL,                  -- SxxEyy
    start_sec      real NOT NULL,                  -- offset into the source episode
    duration_sec   real NOT NULL,
    jellyfin_item  text,                           -- itemId for jumping to the detail page
    is_opening     boolean NOT NULL DEFAULT FALSE, -- OP/ED-range clip (unused for now)
    liked          boolean NOT NULL DEFAULT FALSE, -- TRUE = excluded from auto-delete and restock count
    view_count     int NOT NULL DEFAULT 0,
    last_viewed_at timestamptz,
    created_at     timestamptz NOT NULL DEFAULT now()
);
