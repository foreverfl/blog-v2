package scraper

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/chromedp/chromedp"
)

// FetchDynamic uses a headless browser to fetch JS-rendered content.
// It tries each selector in order and returns the first non-empty match.
func FetchDynamic(rawURL string, selectors []string) (string, error) {
	opts := append(chromedp.DefaultExecAllocatorOptions[:],
		chromedp.Flag("no-sandbox", true),
		chromedp.Flag("disable-setuid-sandbox", true),
	)
	allocCtx, allocCancel := chromedp.NewExecAllocator(context.Background(), opts...)
	defer allocCancel()

	ctx, cancel := chromedp.NewContext(allocCtx)
	defer cancel()

	ctx, cancel = context.WithTimeout(ctx, 30*time.Second)
	defer cancel()

	var body string
	// Navigate and wait for the page to be ready
	if err := chromedp.Run(ctx,
		chromedp.Navigate(rawURL),
		chromedp.WaitReady("body"),
		chromedp.Sleep(2*time.Second),
	); err != nil {
		return "", fmt.Errorf("chromedp navigate: %w", err)
	}

	// Try each selector in order
	for _, sel := range selectors {
		var text string
		err := chromedp.Run(ctx,
			chromedp.TextContent(sel, &text, chromedp.ByQuery),
		)
		if err == nil && text != "" {
			log.Printf("[scraper-dynamic] matched selector: url=%s sel=%s len=%d", rawURL, sel, len(text))
			return cleanText(text), nil
		}
	}

	// Fallback: get entire body text
	if err := chromedp.Run(ctx,
		chromedp.TextContent("body", &body, chromedp.ByQuery),
	); err != nil {
		return "", fmt.Errorf("chromedp body: %w", err)
	}

	if body != "" {
		log.Printf("[scraper-dynamic] fallback body: url=%s len=%d", rawURL, len(body))
		return cleanText(body), nil
	}

	return "", nil
}
