package common

import (
	"fmt"
	"sync"
	"time"
)

type Phase string

const (
	Idle       Phase = "idle"
	Processing Phase = "processing"
	Flushing   Phase = "flushing"
	Done       Phase = "done"
	Error      Phase = "error"
)

type StatusEntry struct {
	Phase     Phase  `json:"phase"`
	Total     int    `json:"total"`
	Processed int    `json:"processed"`
	Flushed   int    `json:"flushed"`
	Message   string `json:"message,omitempty"`
	UpdatedAt string `json:"updatedAt"`
}

type StatusManager struct {
	mu      sync.Mutex
	entries map[string]*StatusEntry
}

func NewStatusManager() *StatusManager {
	return &StatusManager{entries: make(map[string]*StatusEntry)}
}

// StatusKey builds a lookup key like "fetch:2026-04-05" or "translate:ko:2026-04-05".
func StatusKey(parts ...string) string {
	k := parts[0]
	for _, p := range parts[1:] {
		k += ":" + p
	}
	return k
}

func (m *StatusManager) Set(key string, phase Phase, total, processed, flushed int, msg string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.entries[key] = &StatusEntry{
		Phase:     phase,
		Total:     total,
		Processed: processed,
		Flushed:   flushed,
		Message:   msg,
		UpdatedAt: time.Now().UTC().Format(time.RFC3339),
	}
}

func (m *StatusManager) Get(key string) StatusEntry {
	m.mu.Lock()
	defer m.mu.Unlock()
	if e, ok := m.entries[key]; ok {
		return *e
	}
	return StatusEntry{Phase: Idle, Message: fmt.Sprintf("No task found for key: %s", key)}
}

func (m *StatusManager) IncrProcessed(key string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if e, ok := m.entries[key]; ok {
		e.Processed++
		e.UpdatedAt = time.Now().UTC().Format(time.RFC3339)
	}
}