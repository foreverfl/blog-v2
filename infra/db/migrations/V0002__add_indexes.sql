-- V2: Create performance indexes

-- Posts
CREATE INDEX IF NOT EXISTS idx_posts_classification_category ON public.posts(classification, category);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON public.posts(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_posts_slug ON public.posts(slug);
CREATE INDEX IF NOT EXISTS idx_posts_indexed ON public.posts(indexed);

-- Comments
CREATE INDEX IF NOT EXISTS idx_comments_post_id ON public.comments(post_id);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON public.comments(user_id);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON public.comments(created_at DESC);

-- Likes
CREATE INDEX IF NOT EXISTS idx_likes_post_id ON public.likes(post_id);
CREATE INDEX IF NOT EXISTS idx_likes_user_id ON public.likes(user_id);

-- Visitor Fingerprint
CREATE INDEX IF NOT EXISTS idx_visitor_fingerprint_fingerprint ON public.visitor_fingerprint(fingerprint);
CREATE INDEX IF NOT EXISTS idx_visitor_fingerprint_is_bot ON public.visitor_fingerprint(is_bot);
CREATE INDEX IF NOT EXISTS idx_visitor_fingerprint_country ON public.visitor_fingerprint(country);

-- Anime
CREATE INDEX IF NOT EXISTS idx_anime_is_visible ON public.anime(is_visible);
CREATE INDEX IF NOT EXISTS idx_anime_season_year ON public.anime(season, season_year);
CREATE INDEX IF NOT EXISTS idx_anime_seasons_info ON public.anime USING GIN(seasons_info);
CREATE INDEX IF NOT EXISTS idx_anime_romaji_title ON public.anime(romaji_title);

-- API Usage
CREATE INDEX IF NOT EXISTS idx_api_usage_api_name ON public.api_usage(api_name);
CREATE INDEX IF NOT EXISTS idx_api_usage_date ON public.api_usage(date DESC);
