package main

import (
	"fmt"
	"log"
	"net/http"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/handler"
	"blog-go-api/internal/middleware"
	oaiservice "blog-go-api/internal/openai"
	"blog-go-api/internal/r2"
	"blog-go-api/internal/redisclient"
)

func main() {
	cfg := config.Load()

	hackernewsClient := r2.NewClient(cfg.S3Endpoint, cfg.S3BucketBlogHackernews, cfg.AWSAccessKeyID, cfg.AWSSecretAccessKey, cfg.AWSRegion)
	hackernewsImagesClient := r2.NewClient(cfg.S3Endpoint, cfg.S3BucketBlogHackernewsImages, cfg.AWSAccessKeyID, cfg.AWSSecretAccessKey, cfg.AWSRegion)

	redis, err := redisclient.New(cfg.RedisURL)
	if err != nil {
		log.Fatalf("Failed to connect to Redis: %v", err)
	}
	defer redis.Close()

	openai := oaiservice.NewService(cfg.OpenAIAPIKey)
	statusManager := common.NewStatusManager()

	mux := http.NewServeMux()

	// Health
	mux.HandleFunc("GET /health", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})

	// Articles
	articles := handler.ArticlesHandler(cfg, hackernewsClient)
	mux.HandleFunc("GET /hackernews", articles)
	mux.HandleFunc("GET /hackernews/{date}", articles)

	// Pipeline status (R2-based)
	pipelineStatus := handler.PipelineStatusHandler(cfg, hackernewsClient)
	mux.HandleFunc("GET /hackernews/status", pipelineStatus)
	mux.HandleFunc("GET /hackernews/status/{date}", pipelineStatus)

	// Fetch content
	fetch := handler.FetchHandler(cfg, hackernewsClient, redis, statusManager)
	mux.HandleFunc("POST /hackernews/fetch", fetch)
	mux.HandleFunc("POST /hackernews/fetch/{date}", fetch)

	fetchStatus := handler.FetchStatusHandler(cfg, statusManager)
	mux.HandleFunc("GET /hackernews/fetch/status", fetchStatus)
	mux.HandleFunc("GET /hackernews/fetch/status/{date}", fetchStatus)

	// Summarize
	summarize := handler.SummarizeHandler(cfg, hackernewsClient, redis, openai, statusManager)
	mux.HandleFunc("POST /hackernews/summarize", summarize)
	mux.HandleFunc("POST /hackernews/summarize/{date}", summarize)

	summarizeStatus := handler.SummarizeStatusHandler(cfg, statusManager)
	mux.HandleFunc("GET /hackernews/summarize/status", summarizeStatus)
	mux.HandleFunc("GET /hackernews/summarize/status/{date}", summarizeStatus)

	// Translate
	translate := handler.TranslateHandler(cfg, hackernewsClient, redis, openai, statusManager)
	mux.HandleFunc("POST /hackernews/translate", translate)
	mux.HandleFunc("POST /hackernews/translate/{date}", translate)

	translateStatus := handler.TranslateStatusHandler(cfg, statusManager)
	mux.HandleFunc("GET /hackernews/translate/status", translateStatus)
	mux.HandleFunc("GET /hackernews/translate/status/{date}", translateStatus)

	// Draw
	draw := handler.DrawHandler(cfg, hackernewsClient, hackernewsImagesClient, openai, statusManager)
	mux.HandleFunc("POST /hackernews/draw", draw)
	mux.HandleFunc("POST /hackernews/draw/{date}", draw)

	// Inspect
	mux.HandleFunc("GET /hackernews/inspect/json", handler.InspectJSONHandler(cfg, hackernewsClient))
	mux.HandleFunc("GET /hackernews/inspect/webp", handler.InspectWebpHandler(cfg, hackernewsImagesClient))
	mux.HandleFunc("GET /hackernews/inspect/db", handler.InspectDBHandler(cfg))

	log.Println("go-api listening on :8003")
	log.Fatal(http.ListenAndServe(":8003", middleware.Logging(mux)))
}
