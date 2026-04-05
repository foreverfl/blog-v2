package handler

import (
	"net/http"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/dateutil"
)

func FetchStatusHandler(cfg *config.Config, sm *common.StatusManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}
		date := dateutil.ResolveDate(r.PathValue("date"))
		entry := sm.Get(common.StatusKey("fetch", date))
		common.WriteJSON(w, 200, entry)
	}
}

func SummarizeStatusHandler(cfg *config.Config, sm *common.StatusManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}
		date := dateutil.ResolveDate(r.PathValue("date"))
		entry := sm.Get(common.StatusKey("summarize", date))
		common.WriteJSON(w, 200, entry)
	}
}

func TranslateStatusHandler(cfg *config.Config, sm *common.StatusManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}
		date := dateutil.ResolveDate(r.PathValue("date"))
		lang := r.URL.Query().Get("lang")
		if lang == "" {
			lang = "ko"
		}
		entry := sm.Get(common.StatusKey("translate", lang, date))
		common.WriteJSON(w, 200, entry)
	}
}
