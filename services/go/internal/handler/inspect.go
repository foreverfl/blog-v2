package handler

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/r2"
)

var kst = time.FixedZone("KST", 9*60*60)

type inspectResult struct {
	From       string            `json:"from"`
	To         string            `json:"to"`
	Total      int               `json:"total"`
	Found      int               `json:"found"`
	Missing    []string          `json:"missing"`
	Incomplete []incompleteEntry `json:"incomplete,omitempty"`
}

type incompleteEntry struct {
	Date             string   `json:"date"`
	MissingLanguages []string `json:"missing_languages"`
}

// dateRange generates YYMMDD strings from `from` to `to` inclusive.
func dateRange(from, to string) ([]string, error) {
	start, err := time.Parse("060102", from)
	if err != nil {
		return nil, fmt.Errorf("invalid from date %q: %w", from, err)
	}
	end, err := time.Parse("060102", to)
	if err != nil {
		return nil, fmt.Errorf("invalid to date %q: %w", to, err)
	}
	if start.After(end) {
		return nil, fmt.Errorf("from (%s) is after to (%s)", from, to)
	}

	var dates []string
	for d := start; !d.After(end); d = d.AddDate(0, 0, 1) {
		dates = append(dates, d.Format("060102"))
	}
	return dates, nil
}

func todayKST() string {
	return time.Now().In(kst).Format("060102")
}

// InspectJSONHandler checks R2 for missing hackernews/{YYMMDD}.json files.
func InspectJSONHandler(cfg *config.Config, r2c *r2.Client) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		from := r.URL.Query().Get("from")
		to := r.URL.Query().Get("to")
		if from == "" {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": "from is required (YYMMDD)"})
			return
		}
		if to == "" {
			to = todayKST()
		}

		dates, err := dateRange(from, to)
		if err != nil {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": err.Error()})
			return
		}

		var missing []string
		for _, d := range dates {
			exists, err := r2c.Exists(d + ".json")
			if err != nil {
				common.WriteJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
				return
			}
			if !exists {
				missing = append(missing, d)
			}
		}

		common.WriteJSON(w, http.StatusOK, inspectResult{
			From:    from,
			To:      to,
			Total:   len(dates),
			Found:   len(dates) - len(missing),
			Missing: missing,
		})
	}
}

// InspectWebpHandler checks R2 for missing hackernews-images/{YYMMDD}.webp files.
func InspectWebpHandler(cfg *config.Config, r2c *r2.Client) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		from := r.URL.Query().Get("from")
		to := r.URL.Query().Get("to")
		if from == "" {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": "from is required (YYMMDD)"})
			return
		}
		if to == "" {
			to = todayKST()
		}

		dates, err := dateRange(from, to)
		if err != nil {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": err.Error()})
			return
		}

		var missing []string
		for _, d := range dates {
			exists, err := r2c.Exists(d + ".webp")
			if err != nil {
				common.WriteJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
				return
			}
			if !exists {
				missing = append(missing, d)
			}
		}

		common.WriteJSON(w, http.StatusOK, inspectResult{
			From:    from,
			To:      to,
			Total:   len(dates),
			Found:   len(dates) - len(missing),
			Missing: missing,
		})
	}
}

// InspectDBHandler checks the Rust API for missing HN posts and language completeness.
func InspectDBHandler(cfg *config.Config) http.HandlerFunc {
	client := &http.Client{Timeout: 30 * time.Second}

	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.HackernewsSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}

		from := r.URL.Query().Get("from")
		to := r.URL.Query().Get("to")
		if from == "" {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": "from is required (YYMMDD)"})
			return
		}
		if to == "" {
			to = todayKST()
		}

		dates, err := dateRange(from, to)
		if err != nil {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": err.Error()})
			return
		}

		var missing []string
		var incomplete []incompleteEntry

		// fetchOne returns (notFound, missingLangs, err). notFound==true means the post is missing.
		fetchOne := func(d string) (bool, []string, error) {
			url := fmt.Sprintf("%s/posts/trends/hackernews/%s", cfg.RustAPIURL, d)
			resp, err := client.Get(url)
			if err != nil {
				return false, nil, fmt.Errorf("failed to call rust api for %s: %v", d, err)
			}
			defer resp.Body.Close()

			if resp.StatusCode == http.StatusNotFound {
				return true, nil, nil
			}
			if resp.StatusCode != http.StatusOK {
				return false, nil, fmt.Errorf("rust api returned %d for %s", resp.StatusCode, d)
			}

			var post struct {
				Contents []struct {
					Lang string `json:"lang"`
				} `json:"contents"`
			}
			if err := json.NewDecoder(resp.Body).Decode(&post); err != nil {
				return false, nil, fmt.Errorf("failed to decode response for %s: %v", d, err)
			}

			langSet := map[string]bool{}
			for _, c := range post.Contents {
				langSet[c.Lang] = true
			}

			var missingLangs []string
			for _, lang := range []string{"en", "ko", "ja"} {
				if !langSet[lang] {
					missingLangs = append(missingLangs, lang)
				}
			}
			return false, missingLangs, nil
		}

		for _, d := range dates {
			notFound, missingLangs, err := fetchOne(d)
			if err != nil {
				common.WriteJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
				return
			}
			if notFound {
				missing = append(missing, d)
				continue
			}
			if len(missingLangs) > 0 {
				incomplete = append(incomplete, incompleteEntry{
					Date:             d,
					MissingLanguages: missingLangs,
				})
			}
		}

		common.WriteJSON(w, http.StatusOK, inspectResult{
			From:       from,
			To:         to,
			Total:      len(dates),
			Found:      len(dates) - len(missing),
			Missing:    missing,
			Incomplete: incomplete,
		})
	}
}