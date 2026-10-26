package github

import (
	"net/http"
	"os"
)

// get sends an authenticated GET to a GitHub REST URL and returns the response, or the transport error when the request never completed.
func get(rawURL string) (*http.Response, error) {
	request, err := http.NewRequest(http.MethodGet, rawURL, nil)
	if err != nil {
		return nil, err
	}
	request.Header.Set("Authorization", "Bearer "+os.Getenv("GITHUB_TOKEN"))
	request.Header.Set("Accept", "application/vnd.github+json")
	request.Header.Set("X-GitHub-Api-Version", "2022-11-28")
	return http.DefaultClient.Do(request)
}
