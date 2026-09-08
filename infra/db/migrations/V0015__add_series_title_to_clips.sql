-- V15: Hold the series title the clip came from, so the feed can show it
-- instead of the romanised folder slug. NULL until the nightly upload fills it.

ALTER TABLE anime.clips ADD COLUMN series_title text;
