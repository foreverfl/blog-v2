package github

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"os"
	"strconv"
	"time"
)

type repo struct {
	FullName    string    `json:"full_name"`
	URL         string    `json:"html_url"`
	Description string    `json:"description"`
	Stars       int       `json:"stargazers_count"`
	Language    string    `json:"language"`
	Topics      []string  `json:"topics"`
	CreatedAt   time.Time `json:"created_at"`
}

// searchRepos returns one page of up to 100 repos matching query, most stars first, or an error naming the HTTP status when GitHub rejects the request.
func searchRepos(query string, page int) ([]repo, error) {
	params := url.Values{}
	params.Set("q", query)
	params.Set("sort", "stars")
	params.Set("order", "desc")
	params.Set("per_page", "100")
	params.Set("page", strconv.Itoa(page))

	request, err := http.NewRequest(http.MethodGet, "https://api.github.com/search/repositories?"+params.Encode(), nil)
	if err != nil {
		return nil, err
	}
	request.Header.Set("Authorization", "Bearer "+os.Getenv("GITHUB_TOKEN"))
	request.Header.Set("Accept", "application/vnd.github+json")
	request.Header.Set("X-GitHub-Api-Version", "2022-11-28")

	response, err := http.DefaultClient.Do(request)
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
		Items []repo `json:"items"`
	}
	if err := json.NewDecoder(response.Body).Decode(&body); err != nil {
		return nil, err
	}
	return body.Items, nil
}
