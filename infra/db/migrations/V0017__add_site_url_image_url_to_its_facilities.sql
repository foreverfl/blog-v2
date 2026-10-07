-- V17: Add the hotel's own website and one representative photo to its_facilities

ALTER TABLE public.its_facilities
    ADD COLUMN site_url  text,
    ADD COLUMN image_url text;

COMMENT ON COLUMN its_facilities.site_url  IS 'the hotel''s own website; detail_url stays the ITS page';
COMMENT ON COLUMN its_facilities.image_url IS 'one representative photo, hosted on the blog asset bucket; NULL until uploaded';
