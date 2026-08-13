package ur

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"time"

	"go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp"
)

// Undocumented API behind the ur-net search pages; shape captured in
// work/hurl/ur-net. Empty form fields the browser sends are omitted —
// verified the server answers the same without them.
const searchURL = "https://chintai.r6.ur-net.go.jp/chintai/api/bukken/result/bukken_result/"

// FetchAreaPage posts one area-mode search (one Tokyo ward, one page) and
// returns the raw JSON body.
func FetchAreaPage(ctx context.Context, wardCode string, pageIndex int) ([]byte, error) {
	client := &http.Client{Timeout: 15 * time.Second, Transport: otelhttp.NewTransport(nil)}

	form := url.Values{
		"mode":          {"area"},
		"skcs":          {wardCode},
		"block":         {"kanto"},
		"tdfk":          {"13"},
		"orderByField":  {"1"},
		"pageSize":      {"10"},
		"pageIndex":     {strconv.Itoa(pageIndex)},
		"pageIndexRoom": {"0"},
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, searchURL, strings.NewReader(form.Encode()))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
	req.Header.Set("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")

	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("HTTP %d from ur search api", resp.StatusCode)
	}
	return io.ReadAll(resp.Body)
}
