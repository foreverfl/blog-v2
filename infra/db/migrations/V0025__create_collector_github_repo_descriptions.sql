-- V25: Create github_repo_descriptions — the ko and ja translations of a repo description, one row per repo, written once

CREATE TABLE IF NOT EXISTS collector.github_repo_descriptions (
    full_name      text        PRIMARY KEY,
    description_ko text        NOT NULL,
    description_ja text        NOT NULL,
    created_at     timestamptz NOT NULL DEFAULT now()
);

COMMENT ON TABLE collector.github_repo_descriptions IS 'ko and ja translations of a repo description, one row per repo; written once by the collector CLI';
