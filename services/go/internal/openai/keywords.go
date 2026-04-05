package openai

import (
	"context"
	"encoding/json"
	"fmt"
	"log"

	"blog-go-api/internal/model"
	"blog-go-api/internal/r2"

	oai "github.com/sashabaranov/go-openai"
)

// ExtractKeywords extracts image keywords from the top article's summary.
func (s *Service) ExtractKeywords(ctx context.Context, r2Client *r2.Client, date string) (*model.ImageKeywords, error) {
	key := date + ".json"
	articles, err := r2Client.GetArticles("hackernews", key)
	if err != nil {
		return nil, fmt.Errorf("get articles: %w", err)
	}
	if articles == nil {
		return nil, fmt.Errorf("no articles for date %s", date)
	}

	// Find first article with a non-empty summary.en
	var summary string
	for i, item := range articles {
		if s, ok := item["summary"].(map[string]any); ok {
			if en, ok := s["en"].(string); ok && en != "" {
				summary = en
				break
			}
		}
		log.Printf("[keywords] Skipping item[%d] - summary.en missing", i)
	}
	if summary == "" {
		return nil, fmt.Errorf("no valid item with summary.en found")
	}
	log.Printf("Extracted summary: %s", summary)

	promptText, err := s.readPrompt("picture-keywords.md")
	if err != nil {
		return nil, err
	}

	jsonStructure := `{
  "whatIsInTheImage": {
    "person": { "gender": "female", "age": "teenager", "emotion": "string or null" },
    "object": "string or null",
    "action": "string or null"
  },
  "background": {
    "indoorOutdoor": "string or null",
    "background": "string or null",
    "timeOfDay": "string or null"
  }
}`

	systemContent := promptText +
		"\n\nYou must respond with a valid JSON object with this exact structure:\n" +
		jsonStructure +
		"\n\nIMPORTANT: The 'gender' field MUST always be 'female' and the 'age' field MUST always be 'teenager'. Do not use any other values for these fields."

	resp, err := s.client.CreateChatCompletion(ctx, oai.ChatCompletionRequest{
		Model: oai.GPT4oMini,
		Messages: []oai.ChatCompletionMessage{
			{Role: oai.ChatMessageRoleSystem, Content: systemContent},
			{Role: oai.ChatMessageRoleUser, Content: summary},
		},
		Temperature:    1.0,
		ResponseFormat: &oai.ChatCompletionResponseFormat{Type: oai.ChatCompletionResponseFormatTypeJSONObject},
	})
	if err != nil {
		return nil, fmt.Errorf("openai keywords: %w", err)
	}

	content := resp.Choices[0].Message.Content
	log.Printf("OpenAI keywords response: %s", content)

	var keywords model.ImageKeywords
	if err := json.Unmarshal([]byte(content), &keywords); err != nil {
		return nil, fmt.Errorf("parse keywords: %w", err)
	}
	return &keywords, nil
}
