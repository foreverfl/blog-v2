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

// LookupNotes bumps seen_count and last_seen_* of the asked keys that carry a note and returns them as noted, the asked keys that are index rows as indexed, the rest as missing, or the query error.
func LookupNotes(ctx context.Context, pool *pgxpool.Pool, seenIn string, keys []Key) (LookupResponse, error) {
	langs := make([]string, len(keys))
	packages := make([]string, len(keys))
	names := make([]string, len(keys))
	for i, key := range keys {
		langs[i], packages[i], names[i] = key.Lang, key.Package, key.Name
	}

	rows, err := pool.Query(ctx, `
		UPDATE code.api_notes AS n
		   SET seen_count = n.seen_count + 1, last_seen_in = $4, last_seen_at = now()
		  FROM UNNEST($1::text[], $2::text[], $3::text[]) AS k(lang, package, name)
		 WHERE n.lang = k.lang AND n.package = k.package AND n.name = k.name AND n.summary IS NOT NULL
		RETURNING n.lang, n.package, n.name, n.major_version, n.summary, n.example, n.doc_url, n.gotcha,
		          n.seen_count, n.first_seen_in, n.last_seen_in, n.last_seen_at`, langs, packages, names, seenIn)
	if err != nil {
		return LookupResponse{}, err
	}
	noted, err := pgx.CollectRows(rows, pgx.RowToStructByName[Note])
	if err != nil {
		return LookupResponse{}, err
	}

	rows, err = pool.Query(ctx, `
		SELECT n.lang, n.package, n.name
		  FROM code.api_notes AS n
		  JOIN UNNEST($1::text[], $2::text[], $3::text[]) AS k(lang, package, name)
		    ON n.lang = k.lang AND n.package = k.package AND n.name = k.name
		 WHERE n.summary IS NULL`, langs, packages, names)
	if err != nil {
		return LookupResponse{}, err
	}
	indexed, err := pgx.CollectRows(rows, pgx.RowToStructByName[Key])
	if err != nil {
		return LookupResponse{}, err
	}

	found := map[Key]bool{}
	for _, note := range noted {
		found[Key{note.Lang, note.Package, note.Name}] = true
	}
	for _, key := range indexed {
		found[key] = true
	}
	missing := []Key{}
	for _, key := range keys {
		if !found[key] {
			missing = append(missing, key)
		}
	}
	return LookupResponse{Noted: noted, Indexed: indexed, Missing: missing}, nil
}
