package handler

import (
	"net/http"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/dateutil"
	"blog-go-api/internal/r2"
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

func PipelineStatusHandler(cfg *config.Config, r2c *r2.Client) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		date := dateutil.ResolveDate(r.PathValue("date"))
		key := date + ".json"

		articles, err := r2c.GetArticles("hackernews", key)
		if err != nil || articles == nil {
			common.WriteJSON(w, 200, map[string]any{"ok": false, "error": "No data found"})
			return
		}

		total := len(articles)
		fetched, summarized, translatedKo, translatedJa := 0, 0, 0, 0

		for _, item := range articles {
			if !common.IsEmpty(item, "content") {
				fetched++
			}
			if !common.IsSummaryEmpty(item, "en") {
				summarized++
			}
			if !common.IsTitleEmpty(item, "ko") && !common.IsSummaryEmpty(item, "ko") {
				translatedKo++
			}
			if !common.IsTitleEmpty(item, "ja") && !common.IsSummaryEmpty(item, "ja") {
				translatedJa++
			}
		}

		common.WriteJSON(w, 200, map[string]any{
			"date":  date,
			"total": total,
			"fetch":        map[string]any{"done": fetched, "remaining": max(0, total-fetched)},
			"summarize":    map[string]any{"done": summarized, "remaining": max(0, fetched-summarized)},
			"translate_ko": map[string]any{"done": translatedKo, "remaining": max(0, summarized-translatedKo)},
			"translate_ja": map[string]any{"done": translatedJa, "remaining": max(0, summarized-translatedJa)},
		})
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
