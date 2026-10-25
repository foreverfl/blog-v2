-- V24: Create github_repo_kinds — one row per repo with the AI-picked kind and its top 3 languages, written once

CREATE TABLE IF NOT EXISTS collector.github_repo_kinds (
    full_name     text        NOT NULL,
    kind          text        NOT NULL,
    languages     text[]      NOT NULL DEFAULT '{}',
    classified_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (full_name)
);

COMMENT ON TABLE  collector.github_repo_kinds IS 'one row per repo; imported by collector github-kinds-import from data/github_repo_kinds.jsonl in the blog-v2 repo, which the classify workflow appends to';
COMMENT ON COLUMN collector.github_repo_kinds.kind      IS 'one of ai-llm, dev-tools, web-frontend, backend, infra-devops, data-db, mobile, security, learning, other';
COMMENT ON COLUMN collector.github_repo_kinds.languages IS 'up to 3 languages from GET /repos/{full_name}/languages, most bytes first; languages[1] is the repo''s main language';
