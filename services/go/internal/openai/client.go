package openai

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"

	oai "github.com/sashabaranov/go-openai"
)

type Service struct {
	client     *oai.Client
	promptsDir string
}

func NewService(apiKey string) *Service {
	return &Service{
		client:     oai.NewClient(apiKey),
		promptsDir: resolvePromptsDir(),
	}
}

// readPrompt reads a prompt file from the local prompts directory.
func (s *Service) readPrompt(filename string) (string, error) {
	path := filepath.Join(s.promptsDir, filename)
	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("read prompt %s: %w", filename, err)
	}
	return string(data), nil
}

func resolvePromptsDir() string {
	_, file, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(file), "..", "..", "prompts")
}
