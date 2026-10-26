package github

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"os"
	"strconv"
	"strings"
	"time"
)

const (
	rounds        = 10
	pagesPerRound = 10
	searchPause   = 2 * time.Second
)

// SearchRepos returns one page of up to 100 repos matching query, most stars first, or an error naming the HTTP status when GitHub rejects the request.
func SearchRepos(query string, page int) ([]Repo, error) {
	params := url.Values{}
	params.Set("q", query)
	params.Set("sort", "stars")
	params.Set("order", "desc")
	params.Set("per_page", "100")
	params.Set("page", strconv.Itoa(page))

	response, err := get("https://api.github.com/search/repositories?" + params.Encode())
	if err != nil {
		return nil, err
	}
	defer response.Body.Close()

	if response.StatusCode == http.StatusForbidden || response.StatusCode == http.StatusTooManyRequests {
		return nil, fmt.Errorf("search %q page %d: rate limited, Retry-After=%s", query, page, response.Header.Get("Retry-After"))
	}
	if response.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("search %q page %d: HTTP %d", query, page, response.StatusCode)
	}

	var body struct {
		Items []Repo `json:"items"`
	}
	if err := json.NewDecoder(response.Body).Decode(&body); err != nil {
		return nil, err
	}
	return body.Items, nil
}

// CollectTopRepos returns up to rounds × pagesPerRound × 100 repos with 1000+ stars matching filter (empty for every repo), most stars first and no repo twice, narrowing the stars range each round because one search only exposes its first 1000 results.
func CollectTopRepos(filter string) ([]Repo, error) {
	var topRepos []Repo
	seenFullNames := map[string]bool{}
	upperStars := 10_000_000

	for round := 1; round <= rounds; round++ {
		query := strings.TrimSpace(fmt.Sprintf("%s stars:1000..%d", filter, upperStars))
		for page := 1; page <= pagesPerRound; page++ {
			pageRepos, err := SearchRepos(query, page)
			if err != nil {
				return nil, err
			}
			for _, pageRepo := range pageRepos {
				if seenFullNames[pageRepo.FullName] {
					continue
				}
				seenFullNames[pageRepo.FullName] = true
				topRepos = append(topRepos, pageRepo)
			}
			if len(pageRepos) == 0 {
				return topRepos, nil
			}
			time.Sleep(searchPause)
		}
		upperStars = topRepos[len(topRepos)-1].Stars
		fmt.Fprintf(os.Stderr, "round %d done, lowest stars %d, repos %d\n", round, upperStars, len(topRepos))
	}
	return topRepos, nil
}
