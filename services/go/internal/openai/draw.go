package openai

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"strings"

	"blog-go-api/internal/model"
	"blog-go-api/internal/r2"

	oai "github.com/sashabaranov/go-openai"
)

// Draw generates an image for the given date using DALL-E 3.
// Returns the generated image URL.
func (s *Service) Draw(ctx context.Context, r2Client *r2.Client, date string) (string, error) {
	keywords, err := s.ExtractKeywords(ctx, r2Client, date)
	if err != nil {
		return "", err
	}

	stylePromptText, err := s.readPrompt("picture-style.md")
	if err != nil {
		return "", err
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

	reqBody := oai.ImageRequest{
		Model:          oai.CreateImageModelDallE3,
		Prompt:         string(promptJSON),
		N:              1,
		Size:           oai.CreateImageSize1024x1024,
		Quality:        oai.CreateImageQualityHD,
		ResponseFormat: oai.CreateImageResponseFormatURL,
	}

	resp, err := s.client.CreateImage(ctx, reqBody)
	if err != nil {
		return "", fmt.Errorf("openai draw: %w", err)
	}

	if len(resp.Data) == 0 || resp.Data[0].URL == "" {
		return "", fmt.Errorf("openai draw: no image URL returned")
	}

	log.Printf("Image generated: %s", resp.Data[0].URL)
	return resp.Data[0].URL, nil
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
