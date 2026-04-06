package scraper

import (
	"fmt"
	"log"
	"net/http"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
)

// FetchStatic does HTTP GET + goquery-based content extraction.
func FetchStatic(rawURL, domain string) (string, error) {
	client := &http.Client{Timeout: 15 * time.Second}

	req, err := http.NewRequest("GET", rawURL, nil)
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
		return "", fmt.Errorf("HTTP %d for %s", resp.StatusCode, rawURL)
	}

	doc, err := goquery.NewDocumentFromReader(resp.Body)
	if err != nil {
		return "", err
	}

	// 1. Try domain-specific selectors
	if site := findSiteSelectors(domain); site != nil {
		for _, sel := range site.Selectors {
			if site.Join {
				var parts []string
				doc.Find(sel).Each(func(_ int, s *goquery.Selection) {
					if t := strings.TrimSpace(s.Text()); t != "" {
						parts = append(parts, t)
					}
				})
				if len(parts) > 0 {
					log.Printf("[scraper-static] matched domain selector: domain=%s sel=%s", domain, sel)
					return cleanText(strings.Join(parts, "\n\n")), nil
				}
			} else {
				if el := doc.Find(sel).First(); el.Length() > 0 {
					if text := cleanText(el.Text()); text != "" {
						log.Printf("[scraper-static] matched domain selector: domain=%s sel=%s", domain, sel)
						return text, nil
					}
				}
			}
		}
		log.Printf("[scraper-static] domain selectors failed: domain=%s tried=%v", domain, site.Selectors)
	}

	// 2. Try general selectors
	for _, sel := range staticCfg.General {
		if sel == "section" {
			var parts []string
			doc.Find("section").Each(func(_ int, s *goquery.Selection) {
				if t := strings.TrimSpace(s.Text()); t != "" {
					parts = append(parts, t)
				}
			})
			if len(parts) > 0 {
				log.Printf("[scraper-static] matched general selector: sel=section (joined)")
				return cleanText(strings.Join(parts, "\n\n")), nil
			}
		} else {
			if el := doc.Find(sel).First(); el.Length() > 0 {
				if text := cleanText(el.Text()); text != "" {
					log.Printf("[scraper-static] matched general selector: sel=%s", sel)
					return text, nil
				}
			}
		}
	}

	// 3. Fallback to body
	if staticCfg.Fallback != "" {
		if el := doc.Find(staticCfg.Fallback).First(); el.Length() > 0 {
			el.Find("script, style, nav, header, footer, noscript").Remove()
			if text := cleanText(el.Text()); text != "" {
				log.Printf("[scraper-static] matched fallback: sel=%s len=%d", staticCfg.Fallback, len(text))
				return text, nil
			}
		}
	}

	return "", nil
}
