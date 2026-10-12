-- V20: Create places — every spot on /map except ITS (UR, breweries, food …), been there or not

CREATE TABLE IF NOT EXISTS public.places (
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name        text NOT NULL,
    category    text NOT NULL CHECK (category IN ('ur', 'brewery', 'food', 'cafe', 'stay', 'sight', 'other')),
    subcategory text,
    address     text,
    lat         double precision NOT NULL,
    lng         double precision NOT NULL,
    visited_on  date,
    note        text,
    url         text,
    image_url   text,
    created_at  timestamptz NOT NULL DEFAULT now(),
    UNIQUE NULLS NOT DISTINCT (name, category, subcategory)
);

COMMENT ON TABLE  public.places IS 'every spot on /map except ITS, been there or not; rows pushed with psql from work repo seeds/places/*.sql';
COMMENT ON COLUMN public.places.category IS 'ur | brewery | food | cafe | stay | sight | other; other = anything else, named in note, promoted once rows pile up';
COMMENT ON COLUMN public.places.subcategory IS 'free text detail inside a category, e.g. brewery: whisky | shochu | sake | gin | beer | wine | other; NULL when the category needs none';
COMMENT ON COLUMN public.places.visited_on IS 'NULL = not visited yet';
COMMENT ON COLUMN public.places.url IS 'the official page, or a Google Maps link';
COMMENT ON COLUMN public.places.image_url IS 'one representative photo on the blog asset bucket; NULL if none';
