package openai

import (
	"context"
	"fmt"

	oai "github.com/sashabaranov/go-openai"
)

func (s *Service) Summarize(ctx context.Context, text string) (string, error) {
	prompt, err := s.readPrompt("summary.md")
	if err != nil {
		return "", err
	}

	resp, err := s.client.CreateChatCompletion(ctx, oai.ChatCompletionRequest{
		Model: oai.GPT4oMini,
		Messages: []oai.ChatCompletionMessage{
			{Role: oai.ChatMessageRoleSystem, Content: "You are a helpful assistant."},
			{Role: oai.ChatMessageRoleUser, Content: prompt + "\n\n" + text},
		},
		MaxTokens:   15000,
		Temperature: 0.7,
	})
	if err != nil {
		return "", fmt.Errorf("openai summarize: %w", err)
	}

	if len(resp.Choices) == 0 || resp.Choices[0].Message.Content == "" {
		return "", fmt.Errorf("openai summarize: empty response")
	}
	return resp.Choices[0].Message.Content, nil
}
