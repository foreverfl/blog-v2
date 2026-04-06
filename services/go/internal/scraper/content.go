package scraper

import (
	_ "embed"
	"encoding/json"
	"fmt"
	"log"
	"net/url"
	"regexp"
	"strings"
)

//go:embed config/static.json
var staticJSON []byte

//go:embed config/blocked.json
var blockedJSON []byte

//go:embed config/dynamic.json
var dynamicJSON []byte

// --- Config types ---

type siteSelector struct {
	Domain    string   `json:"domain"`
	Selectors []string `json:"selectors"`
	Join      bool     `json:"join"`
}

type staticConfig struct {
	Sites    []siteSelector `json:"sites"`
	General  []string       `json:"general"`
	Fallback string         `json:"fallback"`
}

type blockedSite struct {
	Domain string `json:"domain"`
	Reason string `json:"reason"`
}

type dynamicSite struct {
	Domain    string   `json:"domain"`
	Selectors []string `json:"selectors"`
	Note      string   `json:"note"`
}

// --- Global state ---

var (
	staticCfg    staticConfig
	blockedCfg   []blockedSite
	dynamicCfg   []dynamicSite
	zeroWidthRe  = regexp.MustCompile("[\u200B\u00A0]")
	trailingWsRe = regexp.MustCompile(`[ \t]+(\n)`)
	multiNewline = regexp.MustCompile(`\n{2,}`)
)

func init() {
	if err := json.Unmarshal(staticJSON, &staticCfg); err != nil {
		log.Fatalf("Failed to parse static.json: %v", err)
	}
	if err := json.Unmarshal(blockedJSON, &blockedCfg); err != nil {
		log.Fatalf("Failed to parse blocked.json: %v", err)
	}
	if err := json.Unmarshal(dynamicJSON, &dynamicCfg); err != nil {
		log.Fatalf("Failed to parse dynamic.json: %v", err)
	}
}

// --- Shared helpers ---

func extractDomain(rawURL string) string {
	u, err := url.Parse(rawURL)
	if err != nil {
		return ""
	}
	host := strings.ToLower(u.Hostname())
	host = strings.TrimPrefix(host, "www.")
	return host
}

func cleanText(s string) string {
	s = zeroWidthRe.ReplaceAllString(s, "")
	s = trailingWsRe.ReplaceAllString(s, "$1")
	s = multiNewline.ReplaceAllString(s, "\n\n")
	return strings.TrimSpace(s)
}

func isBlocked(domain string) (string, bool) {
	for _, b := range blockedCfg {
		if domain == b.Domain || strings.HasSuffix(domain, "."+b.Domain) {
			return b.Reason, true
		}
	}
	return "", false
}

func findDynamic(domain string) *dynamicSite {
	for i, d := range dynamicCfg {
		if domain == d.Domain || strings.HasSuffix(domain, "."+d.Domain) {
			return &dynamicCfg[i]
		}
	}
	return nil
}

func findSiteSelectors(domain string) *siteSelector {
	for i := range staticCfg.Sites {
		if domain == staticCfg.Sites[i].Domain || strings.HasSuffix(domain, "."+staticCfg.Sites[i].Domain) {
			return &staticCfg.Sites[i]
		}
	}
	return nil
}

// --- Strategy orchestrator ---

// FetchContent retrieves the main text content from a URL.
// Strategy: blocked check → PDF → dynamic (if configured) → static → dynamic fallback
func FetchContent(rawURL string) (string, error) {
	domain := extractDomain(rawURL)

	// 1. Blocked check
	if reason, blocked := isBlocked(domain); blocked {
		return "", fmt.Errorf("blocked site (%s): %s", domain, reason)
	}

	// 2. PDF strategy
	if strings.HasSuffix(strings.ToLower(rawURL), ".pdf") {
		log.Printf("[scraper] PDF detected: url=%s", rawURL)
		return FetchPDFContent(rawURL)
	}

	// 3. Dynamic strategy (configured SPA sites)
	if dyn := findDynamic(domain); dyn != nil {
		log.Printf("[scraper] dynamic site detected: domain=%s note=%s", domain, dyn.Note)
		return FetchDynamic(rawURL, dyn.Selectors)
	}

	// 4. Static strategy
	content, err := FetchStatic(rawURL, domain)
	if err != nil {
		return "", err
	}
	if content != "" {
		return content, nil
	}

	// 5. Dynamic fallback (static returned empty)
	log.Printf("[scraper] static empty, trying dynamic fallback: url=%s", rawURL)
	dynContent, dynErr := FetchDynamic(rawURL, nil)
	if dynErr != nil {
		log.Printf("[scraper] dynamic fallback also failed: url=%s err=%v", rawURL, dynErr)
		return "", nil
	}
	return dynContent, nil
}

// SliceByTokens truncates text to approximately maxTokens tokens.
func SliceByTokens(text string, maxTokens int) string {
	maxChars := maxTokens * 4
	if len(text) <= maxChars {
		return text
	}
	sliced := text[:maxChars]
	if idx := strings.LastIndex(sliced, " "); idx > 0 {
		sliced = sliced[:idx]
	}
	return sliced
}
