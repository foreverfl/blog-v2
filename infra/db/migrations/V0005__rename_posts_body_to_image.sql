-- Rename posts.body to posts.image (stores thumbnail/cover image URL)
ALTER TABLE public.posts RENAME COLUMN body TO image;
