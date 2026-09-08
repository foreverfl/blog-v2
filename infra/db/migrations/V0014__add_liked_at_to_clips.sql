-- V14: Record when a clip was liked, so the liked grid can order by it
-- (the feed list is ORDER BY random(), which the grid inherits).

ALTER TABLE anime.clips ADD COLUMN liked_at timestamptz;

-- Existing likes have no recorded time. A clip is liked while watching it, so
-- the last view is the closest stand-in; created_at (nightly cut time) backs up
-- the rows that were liked without a counted view.
UPDATE anime.clips SET liked_at = COALESCE(last_viewed_at, created_at) WHERE liked;
