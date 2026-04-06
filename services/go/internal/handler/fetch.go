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
	"blog-go-api/internal/r2"
	"blog-go-api/internal/redisclient"
	"blog-go-api/internal/scraper"
	"blog-go-api/internal/worker"
)

func FetchHandler(cfg *config.Config, r2c *r2.Client, redis *redisclient.Client, statusManager *common.StatusManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		date := dateutil.ResolveDate(r.PathValue("date"))
		key := date + ".json"
		statusKey := common.StatusKey("fetch", date)

		if statusManager.IsRunning(statusKey) {
			common.WriteJSON(w, http.StatusConflict, map[string]any{"ok": false, "error": "Fetch is already running for " + date})
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

		// Clear existing content keys to prevent conflicts
		deleted, _ := redis.DelByPattern(ctx, "content:*")
		if deleted > 0 {
			log.Printf("Cleared %d existing content:* keys", deleted)
		}

		// Filter articles that have URL but no content (fresh=true skips content check)
		fresh := r.URL.Query().Get("fresh") == "true"
		var toFetch []map[string]any
		skippedNoURL := 0
		skippedHasContent := 0
		for _, item := range articles {
			id, _ := item["id"].(string)
			hasURL := !common.IsEmpty(item, "url")
			if !hasURL {
				skippedNoURL++
				log.Printf("[fetch-filter] SKIP no URL: id=%s title=%v", id, item["title"])
				continue
			}
			if fresh {
				toFetch = append(toFetch, item)
			} else {
				if common.IsEmpty(item, "content") {
					toFetch = append(toFetch, item)
				} else {
					skippedHasContent++
				}
			}
		}
		log.Printf("[fetch-filter] total=%d toFetch=%d skippedNoURL=%d skippedHasContent=%d fresh=%v",
			len(articles), len(toFetch), skippedNoURL, skippedHasContent, fresh)

		total := len(toFetch)
		statusManager.Set(statusKey, common.Processing, total, 0, 0, "Fetching content")

		pool := worker.NewPool(10)
		for idx, item := range toFetch {
			pool.Submit(func() {
				id, _ := item["id"].(string)
				url, _ := item["url"].(string)

				content, fetchErr := scraper.FetchContent(url)

				if fetchErr != nil {
					log.Printf("[fetch %d/%d] FAIL id=%s url=%s err=%v", idx+1, total, id, url, fetchErr)
					statusManager.IncrProcessed(statusKey)
					return
				}

				if content == "" {
					log.Printf("[fetch %d/%d] FAIL id=%s url=%s reason=empty", idx+1, total, id, url)
					statusManager.IncrProcessed(statusKey)
					return
				}

				contentLen := len(content)
				content = scraper.SliceByTokens(content, 15000)
				if err := redis.Set(ctx, "content:"+id, content, 24*time.Hour); err != nil {
					log.Printf("[fetch %d/%d] FAIL id=%s reason=redis err=%v", idx+1, total, id, err)
				} else {
					log.Printf("[fetch %d/%d] OK id=%s chars=%d", idx+1, total, id, contentLen)
				}
				statusManager.IncrProcessed(statusKey)
			})
		}

		// Return immediately; auto-flush runs in background
		common.WriteJSON(w, 200, map[string]any{
			"ok":      true,
			"type":    "fetch",
			"total":   total,
			"message": fmt.Sprintf("Enqueued %d fetch tasks.", total),
		})

		go func() {
			pool.Wait()
			statusManager.Set(statusKey, common.Flushing, total, total, 0, "Flushing to R2")

			flushed := 0
			for i, item := range articles {
				id, _ := item["id"].(string)
				val, _ := redis.Get(ctx, "content:"+id)
				if val != "" {
					articles[i]["content"] = val
					redis.Del(ctx, "content:"+id)
					flushed++
				}
			}

			if flushed > 0 {
				if err := r2c.PutJSON("hackernews", key, articles); err != nil {
					log.Printf("Failed to write back to R2: %v", err)
					statusManager.Set(statusKey, common.Error, total, total, flushed, fmt.Sprintf("R2 write failed: %v", err))
					return
				}
			}

			statusManager.Set(statusKey, common.Done, total, total, flushed, fmt.Sprintf("Flushed %d items to R2.", flushed))
			log.Printf("[fetch] Auto-flush complete: %d items flushed for %s", flushed, date)
		}()
	}
}
