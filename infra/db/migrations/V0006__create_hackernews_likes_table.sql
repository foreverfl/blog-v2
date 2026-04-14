-- V6: Create hackernews_likes table for per-user HN item likes

CREATE TABLE IF NOT EXISTS public.hackernews_likes (
    user_id uuid NOT NULL,
    hn_id bigint NOT NULL,
    created_at timestamptz DEFAULT now() NOT NULL,
    CONSTRAINT hackernews_likes_pkey PRIMARY KEY (user_id, hn_id),
    CONSTRAINT hackernews_likes_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);
