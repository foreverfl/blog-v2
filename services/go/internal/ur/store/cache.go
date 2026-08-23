package store

import (
	"encoding/json"
	"errors"
	"io/fs"
	"os"
	"sync"

	"blog-go-api/internal/ur/model"
)

// Cache persists danchi attributes as one JSON file (danchi key → detail),
// so a danchi is crawled once and reused across runs.
type Cache struct {
	path    string
	mu      sync.Mutex
	entries map[string]model.DanchiDetail
}

// LoadCache reads the cache file; a missing file starts an empty cache.
func LoadCache(path string) (*Cache, error) {
	cache := &Cache{path: path, entries: map[string]model.DanchiDetail{}}

	data, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return cache, nil
	}
	if err != nil {
		return nil, err
	}
	if err := json.Unmarshal(data, &cache.entries); err != nil {
		return nil, err
	}
	return cache, nil
}

// GetOrFetch returns the cached entry; on a miss it calls fetch and persists
// the result immediately, so a crash mid-crawl keeps what was already fetched.
func (c *Cache) GetOrFetch(key string, fetch func() (model.DanchiDetail, error)) (model.DanchiDetail, error) {
	c.mu.Lock()
	defer c.mu.Unlock()

	if detail, ok := c.entries[key]; ok {
		return detail, nil
	}
	detail, err := fetch()
	if err != nil {
		return detail, err
	}
	c.entries[key] = detail
	return detail, c.save()
}

func (c *Cache) save() error {
	data, err := json.MarshalIndent(c.entries, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(c.path, data, 0o644)
}
