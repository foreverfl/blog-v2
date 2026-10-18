-- V21: Merge ITS into places — ITS-only columns move to its_facility_details, per-user records to place_visits

CREATE TABLE IF NOT EXISTS public.its_facility_details (
    place_id         uuid PRIMARY KEY REFERENCES public.places(id) ON DELETE CASCADE,
    kind             text NOT NULL CHECK (kind IN ('direct', 'yearround', 'seasonal')),
    seasons          text[] NOT NULL DEFAULT '{}',
    prefecture       text NOT NULL,
    city             text NOT NULL,
    room_types       text NOT NULL,
    min_party        smallint,
    detail_url       text,
    market_price_yen integer,
    closed           boolean NOT NULL DEFAULT false
);

COMMENT ON TABLE  public.its_facility_details IS 'ITS-only columns for a places row (category stay, subcategory its); seeded together with the places row from work repo seeds/001';
COMMENT ON COLUMN public.its_facility_details.kind IS 'direct | yearround | seasonal';
COMMENT ON COLUMN public.its_facility_details.seasons IS 'seasonal only: summer, winter, or both';
COMMENT ON COLUMN public.its_facility_details.room_types IS 'free text as printed in the PDF, e.g. 4名洋室または5名和洋室';
COMMENT ON COLUMN public.its_facility_details.min_party IS '1 or 2; NULL until the detail page is checked';
COMMENT ON COLUMN public.its_facility_details.detail_url IS 'the ITS page; the hotel''s own site is places.url';
COMMENT ON COLUMN public.its_facility_details.market_price_yen IS 'usual price per adult per night on the hotel''s own site, same meal plan as ITS; NULL until looked up. ITS charges 6,600 so the gap is the bragging number';
COMMENT ON COLUMN public.its_facility_details.closed IS 'true while a facility is shut (あえの風 after the 能登 quake)';

CREATE TABLE IF NOT EXISTS public.place_visits (
    user_id         uuid NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    place_id        uuid NOT NULL REFERENCES public.places(id) ON DELETE CASCADE,
    applied_count   smallint NOT NULL DEFAULT 0,
    last_applied_on date,
    visited_on      date,
    visited_with    text,
    review          text,
    created_at      timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, place_id)
);

COMMENT ON TABLE  public.place_visits IS 'one row per user per place: how many times they applied (ITS) and whether they went; edited from the /map place modal';
COMMENT ON COLUMN public.place_visits.applied_count   IS 'ITS only: times this user applied for this facility, counted by hand';
COMMENT ON COLUMN public.place_visits.last_applied_on IS 'ITS only: date of the latest application; tells whether this month is already counted';

COMMENT ON TABLE public.places IS 'every spot on /map, ITS included, been there or not; rows pushed with psql from work repo seeds/NNN_<category>[-<subcategory>].sql';

DROP TABLE public.its_facility_visits;
DROP TABLE public.its_facilities;
