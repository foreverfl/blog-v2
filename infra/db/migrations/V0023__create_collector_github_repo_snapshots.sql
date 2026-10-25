-- V23: Create collector schema + github_repo_snapshots — the top repos per unit (search query or trending page), one snapshot a day

CREATE SCHEMA IF NOT EXISTS collector;

CREATE TABLE IF NOT EXISTS collector.github_repo_snapshots (
    snapshot_date   date        NOT NULL,
    category        text        NOT NULL,
    rank            smallint    NOT NULL,
    full_name       text        NOT NULL,
    url             text        NOT NULL,
    description     text,
    stars           integer     NOT NULL,
    language        text,
    topics          text[]      NOT NULL DEFAULT '{}',
    repo_created_at timestamptz NOT NULL,
    stars_today     integer,
    PRIMARY KEY (snapshot_date, category, full_name)
);

COMMENT ON TABLE  collector.github_repo_snapshots IS 'one row per repo per category per day; written by the collector CLI, read by the trends page';
COMMENT ON COLUMN collector.github_repo_snapshots.category    IS 'unit key from the collector: overall, claude-code, mcp-server, observability (search) or trending-daily, trending-monthly (scraped)';
COMMENT ON COLUMN collector.github_repo_snapshots.full_name   IS 'owner/repo';
COMMENT ON COLUMN collector.github_repo_snapshots.stars_today IS 'the "N stars today / this month" figure on github.com/trending; NULL for search units';
