package apinotes

import (
	"context"
	"errors"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

const noteColumns = `lang, package, name, major_version, summary, example, doc_url, gotcha,
		       seen_count, first_seen_in, last_seen_in, last_seen_at`

// ErrNoteExists is returned by SaveNote when the key already carries a note.
var ErrNoteExists = errors.New("note already exists")

// savedNote is the row SaveNote gets back plus whether the upsert inserted it (xmax = 0) or filled an index row.
type savedNote struct {
	Note
	Inserted bool `db:"inserted"`
}

// LoadNotes returns the rows of lang (every row when lang is "") ordered by lang, package, name, an empty slice when none match, or the query error.
func LoadNotes(ctx context.Context, pool *pgxpool.Pool, lang string) ([]Note, error) {
	rows, err := pool.Query(ctx, `
		SELECT `+noteColumns+`
		FROM code.api_notes
		WHERE ($1 = '' OR lang = $1)
		ORDER BY lang, package, name`, lang)
	if err != nil {
		return nil, err
	}
	return pgx.CollectRows(rows, pgx.RowToStructByName[Note])
}

// SaveNote inserts the note, or fills the index row of the same key when it has no summary yet, and returns the row plus true when it was newly inserted; ErrNoteExists when a note is already there, or the query error.
func SaveNote(ctx context.Context, pool *pgxpool.Pool, input NoteInput) (Note, bool, error) {
	rows, err := pool.Query(ctx, `
		INSERT INTO code.api_notes
		       (lang, package, name, doc_url, summary, example, gotcha, major_version,
		        first_seen_in, last_seen_in, seen_count, last_seen_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
		        $9::text, $9::text, CASE WHEN $9::text IS NULL THEN 0 ELSE 1 END, CASE WHEN $9::text IS NULL THEN NULL ELSE now() END)
		ON CONFLICT (lang, package, name) DO UPDATE
		   SET summary = EXCLUDED.summary, example = EXCLUDED.example, gotcha = EXCLUDED.gotcha,
		       major_version = EXCLUDED.major_version, first_seen_in = EXCLUDED.first_seen_in,
		       last_seen_in = EXCLUDED.last_seen_in, seen_count = EXCLUDED.seen_count, last_seen_at = EXCLUDED.last_seen_at
		 WHERE code.api_notes.summary IS NULL
		RETURNING `+noteColumns+`, xmax = 0 AS inserted`,
		input.Lang, input.Package, input.Name, input.DocURL, input.Summary, input.Example,
		input.Gotcha, input.MajorVersion, input.FirstSeenIn)
	if err != nil {
		return Note{}, false, err
	}
	saved, err := pgx.CollectOneRow(rows, pgx.RowToStructByName[savedNote])
	if errors.Is(err, pgx.ErrNoRows) {
		return Note{}, false, ErrNoteExists
	}
	if err != nil {
		return Note{}, false, err
	}
	return saved.Note, saved.Inserted, nil
}
