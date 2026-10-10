-- V19: Create code schema + api_notes — one row per outside API: an index row (name + doc link) until the review-api skill fills in the note, then how many diffs it has shown up in

CREATE SCHEMA IF NOT EXISTS code;

CREATE TABLE IF NOT EXISTS code.api_notes (
    id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    lang          text NOT NULL,
    package       text NOT NULL,
    name          text NOT NULL,
    major_version text,
    summary       text,
    example       text,
    doc_url       text NOT NULL,
    gotcha        text,
    seen_count    smallint NOT NULL DEFAULT 0,
    first_seen_in text,
    last_seen_in  text,
    last_seen_at  timestamptz,
    created_at    timestamptz NOT NULL DEFAULT now(),
    UNIQUE (lang, package, name)
);

COMMENT ON TABLE  code.api_notes IS 'one row per outside API (built-in, stdlib, framework, library); index rows are seeded per language with summary NULL, the review-api skill fills the note through the Go API, the blog /api-notes page reads both';
COMMENT ON COLUMN code.api_notes.lang          IS 'php | go | nodejs | js | ts | bash | git | postgres | mysql | clickhouse | sparql | rust | java | python | swift';
COMMENT ON COLUMN code.api_notes.package       IS 'php extension (standard, mbstring), go package path (net/http), nodejs module (fs), library name (fastify), bash builtin | command, postgres function, mysql help category; builtin for a js built-in';
COMMENT ON COLUMN code.api_notes.name          IS 'exact symbol: Cut, NewRequest, register, mb_strlen, Array.prototype.flatMap';
COMMENT ON COLUMN code.api_notes.major_version IS 'installed major when the note was written, e.g. 5; NULL for built-ins and index rows';
COMMENT ON COLUMN code.api_notes.summary       IS 'one plain-English sentence with simple words: what it does, effect first; NULL = index row, no note yet';
COMMENT ON COLUMN code.api_notes.example       IS 'a 2-5 line example that runs on its own, with real values like [1, 2, 3] instead of variables; the output goes in a comment on the last line';
COMMENT ON COLUMN code.api_notes.gotcha        IS 'the mistake people make the first time they use it, in one short English line; NULL when there is none';
COMMENT ON COLUMN code.api_notes.seen_count    IS 'diffs this API has appeared in; 0 on an index row nobody has met yet';
COMMENT ON COLUMN code.api_notes.last_seen_in  IS '<repo>@<sha|wip>:<file>:<line> of the latest sighting, or <repo>@<date> on the work machine; text, not a link';
