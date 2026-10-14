package apinotes

import (
	"context"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

// LoadNotes returns every row ordered by lang, package, name, an empty slice when the table is empty, or the query error.
func LoadNotes(ctx context.Context, pool *pgxpool.Pool) ([]Note, error) {
	rows, err := pool.Query(ctx, `
		SELECT lang, package, name, major_version, summary, example, doc_url, gotcha,
		       seen_count, first_seen_in, last_seen_in, last_seen_at
		FROM code.api_notes
		ORDER BY lang, package, name`)
	if err != nil {
		return nil, err
	}
	return pgx.CollectRows(rows, pgx.RowToStructByName[Note])
}
