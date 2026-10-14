package apinotes

import (
	"encoding/json"
	"errors"
	"log/slog"
	"net/http"

	"blog-go-api/internal/common"

	"github.com/jackc/pgx/v5/pgxpool"
)

// ListHandler returns every index row and note row, or only one language's when the path has {lang}; public, no auth.
//
// Request:  GET /code/api-notes or GET /code/api-notes/{lang}
// Response: 200 []Note JSON ordered by lang, package, name / 500 query failed
func ListHandler(pool *pgxpool.Pool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		notes, err := LoadNotes(r.Context(), pool, r.PathValue("lang"))
		if err != nil {
			slog.Error("api notes query failed", "err", err)
			common.WriteJSON(w, http.StatusInternalServerError, map[string]string{"error": "api notes query failed"}) // 500
			return
		}
		common.WriteJSON(w, http.StatusOK, notes) // 200
	}
}

// SaveHandler stores one note: a new row, or the text of an index row that has no summary yet; apiSecret required.
//
// Request:  POST /code/api-notes, JSON NoteInput (lang, package, name, doc_url, summary, example required)
// Response: 201 Note new row / 200 Note index row filled / 400 bad body / 401 / 409 note already there / 500 query failed
func SaveHandler(pool *pgxpool.Pool, apiSecret string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, apiSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"}) // 401
			return
		}
		var input NoteInput
		if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": "invalid JSON body"}) // 400
			return
		}
		if input.Lang == "" || input.Package == "" || input.Name == "" ||
			input.DocURL == "" || input.Summary == "" || input.Example == "" {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": "lang, package, name, doc_url, summary, example are required"}) // 400
			return
		}
		note, inserted, err := SaveNote(r.Context(), pool, input)
		if errors.Is(err, ErrNoteExists) {
			common.WriteJSON(w, http.StatusConflict, map[string]string{"error": "note already exists"}) // 409
			return
		}
		if err != nil {
			slog.Error("api note save failed", "err", err)
			common.WriteJSON(w, http.StatusInternalServerError, map[string]string{"error": "api note save failed"}) // 500
			return
		}
		status := http.StatusOK // 200 when an index row was filled in
		if inserted {
			status = http.StatusCreated // 201 when a new row was inserted
		}
		common.WriteJSON(w, status, note)
	}
}

// LookupHandler records that one diff showed the given keys and tells the caller which ones already carry a note; apiSecret required.
//
// Request:  POST /code/api-notes/lookups, JSON LookupRequest (seen_in and at least one key required)
// Response: 200 LookupResponse / 400 bad body / 401 / 500 query failed
func LookupHandler(pool *pgxpool.Pool, apiSecret string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, apiSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"}) // 401
			return
		}
		var input LookupRequest
		if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": "invalid JSON body"}) // 400
			return
		}
		if input.SeenIn == "" || len(input.Keys) == 0 {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": "seen_in and at least one key are required"}) // 400
			return
		}
		result, err := LookupNotes(r.Context(), pool, input.SeenIn, input.Keys)
		if err != nil {
			slog.Error("api note lookup failed", "err", err)
			common.WriteJSON(w, http.StatusInternalServerError, map[string]string{"error": "api note lookup failed"}) // 500
			return
		}
		common.WriteJSON(w, http.StatusOK, result) // 200
	}
}
