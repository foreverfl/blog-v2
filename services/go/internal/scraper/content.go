package scraper

import (
	"fmt"
	"net/http"
	"regexp"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
)

var (
	zeroWidthRe  = regexp.MustCompile("[\u200B\u00A0]")
	trailingWsRe = regexp.MustCompile(`[ \t]+(\n)`)
	multiNewline = regexp.MustCompile(`\n{2,}`)
)

func cleanText(s string) string {
	s = zeroWidthRe.ReplaceAllString(s, "")
	s = trailingWsRe.ReplaceAllString(s, "$1")
	s = multiNewline.ReplaceAllString(s, "\n\n")
	return strings.TrimSpace(s)
}

// FetchContent retrieves the main text content from a URL using HTTP + goquery.
func FetchContent(url string) (string, error) {
	client := &http.Client{Timeout: 15 * time.Second}

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
	req.Header.Set("Accept-Language", "en-US,en;q=0.9")

	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("HTTP %d for %s", resp.StatusCode, url)
	}

	doc, err := goquery.NewDocumentFromReader(resp.Body)
	if err != nil {
		return "", err
	}

	// Arxiv abstract
	if text := doc.Find("blockquote.abstract").Text(); text != "" {
		text = strings.Replace(text, "Abstract:", "", 1)
		return cleanText(text), nil
	}

	// Economist paragraphs
	var econParts []string
	doc.Find(`p[data-component="paragraph"]`).Each(func(_ int, s *goquery.Selection) {
		if t := strings.TrimSpace(s.Text()); t != "" {
			econParts = append(econParts, t)
		}
	})
	if len(econParts) > 0 {
		return cleanText(strings.Join(econParts, "\n\n")), nil
	}

	// General selectors
	selectors := []string{
		"article",
		"div#content", "div.content",
		"div#post-content", "div.content-area",
		"div#main", "div.main",
		"div.prose", "div.entry",
		"div.bodycopy", "div.node__content",
		"div.essay__content",
		"main",
	}
	for _, sel := range selectors {
		if el := doc.Find(sel).First(); el.Length() > 0 {
			return cleanText(el.Text()), nil
		}
	}

	// Section fallback
	var sectionParts []string
	doc.Find("section").Each(func(_ int, s *goquery.Selection) {
		if t := strings.TrimSpace(s.Text()); t != "" {
			sectionParts = append(sectionParts, t)
		}
	})
	if len(sectionParts) > 0 {
		return cleanText(strings.Join(sectionParts, "\n\n")), nil
	}

	return "", nil
}

// SliceByTokens truncates text to approximately maxTokens tokens.
// Uses a rough heuristic of ~4 characters per token.
func SliceByTokens(text string, maxTokens int) string {
	maxChars := maxTokens * 4
	if len(text) <= maxChars {
		return text
	}
	// Slice at word boundary
	sliced := text[:maxChars]
	if idx := strings.LastIndex(sliced, " "); idx > 0 {
		sliced = sliced[:idx]
	}
	return sliced
}
