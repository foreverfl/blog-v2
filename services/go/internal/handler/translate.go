package handler

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"time"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/dateutil"
	oaiservice "blog-go-api/internal/openai"
	"blog-go-api/internal/r2"
	"blog-go-api/internal/redisclient"
	"blog-go-api/internal/worker"
)

func TranslateHandler(cfg *config.Config, r2c *r2.Client, redis *redisclient.Client, oai *oaiservice.Service, sm *common.StatusManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		lang := r.URL.Query().Get("lang")
		if lang != "ko" && lang != "ja" {
			common.WriteJSON(w, 200, map[string]any{"ok": false, "error": "Invalid language"})
			return
		}

		date := dateutil.ResolveDate(r.PathValue("date"))
		key := date + ".json"
		statusKey := common.StatusKey("translate", lang, date)

		if sm.IsRunning(statusKey) {
			common.WriteJSON(w, http.StatusConflict, map[string]any{"ok": false, "error": "Translate (" + lang + ") is already running for " + date})
			return
		}

		articles, err := r2c.GetArticles("hackernews", key)
		if err != nil || articles == nil {
			common.WriteJSON(w, 200, map[string]any{"ok": false, "error": "File not found"})
			return
		}

		ctx := context.Background()
		if err := redis.Ping(ctx); err != nil {
			common.WriteJSON(w, 200, map[string]any{"ok": false, "error": "Redis connection failed"})
			return
		}

		// Clear existing keys for the target language to prevent conflicts
		deleted1, _ := redis.DelByPattern(ctx, lang+":summary:*")
		deleted2, _ := redis.DelByPattern(ctx, lang+":title:*")
		if deleted1+deleted2 > 0 {
			log.Printf("Cleared %d existing %s:* keys", deleted1+deleted2, lang)
		}

		// Filter articles with summary.en but missing target lang summary (fresh=true skips lang check)
		fresh := r.URL.Query().Get("fresh") == "true"
		var toTranslate []map[string]any
		for _, item := range articles {
			hasSummaryEn := !common.IsSummaryEmpty(item, "en")
			if fresh {
				if hasSummaryEn {
					toTranslate = append(toTranslate, item)
				}
			} else {
				if hasSummaryEn && common.IsSummaryEmpty(item, lang) {
					toTranslate = append(toTranslate, item)
				}
			}
		}

		total := len(toTranslate)
		log.Printf("toTranslate[%s].length: %d", lang, total)
		sm.Set(statusKey, common.Processing, total, 0, 0, fmt.Sprintf("Translating to %s", lang))

		pool := worker.NewPool(10)
		for idx, item := range toTranslate {
			pool.Submit(func() {
				id, _ := item["id"].(string)

				// Translate title
				titleMap, _ := item["title"].(map[string]any)
				if titleMap != nil {
					if titleEn, ok := titleMap["en"].(string); ok && titleEn != "" {
						existing, _ := titleMap[lang].(string)
						if fresh || existing == "" {
							log.Printf("[%s][title][%d/%d] Translating title: %s...", lang, idx+1, total, id)
							translated, err := oai.Translate(ctx, titleEn, lang, "title")
							if err != nil {
								log.Printf("[%s][title][%d/%d] Error: %s (%v)", lang, idx+1, total, id, err)
							} else {
								redis.Set(ctx, fmt.Sprintf("%s:title:%s", lang, id), translated, 24*time.Hour)
								log.Printf("[%s][title][%d/%d] Done: %s", lang, idx+1, total, id)
							}
						}
					}
				}

				// Translate summary
				summaryMap, _ := item["summary"].(map[string]any)
				if summaryMap != nil {
					if summaryEn, ok := summaryMap["en"].(string); ok && summaryEn != "" {
						existing, _ := summaryMap[lang].(string)
						if fresh || existing == "" {
							log.Printf("[%s][summary][%d/%d] Translating summary: %s...", lang, idx+1, total, id)
							translated, err := oai.Translate(ctx, summaryEn, lang, "content")
							if err != nil {
								log.Printf("[%s][summary][%d/%d] Error: %s (%v)", lang, idx+1, total, id, err)
							} else {
								redis.Set(ctx, fmt.Sprintf("%s:summary:%s", lang, id), translated, 24*time.Hour)
								log.Printf("[%s][summary][%d/%d] Done: %s", lang, idx+1, total, id)
							}
						}
					}
				}

				sm.IncrProcessed(statusKey)
			})
		}

		common.WriteJSON(w, 200, map[string]any{
			"ok":      true,
			"type":    "translate",
			"lang":    lang,
			"total":   total,
			"message": fmt.Sprintf("Enqueued %d %s translation tasks.", total, lang),
		})

		go func() {
			pool.Wait()
			sm.Set(statusKey, common.Flushing, total, total, 0, "Flushing to R2")

			flushedTitles, flushedSummaries := 0, 0
			for i, item := range articles {
				id, _ := item["id"].(string)

				summaryKey := fmt.Sprintf("%s:summary:%s", lang, id)
				if val, _ := redis.Get(ctx, summaryKey); val != "" && (fresh || common.IsSummaryEmpty(item, lang)) {
					common.EnsureSummaryMap(articles[i])
					articles[i]["summary"].(map[string]any)[lang] = val
					redis.Del(ctx, summaryKey)
					flushedSummaries++
				}

				titleKey := fmt.Sprintf("%s:title:%s", lang, id)
				if val, _ := redis.Get(ctx, titleKey); val != "" && (fresh || common.IsTitleEmpty(item, lang)) {
					common.EnsureTitleMap(articles[i])
					articles[i]["title"].(map[string]any)[lang] = val
					redis.Del(ctx, titleKey)
					flushedTitles++
				}
			}

			flushed := flushedTitles + flushedSummaries
			if flushed > 0 {
				if err := r2c.PutJSON("hackernews", key, articles); err != nil {
					log.Printf("Failed to write back to R2: %v", err)
					sm.Set(statusKey, common.Error, total, total, flushed, fmt.Sprintf("R2 write failed: %v", err))
					return
				}
			}

			detail := map[string]int{"titles": flushedTitles, "summaries": flushedSummaries}
			entry := sm.Get(statusKey)
			entry.Phase = common.Done
			entry.Flushed = flushed
			entry.FlushedDetail = detail
			entry.Message = fmt.Sprintf("Flushed %d titles + %d summaries to R2.", flushedTitles, flushedSummaries)
			sm.SetEntry(statusKey, &entry)
			log.Printf("[translate:%s] Auto-flush complete: %d titles + %d summaries flushed for %s", lang, flushedTitles, flushedSummaries, date)
		}()
	}
}
