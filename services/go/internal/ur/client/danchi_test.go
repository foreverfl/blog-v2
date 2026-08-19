package client

import (
	"context"
	"testing"
	"time"

	"blog-go-api/internal/ur/model"
)

func TestParseUnitCount(t *testing.T) {
	page := []byte("<th scope=\"row\">戸数</th>\n\t<td><p>173</p></td>")
	if count := parseUnitCount(page); count == nil || *count != 173 {
		t.Errorf("unit count = %v", count)
	}
	if parseUnitCount([]byte("<p>no table here</p>")) != nil {
		t.Error("expected nil for a page without the table")
	}
}

func TestParseTotalFloors(t *testing.T) {
	if floors := parseTotalFloors("6階 /8階"); floors == nil || *floors != 8 {
		t.Errorf("floors = %v", floors)
	}
	if parseTotalFloors("6階") != nil {
		t.Error("expected nil without a total part")
	}
}

// Hits the real site: page 0 of Tokyo vacants, then the first danchi's
// detail page and room-detail API.
func TestFetchDanchiDetail(t *testing.T) {
	if testing.Short() {
		t.Skip("network test")
	}
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	body, err := FetchVacantPage(ctx, "13", 0)
	if err != nil {
		t.Fatal(err)
	}
	danchis, err := model.ParseBukkenResult(body)
	if err != nil || len(danchis) == 0 {
		t.Fatalf("no danchi to probe: %v", err)
	}

	detail, err := FetchDanchiDetail(ctx, "13", danchis[0])
	if err != nil {
		t.Fatal(err)
	}
	if detail.UnitCount == nil || *detail.UnitCount < 1 {
		t.Errorf("unit count = %v", detail.UnitCount)
	}
	if detail.Floors == nil || *detail.Floors < 1 {
		t.Errorf("floors = %v", detail.Floors)
	}
	if detail.BuiltYear == nil || *detail.BuiltYear < 1950 || *detail.BuiltYear > time.Now().Year() {
		t.Errorf("built year = %v", detail.BuiltYear)
	}
	t.Logf("%s: units %d, floors %d, built %d", danchis[0].Name, *detail.UnitCount, *detail.Floors, *detail.BuiltYear)
}
