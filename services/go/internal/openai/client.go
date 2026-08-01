package openai

import (
	"embed"
	"fmt"
	"net/http"

	oai "github.com/sashabaranov/go-openai"
	"go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp"
)

//go:embed prompts/*.md
var promptsFS embed.FS

type Service struct {
	client *oai.Client
}

func NewService(apiKey string) *Service {
	cfg := oai.DefaultConfig(apiKey)
	cfg.HTTPClient = &http.Client{Transport: otelhttp.NewTransport(nil)}
	return &Service{
		client: oai.NewClientWithConfig(cfg),
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