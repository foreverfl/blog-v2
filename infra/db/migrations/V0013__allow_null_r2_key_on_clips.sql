-- V13: Allow NULL r2_key on anime.clips — cleanup deletes the R2 object but keeps the row
-- (view history survives; the clip can be re-cut later). UNIQUE stays: NULLs never collide.

ALTER TABLE anime.clips ALTER COLUMN r2_key DROP NOT NULL;
