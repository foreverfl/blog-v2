package model

import "encoding/json"

type LocalizedText struct {
	En *string `json:"en"`
	Ko *string `json:"ko"`
	Ja *string `json:"ja"`
}

type HackerNewsArticle struct {
	ID      string        `json:"id"`
	HnID    int           `json:"hnId"`
	Title   LocalizedText `json:"title"`
	Type    string        `json:"type"`
	URL     *string       `json:"url"`
	Score   *int          `json:"score"`
	By      *string       `json:"by"`
	Time    *int64        `json:"time"`
	Content *string       `json:"content"`
	Summary LocalizedText `json:"summary"`
}

// HN Firebase API response
type HNItem struct {
	ID    int    `json:"id"`
	Title string `json:"title"`
	Type  string `json:"type"`
	URL   string `json:"url"`
	Score int    `json:"score"`
	By    string `json:"by"`
	Time  int64  `json:"time"`
	Text  string `json:"text"`
}

// Response types

type CountResponse struct {
	OK      bool            `json:"ok"`
	Date    string          `json:"date,omitempty"`
	Counts  *NullFieldCount `json:"counts,omitempty"`
	Message string          `json:"message,omitempty"`
	Error   string          `json:"error,omitempty"`
}

type NullFieldCount struct {
	NullContentCount   int `json:"nullContentCount"`
	NullSummaryEnCount int `json:"nullSummaryEnCount"`
	NullSummaryJaCount int `json:"nullSummaryJaCount"`
	NullSummaryKoCount int `json:"nullSummaryKoCount"`
}

type FlushRequest struct {
	Type  string `json:"type"`
	Total int    `json:"total"`
	Lang  string `json:"lang,omitempty"`
}

type FlushResponse struct {
	OK        bool         `json:"ok"`
	Type      string       `json:"type"`
	Lang      *string      `json:"lang"`
	Attempted bool         `json:"attempted"`
	CanFlush  bool         `json:"canFlush"`
	Flushed   int          `json:"flushed"`
	Counts    PrefixCounts `json:"counts"`
	TotalKeys int          `json:"totalKeys"`
	Message   string       `json:"message"`
}

type PrefixCounts struct {
	En      int `json:"en"`
	Ja      int `json:"ja"`
	Ko      int `json:"ko"`
	Content int `json:"content"`
}

type RedisCountResponse struct {
	OK       bool     `json:"ok"`
	KeyCount int      `json:"keyCount"`
	Keys     []string `json:"keys"`
	Error    string   `json:"error,omitempty"`
}

type ErrorResponse struct {
	Error string `json:"error"`
}

type OkErrorResponse struct {
	OK    bool   `json:"ok"`
	Error string `json:"error"`
}

// ImageKeywords represents the keyword extraction result for image generation
type ImageKeywords struct {
	WhatIsInTheImage struct {
		Person struct {
			Gender  string  `json:"gender"`
			Age     string  `json:"age"`
			Emotion *string `json:"emotion"`
		} `json:"person"`
		Object *string `json:"object"`
		Action *string `json:"action"`
	} `json:"whatIsInTheImage"`
	Background struct {
		IndoorOutdoor *string `json:"indoorOutdoor"`
		Background    *string `json:"background"`
		TimeOfDay     *string `json:"timeOfDay"`
	} `json:"background"`
}

// StylePrompt holds parsed style prompt categories
type StylePrompt struct {
	Style             []string `json:"style"`
	Mood              []string `json:"mood"`
	Perspective       []string `json:"perspective"`
	Colors            []string `json:"colors"`
	AdditionalEffects []string `json:"additionalEffects"`
}

// FullImagePrompt combines style and keywords for DALL-E
type FullImagePrompt struct {
	Style             []string        `json:"style"`
	Mood              []string        `json:"mood"`
	Perspective       []string        `json:"perspective"`
	Colors            []string        `json:"colors"`
	AdditionalEffects []string        `json:"additionalEffects"`
	WhatIsInTheImage  json.RawMessage `json:"whatisInTheImage"`
	Background        json.RawMessage `json:"background"`
}
