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
