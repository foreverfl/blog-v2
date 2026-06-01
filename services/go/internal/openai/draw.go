package openai

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"strings"

	"blog-go-api/internal/model"
	"blog-go-api/internal/r2"

	oai "github.com/sashabaranov/go-openai"
)

// Draw generates an image for the given date using gpt-image-1.5.
// gpt-image-1.5 always returns base64-encoded image data (no URL), so this
// returns the decoded PNG bytes directly.
func (s *Service) Draw(ctx context.Context, r2Client *r2.Client, date string) ([]byte, error) {
	keywords, err := s.ExtractKeywords(ctx, r2Client, date)
	if err != nil {
		return nil, err
	}

	stylePromptText, err := s.readPrompt("picture-style.md")
	if err != nil {
		return nil, err
	}
	style := parseStylePrompt(stylePromptText)

	whatBytes, _ := json.Marshal(keywords.WhatIsInTheImage)
	bgBytes, _ := json.Marshal(keywords.Background)

	fullPrompt := model.FullImagePrompt{
		Style:             style.Style,
		Mood:              style.Mood,
		Perspective:       style.Perspective,
		Colors:            style.Colors,
		AdditionalEffects: style.AdditionalEffects,
		WhatIsInTheImage:  whatBytes,
		Background:        bgBytes,
	}

	promptJSON, _ := json.Marshal(fullPrompt)

	// gpt-image-1.5 has no go-openai constant yet, so the model id is a literal.
	// It does not accept response_format and always returns b64_json.
	reqBody := oai.ImageRequest{
		Model:   "gpt-image-1.5",
		Prompt:  string(promptJSON),
		N:       1,
		Size:    oai.CreateImageSize1024x1024,
		Quality: oai.CreateImageQualityMedium,
	}

	resp, err := s.client.CreateImage(ctx, reqBody)
	if err != nil {
		return nil, fmt.Errorf("openai draw: %w", err)
	}

	if len(resp.Data) == 0 || resp.Data[0].B64JSON == "" {
		return nil, fmt.Errorf("openai draw: no image data returned")
	}

	imgData, err := base64.StdEncoding.DecodeString(resp.Data[0].B64JSON)
	if err != nil {
		return nil, fmt.Errorf("openai draw: decode b64: %w", err)
	}

	log.Printf("Image generated: %d bytes", len(imgData))
	return imgData, nil
}

func parseStylePrompt(text string) model.StylePrompt {
	result := model.StylePrompt{}
	var current *[]string

	for _, line := range strings.Split(text, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		if strings.HasPrefix(line, "## ") {
			category := strings.ToLower(strings.TrimPrefix(line, "## "))
			switch category {
			case "style":
				current = &result.Style
			case "mood":
				current = &result.Mood
			case "perspective":
				current = &result.Perspective
			case "colors":
				current = &result.Colors
			case "additionaleffects":
				current = &result.AdditionalEffects
			default:
				current = nil
			}
		} else if strings.HasPrefix(line, "-") && current != nil {
			keyword := strings.TrimSpace(strings.TrimPrefix(line, "-"))
			*current = append(*current, keyword)
		}
	}
	return result
}
