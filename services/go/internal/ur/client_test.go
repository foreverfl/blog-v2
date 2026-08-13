package ur

import (
	"context"
	"testing"
	"time"
)

// Hits the real ur-net API once: Setagaya (112), page 0.
func TestFetchAreaPage(t *testing.T) {
	if testing.Short() {
		t.Skip("network test")
	}
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Second)
	defer cancel()

	body, err := FetchAreaPage(ctx, "112", 0)
	if err != nil {
		t.Fatal(err)
	}
	if len(body) == 0 {
		t.Fatal("empty response body")
	}
	if body[0] != '[' {
		t.Fatalf("expected a JSON array, got: %.60s", body)
	}
}
