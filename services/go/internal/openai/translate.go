package openai

import (
	"context"
	"fmt"

	oai "github.com/sashabaranov/go-openai"
)

// Translate translates text to the target language.
// mode is either "title" or "content".
func (s *Service) Translate(ctx context.Context, text, lang, mode string) (string, error) {
	var promptFile string
	if mode == "title" {
		promptFile = fmt.Sprintf("translate-title-%s.md", lang)
	} else {
		promptFile = fmt.Sprintf("translate-%s.md", lang)
	}

	prompt, err := s.readPrompt(promptFile)
	if err != nil {
		return "", err
	}

	resp, err := s.client.CreateChatCompletion(ctx, oai.ChatCompletionRequest{
		Model: oai.GPT4oMini,
		Messages: []oai.ChatCompletionMessage{
			{Role: oai.ChatMessageRoleSystem, Content: "You are a helpful translator."},
			{Role: oai.ChatMessageRoleUser, Content: prompt + "\n\n" + text},
		},
		MaxTokens:   15000,
		Temperature: 0.2,
	})
	if err != nil {
		return "", fmt.Errorf("openai translate: %w", err)
	}

	if len(resp.Choices) == 0 || resp.Choices[0].Message.Content == "" {
		return "", fmt.Errorf("openai translate: empty response")
	}
	return resp.Choices[0].Message.Content, nil
}
