package apinotes

import (
	"context"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

// LoadNotes returns the rows of lang (every row when lang is "") ordered by lang, package, name, an empty slice when none match, or the query error.
func LoadNotes(ctx context.Context, pool *pgxpool.Pool, lang string) ([]Note, error) {
	rows, err := pool.Query(ctx, `
		SELECT lang, package, name, major_version, summary, example, doc_url, gotcha,
		       seen_count, first_seen_in, last_seen_in, last_seen_at
		FROM code.api_notes
		WHERE ($1 = '' OR lang = $1)
		ORDER BY lang, package, name`, lang)
	if err != nil {
		return nil, err
	}
	return pgx.CollectRows(rows, pgx.RowToStructByName[Note])
}
