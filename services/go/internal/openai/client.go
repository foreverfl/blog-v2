package openai

import (
	"embed"
	"fmt"

	oai "github.com/sashabaranov/go-openai"
)

//go:embed prompts/*.md
var promptsFS embed.FS

type Service struct {
	client *oai.Client
}

func NewService(apiKey string) *Service {
	return &Service{
		client: oai.NewClient(apiKey),
	}
}

// readPrompt reads a prompt file from the embedded prompts directory.
func (s *Service) readPrompt(filename string) (string, error) {
	data, err := promptsFS.ReadFile("prompts/" + filename)
	if err != nil {
		return "", fmt.Errorf("read prompt %s: %w", filename, err)
	}
	return string(data), nil
}