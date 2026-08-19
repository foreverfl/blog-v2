package client

import (
	"context"
	"strconv"
	"strings"
	"testing"
	"time"
)

// Pages through the real API for Tokyo (13) and Kanagawa (14), then checks
// the collected room total against the API's own allCount.
func TestCollectVacant(t *testing.T) {
	if testing.Short() {
		t.Skip("network test")
	}
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Minute)
	defer cancel()

	total := 0
	// JIS prefecture codes: 13 = Tokyo, 14 = Kanagawa.
	for _, prefCode := range []string{"13", "14"} {
		danchis, err := CollectVacant(ctx, prefCode)
		if err != nil {
			t.Fatal(err)
		}
		if len(danchis) == 0 {
			t.Fatalf("pref %s: no danchi collected", prefCode)
		}

		rooms := 0
		for _, danchi := range danchis {
			rooms += len(danchi.Rooms)
		}
		// allCount comes as "107" or "1,234" — digits only for comparison.
		want, err := strconv.Atoi(strings.ReplaceAll(danchis[0].AllCount, ",", ""))
		if err != nil {
			t.Fatalf("pref %s: bad allCount %q", prefCode, danchis[0].AllCount)
		}
		if rooms != want {
			t.Errorf("pref %s: collected %d rooms, api allCount says %d", prefCode, rooms, want)
		}
		t.Logf("pref %s: danchi %d, rooms %d (allCount %d)", prefCode, len(danchis), rooms, want)
		total += rooms
	}
	if total == 0 {
		t.Fatal("no vacant rooms collected at all")
	}
}
