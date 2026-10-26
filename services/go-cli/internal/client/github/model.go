package github

import "time"

type Repo struct {
	FullName    string    `json:"full_name"`
	URL         string    `json:"html_url"`
	Description string    `json:"description"`
	Stars       int       `json:"stargazers_count"`
	Language    string    `json:"language"`
	Topics      []string  `json:"topics"`
	CreatedAt   time.Time `json:"created_at"`
}
