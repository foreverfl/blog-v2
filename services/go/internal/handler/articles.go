package handler

import (
	"crypto/sha256"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"regexp"
	"strings"
	"sync"
	"time"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/dateutil"
	"blog-go-api/internal/r2"
)

const hnAPIBase = "https://hacker-news.firebaseio.com/v0"

var hnTitlePrefixRe = regexp.MustCompile(`(?i)^\s*(show|ask|launch)\s+hn\s*:\s*`)

func generateID(title string) string {
	h := sha256.Sum256([]byte(title))
	return fmt.Sprintf("%x", h[:8])
}

func cleanHNTitle(title string) string {
	return strings.TrimSpace(hnTitlePrefixRe.ReplaceAllString(title, ""))
}

func ArticlesHandler(cfg *config.Config, r2c *r2.Client) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		date := dateutil.ResolveDate(r.PathValue("date"))
		key := date + ".json"
		fresh := r.URL.Query().Get("fresh") == "true"

		if !fresh {
			existing, err := r2c.Get("hackernews", key)
			if err != nil {
				common.WriteJSON(w, 500, map[string]string{"error": "Failed to fetch from R2"})
				return
			}
			if existing != nil {
				log.Printf("Skipping fetch: hackernews/%s already exists in R2", key)
				w.Header().Set("Content-Type", "application/json")
				w.Write(existing)
				return
			}
		} else {
			log.Printf("Fresh mode: overwriting hackernews/%s", key)
		}

		// Fetch top 100 story IDs from HN
		log.Println("Fetching new data from HackerNews API...")
		httpClient := &http.Client{Timeout: 30 * time.Second}
		resp, err := httpClient.Get(hnAPIBase + "/topstories.json")
		if err != nil {
			common.WriteJSON(w, 500, map[string]string{"error": "Failed to fetch top stories"})
			return
		}
		defer resp.Body.Close()

		var storyIDs []int
		if err := json.NewDecoder(resp.Body).Decode(&storyIDs); err != nil {
			common.WriteJSON(w, 500, map[string]string{"error": "Failed to parse top stories"})
			return
		}
		if len(storyIDs) > 100 {
			storyIDs = storyIDs[:100]
		}

		type hnItem struct {
			ID    int    `json:"id"`
			Title string `json:"title"`
			Type  string `json:"type"`
			URL   string `json:"url"`
			Score int    `json:"score"`
			By    string `json:"by"`
			Time  int64  `json:"time"`
			Text  string `json:"text"`
		}

		articles := make([]map[string]any, len(storyIDs))
		var mu sync.Mutex
		var wg sync.WaitGroup

		for i, sid := range storyIDs {
			wg.Add(1)
			go func(idx, id int) {
				defer wg.Done()
				url := fmt.Sprintf("%s/item/%d.json", hnAPIBase, id)
				resp, err := httpClient.Get(url)
				if err != nil {
					log.Printf("Failed to fetch item %d: %v", id, err)
					return
				}
				defer resp.Body.Close()

				var item hnItem
				if err := json.NewDecoder(resp.Body).Decode(&item); err != nil {
					log.Printf("Failed to parse item %d: %v", id, err)
					return
				}

				cleaned := cleanHNTitle(item.Title)
				articleID := generateID(item.Title)

				article := map[string]any{
					"id":   articleID,
					"hnId": item.ID,
					"title": map[string]any{
						"en": cleaned,
						"ko": nil,
						"ja": nil,
					},
					"type":  item.Type,
					"url":   nilIfEmpty(item.URL),
					"score": nilIfZero(item.Score),
					"by":    nilIfEmpty(item.By),
					"time":  nilIfZeroInt64(item.Time),
					"content": nilIfEmpty(item.Text),
					"summary": map[string]any{
						"en": nil,
						"ko": nil,
						"ja": nil,
					},
				}

				mu.Lock()
				articles[idx] = article
				mu.Unlock()
			}(i, sid)
		}
		wg.Wait()

		// Filter out nil entries (failed fetches)
		var result []map[string]any
		for _, a := range articles {
			if a != nil {
				result = append(result, a)
			}
		}

		if err := r2c.PutJSON("hackernews", key, result); err != nil {
			log.Printf("Failed to upload to R2: %v", err)
		} else {
			log.Printf("Uploaded to R2: hackernews/%s", key)
		}

		common.WriteJSON(w, 200, result)
	}
}

func nilIfEmpty(s string) any {
	if s == "" {
		return nil
	}
	return s
}

func nilIfZero(n int) any {
	if n == 0 {
		return nil
	}
	return n
}

func nilIfZeroInt64(n int64) any {
	if n == 0 {
		return nil
	}
	return n
}

