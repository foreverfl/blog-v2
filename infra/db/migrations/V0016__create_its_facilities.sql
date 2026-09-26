-- V16: Create its_facilities — the 関東ITS健保 resorts the author plans to visit, one row each

CREATE TABLE IF NOT EXISTS public.its_facilities (
    id               uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name             text NOT NULL UNIQUE,
    kind             text NOT NULL CHECK (kind IN ('direct', 'yearround', 'seasonal')),
    seasons          text[] NOT NULL DEFAULT '{}',
    prefecture       text NOT NULL,
    city             text NOT NULL,
    address          text NOT NULL,
    room_types       text NOT NULL,
    min_party        smallint,
    lat              double precision NOT NULL,
    lng              double precision NOT NULL,
    detail_url       text,
    market_price_yen integer,
    closed           boolean NOT NULL DEFAULT false,
    visited_on       date,
    visited_with     text,
    review           text,
    created_at       timestamptz NOT NULL DEFAULT now()
);

COMMENT ON TABLE  public.its_facilities IS '関東ITS健保 resorts; rows seeded from the FY2026 PDFs, visited_* written from the admin-only /its page';
COMMENT ON COLUMN public.its_facilities.kind IS 'direct | yearround | seasonal';
COMMENT ON COLUMN public.its_facilities.seasons IS 'seasonal only: summer, winter, or both';
COMMENT ON COLUMN public.its_facilities.room_types IS 'free text as printed in the PDF, e.g. 4名洋室または5名和洋室';
COMMENT ON COLUMN public.its_facilities.min_party IS '1 or 2; NULL until the detail page is checked';
COMMENT ON COLUMN public.its_facilities.market_price_yen IS 'usual price per adult per night on the hotel''s own site, same meal plan as ITS; NULL until looked up. ITS charges 6,600 so the gap is the bragging number';
COMMENT ON COLUMN public.its_facilities.closed IS 'true while a facility is shut (あえの風 after the 能登 quake)';
COMMENT ON COLUMN public.its_facilities.visited_on IS 'NULL = not visited yet';
