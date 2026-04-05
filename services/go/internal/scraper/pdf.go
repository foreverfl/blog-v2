package scraper

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"regexp"
	"strings"
	"time"

	"rsc.io/pdf"
)

var (
	pdfTrailingWs   = regexp.MustCompile(`[ \t]+\n`)
	pdfMultiNewline = regexp.MustCompile(`\n{3,}`)
)

// FetchPDFContent downloads a PDF from the given URL and extracts text (first 10 pages).
func FetchPDFContent(url string) (string, error) {
	client := &http.Client{Timeout: 30 * time.Second}

	resp, err := client.Get(url)
	if err != nil {
		return "", fmt.Errorf("download pdf: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("download pdf: status %d", resp.StatusCode)
	}

	tmp, err := os.CreateTemp("", "hn-pdf-*.pdf")
	if err != nil {
		return "", fmt.Errorf("create temp file: %w", err)
	}
	defer os.Remove(tmp.Name())
	defer tmp.Close()

	if _, err := io.Copy(tmp, resp.Body); err != nil {
		return "", fmt.Errorf("write temp pdf: %w", err)
	}
	tmp.Close()

	return extractPDFText(tmp.Name())
}

func extractPDFText(path string) (string, error) {
	r, err := pdf.Open(path)
	if err != nil {
		return "", fmt.Errorf("open pdf: %w", err)
	}

	totalPages := min(r.NumPage(), 10)

	var buf strings.Builder
	for i := 1; i <= totalPages; i++ {
		p := r.Page(i)
		if p.V.IsNull() {
			continue
		}
		content := p.Content()
		for _, text := range content.Text {
			buf.WriteString(text.S)
			buf.WriteString(" ")
		}
		buf.WriteString("\n\n")
	}

	text := buf.String()
	text = pdfTrailingWs.ReplaceAllString(text, "\n")
	text = pdfMultiNewline.ReplaceAllString(text, "\n\n")
	return strings.TrimSpace(text), nil
}
