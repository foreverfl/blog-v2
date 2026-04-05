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
	mutex      sync.Mutex
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

func (manager *StatusManager) Set(key string, phase Phase, total, processed, flushed int, msg string) {
	manager.mutex.Lock()
	defer manager.mutex.Unlock()
	manager.entries[key] = &StatusEntry{
		Phase:     phase,
		Total:     total,
		Processed: processed,
		Flushed:   flushed,
		Message:   msg,
		UpdatedAt: time.Now().UTC().Format(time.RFC3339),
	}
}

func (manager *StatusManager) Get(key string) StatusEntry {
	manager.mutex.Lock()
	defer manager.mutex.Unlock()
	if e, ok := manager.entries[key]; ok {
		return *e
	}
	return StatusEntry{Phase: Idle, Message: fmt.Sprintf("No task found for key: %s", key)}
}

func (manager *StatusManager) IsRunning(key string) bool {
	manager.mutex.Lock()
	defer manager.mutex.Unlock()
	if entry, ok := manager.entries[key]; ok {
		return entry.Phase == Processing || entry.Phase == Flushing
	}
	return false
}

func (manager *StatusManager) IncrProcessed(key string) {
	manager.mutex.Lock()
	defer manager.mutex.Unlock()
	if entry, ok := manager.entries[key]; ok {
		entry.Processed++
		entry.UpdatedAt = time.Now().UTC().Format(time.RFC3339)
	}
}