package handler

import (
	"context"
	"io"
	"log"
	"net/http"
	"time"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/dateutil"
	oaiservice "blog-go-api/internal/openai"
	"blog-go-api/internal/r2"
	"blog-go-api/internal/worker"
)

func DrawHandler(cfg *config.Config, r2c *r2.Client, oai *oaiservice.Service, sm *common.StatusManager) http.HandlerFunc {
	drawPool := worker.NewPool(1) // concurrency 1

	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		date := dateutil.ResolveDate(r.PathValue("date"))
		key := date + ".json"
		statusKey := common.StatusKey("draw", date)

		if sm.IsRunning(statusKey) {
			common.WriteJSON(w, http.StatusConflict, map[string]any{"ok": false, "error": "Draw is already running for " + date})
			return
		}

		articles, err := r2c.GetArticles("hackernews", key)
		if err != nil || articles == nil {
			common.WriteJSON(w, 200, map[string]any{"ok": false, "error": "No data found for the given date in R2"})
			return
		}

		// Find top-scored article with non-empty summary.en
		var topItem map[string]any
		topScore := -1
		for _, item := range articles {
			if !common.IsSummaryEmpty(item, "en") {
				score := 0
				if s, ok := item["score"].(float64); ok {
					score = int(s)
				}
				if score > topScore {
					topScore = score
					topItem = item
				}
			}
		}

		if topItem == nil {
			common.WriteJSON(w, 200, map[string]any{"ok": false, "error": "No valid item with non-empty summary.en found"})
			return
		}

		sm.Set(statusKey, common.Processing, 1, 0, 0, "Generating image")

		drawPool.Submit(func() {
			ctx := context.Background()
			imageURL, err := oai.Draw(ctx, r2c, date)
			if err != nil {
				log.Printf("Failed to generate image: %v", err)
				sm.Set(statusKey, common.Error, 1, 1, 0, err.Error())
				return
			}

			// Download the image
			client := &http.Client{Timeout: 30 * time.Second}
			resp, err := client.Get(imageURL)
			if err != nil {
				log.Printf("Failed to download image: %v", err)
				sm.Set(statusKey, common.Error, 1, 1, 0, err.Error())
				return
			}
			defer resp.Body.Close()

			imgData, err := io.ReadAll(resp.Body)
			if err != nil {
				log.Printf("Failed to read image data: %v", err)
				sm.Set(statusKey, common.Error, 1, 1, 0, err.Error())
				return
			}

			// Upload to R2 as PNG
			imgKey := date + ".png"
			if err := r2c.PutBytes("hackernews-images", imgKey, imgData, "image/png"); err != nil {
				log.Printf("Failed to upload image to R2: %v", err)
				sm.Set(statusKey, common.Error, 1, 1, 0, err.Error())
				return
			}
			log.Printf("Uploaded image to R2: hackernews-images/%s", imgKey)
			sm.Set(statusKey, common.Done, 1, 1, 1, "Image uploaded")
		})

		common.WriteJSON(w, 200, map[string]any{
			"ok":      true,
			"message": "Image generation request queued.",
		})
	}
}
