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

func SummarizeHandler(cfg *config.Config, r2c *r2.Client, redis *redisclient.Client, oai *oaiservice.Service, sm *common.StatusManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		date := dateutil.ResolveDate(r.PathValue("date"))
		key := date + ".json"
		statusKey := common.StatusKey("summarize", date)

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

		// Clear existing en summary keys to prevent conflicts
		deleted, _ := redis.DelByPattern(ctx, "en:*")
		if deleted > 0 {
			log.Printf("Cleared %d existing en:* keys", deleted)
		}

		// Filter articles with content but no summary.en
		var toSummarize []map[string]any
		for _, item := range articles {
			if !common.IsEmpty(item, "content") && common.IsSummaryEmpty(item, "en") {
				toSummarize = append(toSummarize, item)
			}
		}

		total := len(toSummarize)
		log.Printf("toSummarize.length: %d", total)
		sm.Set(statusKey, common.Processing, total, 0, 0, "Summarizing content")

		pool := worker.NewPool(10)
		for idx, item := range toSummarize {
			pool.Submit(func() {
				id, _ := item["id"].(string)
				content, _ := item["content"].(string)
				log.Printf("[%d/%d] Summarizing: %s...", idx+1, total, id)

				summary, err := oai.Summarize(ctx, content)
				if err != nil {
					log.Printf("[%d/%d] Error: %s (%v)", idx+1, total, id, err)
					sm.IncrProcessed(statusKey)
					return
				}

				if err := redis.Set(ctx, "en:"+id, summary, 24*time.Hour); err != nil {
					log.Printf("Redis set error: %v", err)
				}
				sm.IncrProcessed(statusKey)
				log.Printf("[%d/%d] Done: %s", idx+1, total, id)
			})
		}

		common.WriteJSON(w, 200, map[string]any{
			"ok":      true,
			"type":    "summarize",
			"total":   total,
			"message": fmt.Sprintf("Enqueued %d summarize tasks.", total),
		})

		go func() {
			pool.Wait()
			sm.Set(statusKey, common.Flushing, total, total, 0, "Flushing to R2")

			flushed := 0
			for i, item := range articles {
				id, _ := item["id"].(string)
				val, _ := redis.Get(ctx, "en:"+id)
				if val != "" && common.IsSummaryEmpty(item, "en") {
					common.EnsureSummaryMap(articles[i])
					articles[i]["summary"].(map[string]any)["en"] = val
					redis.Del(ctx, "en:"+id)
					flushed++
				}
			}

			if flushed > 0 {
				if err := r2c.PutJSON("hackernews", key, articles); err != nil {
					log.Printf("Failed to write back to R2: %v", err)
					sm.Set(statusKey, common.Error, total, total, flushed, fmt.Sprintf("R2 write failed: %v", err))
					return
				}
			}

			sm.Set(statusKey, common.Done, total, total, flushed, fmt.Sprintf("Flushed %d items to R2.", flushed))
			log.Printf("[summarize] Auto-flush complete: %d items flushed for %s", flushed, date)
		}()
	}
}
