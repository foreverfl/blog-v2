-- V18: Move the visit record off its_facilities into a per-user table, and keep the application count there too

CREATE TABLE IF NOT EXISTS public.its_facility_visits (
    user_id         uuid NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    facility_id     uuid NOT NULL REFERENCES public.its_facilities(id) ON DELETE CASCADE,
    applied_count   smallint NOT NULL DEFAULT 0,
    last_applied_on date,
    visited_on      date,
    visited_with    text,
    review          text,
    created_at      timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, facility_id)
);

COMMENT ON TABLE  its_facility_visits IS 'one row per user per ITS facility: how many times they applied and whether they stayed; edited from the /map place modal';
COMMENT ON COLUMN its_facility_visits.applied_count   IS 'times this user applied for this facility, counted by hand';
COMMENT ON COLUMN its_facility_visits.last_applied_on IS 'date of the latest application; tells whether this month is already counted';

ALTER TABLE public.its_facilities
    DROP COLUMN visited_on,
    DROP COLUMN visited_with,
    DROP COLUMN review;
