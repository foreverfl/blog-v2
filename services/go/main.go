package main

import (
	"fmt"
	"log"
	"net/http"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/handler"
	oaiservice "blog-go-api/internal/openai"
	"blog-go-api/internal/r2"
	"blog-go-api/internal/redisclient"
)

func main() {
	cfg := config.Load()

	r2c := r2.NewClient(cfg.S3Endpoint, cfg.S3Bucket, cfg.S3Prefix, cfg.AWSAccessKeyID, cfg.AWSSecretAccessKey, cfg.AWSRegion)

	redis, err := redisclient.New(cfg.RedisURL)
	if err != nil {
		log.Fatalf("Failed to connect to Redis: %v", err)
	}
	defer redis.Close()

	openai := oaiservice.NewService(cfg.OpenAIAPIKey)
	sm := common.NewStatusManager()

	mux := http.NewServeMux()

	// Health
	mux.HandleFunc("GET /health", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})

	// Articles
	articles := handler.ArticlesHandler(cfg, r2c)
	mux.HandleFunc("GET /hackernews", articles)
	mux.HandleFunc("GET /hackernews/{date}", articles)

	// Fetch content
	fetch := handler.FetchHandler(cfg, r2c, redis, sm)
	mux.HandleFunc("POST /hackernews/fetch", fetch)
	mux.HandleFunc("POST /hackernews/fetch/{date}", fetch)

	fetchStatus := handler.FetchStatusHandler(cfg, sm)
	mux.HandleFunc("GET /hackernews/fetch/status", fetchStatus)
	mux.HandleFunc("GET /hackernews/fetch/status/{date}", fetchStatus)

	// Summarize
	summarize := handler.SummarizeHandler(cfg, r2c, redis, openai, sm)
	mux.HandleFunc("POST /hackernews/summarize", summarize)
	mux.HandleFunc("POST /hackernews/summarize/{date}", summarize)

	summarizeStatus := handler.SummarizeStatusHandler(cfg, sm)
	mux.HandleFunc("GET /hackernews/summarize/status", summarizeStatus)
	mux.HandleFunc("GET /hackernews/summarize/status/{date}", summarizeStatus)

	// Translate
	translate := handler.TranslateHandler(cfg, r2c, redis, openai, sm)
	mux.HandleFunc("POST /hackernews/translate", translate)
	mux.HandleFunc("POST /hackernews/translate/{date}", translate)

	translateStatus := handler.TranslateStatusHandler(cfg, sm)
	mux.HandleFunc("GET /hackernews/translate/status", translateStatus)
	mux.HandleFunc("GET /hackernews/translate/status/{date}", translateStatus)

	// Draw
	draw := handler.DrawHandler(cfg, r2c, openai)
	mux.HandleFunc("POST /hackernews/draw", draw)
	mux.HandleFunc("POST /hackernews/draw/{date}", draw)

	log.Println("go-api listening on :8003")
	log.Fatal(http.ListenAndServe(":8003", mux))
}
