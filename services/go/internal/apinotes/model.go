package apinotes

import "time"

// Note is one code.api_notes row and the JSON the API returns; a nil pointer is SQL NULL and JSON null.
type Note struct {
	Lang         string     `db:"lang" json:"lang"`
	Package      string     `db:"package" json:"package"`
	Name         string     `db:"name" json:"name"`
	MajorVersion *string    `db:"major_version" json:"major_version"`
	Summary      *string    `db:"summary" json:"summary"`
	Example      *string    `db:"example" json:"example"`
	DocURL       string     `db:"doc_url" json:"doc_url"`
	Gotcha       *string    `db:"gotcha" json:"gotcha"`
	SeenCount    int16      `db:"seen_count" json:"seen_count"`
	FirstSeenIn  *string    `db:"first_seen_in" json:"first_seen_in"`
	LastSeenIn   *string    `db:"last_seen_in" json:"last_seen_in"`
	LastSeenAt   *time.Time `db:"last_seen_at" json:"last_seen_at"`
}

// NoteInput is the POST /code/api-notes body: the key, the note text, and where the API was first seen.
type NoteInput struct {
	Lang         string  `json:"lang"`
	Package      string  `json:"package"`
	Name         string  `json:"name"`
	DocURL       string  `json:"doc_url"`
	Summary      string  `json:"summary"`
	Example      string  `json:"example"`
	Gotcha       *string `json:"gotcha"`
	MajorVersion *string `json:"major_version"`
	FirstSeenIn  *string `json:"first_seen_in"`
}

// Key is the (lang, package, name) that identifies one api_notes row.
type Key struct {
	Lang    string `db:"lang" json:"lang"`
	Package string `db:"package" json:"package"`
	Name    string `db:"name" json:"name"`
}

// LookupRequest is the POST /code/api-notes/lookups body: the keys one diff showed and where that diff lives.
type LookupRequest struct {
	SeenIn string `json:"seen_in"`
	Keys   []Key  `json:"keys"`
}

// LookupResponse splits the asked keys into note rows (noted, bumped), index rows (indexed) and keys with no row (missing).
type LookupResponse struct {
	Noted   []Note `json:"noted"`
	Indexed []Key  `json:"indexed"`
	Missing []Key  `json:"missing"`
}
