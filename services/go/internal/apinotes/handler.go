package apinotes

import (
	"log/slog"
	"net/http"

	"blog-go-api/internal/common"

	"github.com/jackc/pgx/v5/pgxpool"
)

// ListHandler returns every index row and note row; public, no auth.
//
// Request:  GET /code/api-notes
// Response: 200 []Note JSON ordered by lang, package, name / 500 query failed
func ListHandler(pool *pgxpool.Pool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		notes, err := LoadNotes(r.Context(), pool)
		if err != nil {
			slog.Error("api notes query failed", "err", err)
			common.WriteJSON(w, http.StatusInternalServerError, map[string]string{"error": "api notes query failed"})
			return
		}
		common.WriteJSON(w, http.StatusOK, notes)
	}
}
