package snapshot

import (
	"context"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"

	"blog-go-cli/internal/github"
)

const upsertSnapshotSQL = `
INSERT INTO collector.github_repo_snapshots
  (snapshot_date, category, rank, full_name, url, description, stars, language, topics, repo_created_at)
VALUES ($1, $2, $3, $4, $5, NULLIF($6, ''), $7, NULLIF($8, ''), $9, $10)
ON CONFLICT (snapshot_date, category, full_name) DO UPDATE SET
  rank = EXCLUDED.rank,
  url = EXCLUDED.url,
  description = EXCLUDED.description,
  stars = EXCLUDED.stars,
  language = EXCLUDED.language,
  topics = EXCLUDED.topics,
  repo_created_at = EXCLUDED.repo_created_at`

// saveSnapshot upserts unitRepos as one day's ranking for category, rank 1 first, or returns the first failed row's error.
func saveSnapshot(ctx context.Context, pool *pgxpool.Pool, snapshotDate string, category string, unitRepos []github.Repo) error {
	batch := &pgx.Batch{}
	for index, unitRepo := range unitRepos {
		topics := unitRepo.Topics
		if topics == nil {
			topics = []string{}
		}
		batch.Queue(upsertSnapshotSQL,
			snapshotDate, category, index+1, unitRepo.FullName, unitRepo.URL, unitRepo.Description,
			unitRepo.Stars, unitRepo.Language, topics, unitRepo.CreatedAt)
	}
	return pool.SendBatch(ctx, batch).Close()
}
