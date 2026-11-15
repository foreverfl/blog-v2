-- V26: Point hacker news posts at .png on the days R2 holds only a png image (the rest keep .webp)

UPDATE public.posts
SET image = regexp_replace(image, '\.webp$', '.png')
WHERE classification = 'trends'
  AND category = 'hackernews'
  AND image LIKE '%.webp'
  AND (slug >= '260408' OR slug IN ('260322', '260331', '260401', '260402', '260404'));
