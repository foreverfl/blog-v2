package store

import (
	"errors"
	"path/filepath"
	"testing"

	"blog-go-api/internal/ur/model"
)

func TestCacheSecondRunFetchesNothing(t *testing.T) {
	path := filepath.Join(t.TempDir(), "danchi_cache.json")
	units := 173
	fetchCalls := 0
	fetch := func() (model.DanchiDetail, error) {
		fetchCalls++
		return model.DanchiDetail{UnitCount: &units}, nil
	}

	// First run: miss → fetch once and persist.
	cache, err := LoadCache(path)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := cache.GetOrFetch("20_6560", fetch); err != nil {
		t.Fatal(err)
	}
	if _, err := cache.GetOrFetch("20_6560", fetch); err != nil {
		t.Fatal(err)
	}
	if fetchCalls != 1 {
		t.Fatalf("first run: fetch called %d times, want 1", fetchCalls)
	}

	// Second run: reload from disk → no fetch at all.
	reloaded, err := LoadCache(path)
	if err != nil {
		t.Fatal(err)
	}
	fetchCalls = 0
	detail, err := reloaded.GetOrFetch("20_6560", fetch)
	if err != nil {
		t.Fatal(err)
	}
	if fetchCalls != 0 {
		t.Fatalf("second run: fetch called %d times, want 0", fetchCalls)
	}
	if detail.UnitCount == nil || *detail.UnitCount != 173 {
		t.Errorf("unit count survived reload wrong: %v", detail.UnitCount)
	}
}

func TestCacheFetchErrorIsNotCached(t *testing.T) {
	cache, err := LoadCache(filepath.Join(t.TempDir(), "cache.json"))
	if err != nil {
		t.Fatal(err)
	}
	failing := func() (model.DanchiDetail, error) {
		return model.DanchiDetail{}, errTest
	}
	if _, err := cache.GetOrFetch("20_1180", failing); err == nil {
		t.Fatal("expected the fetch error to surface")
	}
	fetchCalls := 0
	counting := func() (model.DanchiDetail, error) {
		fetchCalls++
		return model.DanchiDetail{}, nil
	}
	if _, err := cache.GetOrFetch("20_1180", counting); err != nil {
		t.Fatal(err)
	}
	if fetchCalls != 1 {
		t.Error("a failed fetch must stay a miss and be retried")
	}
}

var errTest = errors.New("fetch failed")
