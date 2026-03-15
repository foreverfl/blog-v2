-- V4: Add unique constraint on assets.sha256 to prevent duplicate uploads

CREATE UNIQUE INDEX IF NOT EXISTS idx_assets_sha256_unique
ON public.assets (sha256)
WHERE sha256 IS NOT NULL;
