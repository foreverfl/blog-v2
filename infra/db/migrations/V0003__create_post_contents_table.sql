-- V3: Create post_contents, assets, and post_assets tables

CREATE TABLE public.post_contents (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    post_id uuid NOT NULL,
    lang text NOT NULL,
    content_type text NOT NULL,
    title text NULL,
    excerpt text NULL,
    body_markdown text NULL,
    body_json jsonb NULL,
    body_text text NULL,
    metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp DEFAULT now() NULL,
    updated_at timestamp DEFAULT now() NULL,
    CONSTRAINT post_contents_pkey PRIMARY KEY (id),
    CONSTRAINT post_contents_post_id_fkey
        FOREIGN KEY (post_id) REFERENCES public.posts(id) ON DELETE CASCADE,
    CONSTRAINT unique_post_content_lang UNIQUE (post_id, lang)
);

CREATE INDEX idx_post_contents_lang
ON public.post_contents USING btree (lang);

CREATE INDEX idx_post_contents_content_type
ON public.post_contents USING btree (content_type);

-- assets
CREATE TABLE public.assets (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    bucket text NOT NULL,
    object_key text NOT NULL,
    file_name text NOT NULL,
    mime_type text NOT NULL,
    size_bytes bigint NOT NULL,
    sha256 text NULL,
    width integer NULL,
    height integer NULL,
    duration_ms integer NULL,
    kind text NOT NULL,
    status text NOT NULL DEFAULT 'active',
    metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp DEFAULT now() NULL,
    updated_at timestamp DEFAULT now() NULL,
    CONSTRAINT assets_pkey PRIMARY KEY (id),
    CONSTRAINT assets_object_key_key UNIQUE (bucket, object_key)
);

-- post_assets
CREATE TABLE public.post_assets (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    post_id uuid NOT NULL,
    asset_id uuid NOT NULL,
    lang text NULL,
    role text NOT NULL,
    sort_order integer DEFAULT 0 NOT NULL,
    created_at timestamp DEFAULT now() NULL,
    CONSTRAINT post_assets_pkey PRIMARY KEY (id),
    CONSTRAINT post_assets_post_id_fkey
        FOREIGN KEY (post_id) REFERENCES public.posts(id) ON DELETE CASCADE,
    CONSTRAINT post_assets_asset_id_fkey
        FOREIGN KEY (asset_id) REFERENCES public.assets(id) ON DELETE CASCADE
);
