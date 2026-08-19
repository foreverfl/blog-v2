package main

import (
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"os"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/handler"
	"blog-go-api/internal/middleware"
	oaiservice "blog-go-api/internal/openai"
	"blog-go-api/internal/r2"
	"blog-go-api/internal/redisclient"
	urhandler "blog-go-api/internal/ur/handler"

	"go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracegrpc"
	"go.opentelemetry.io/otel/propagation"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.34.0"
	"go.opentelemetry.io/otel/trace"
)

func main() {
	level := slog.LevelInfo
	if v := os.Getenv("LOG_LEVEL"); v != "" {
		// Bad values keep the info default
		_ = level.UnmarshalText([]byte(v))
	}
	slog.SetDefault(slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{Level: level})))

	ctx := context.Background()
	exporter, err := otlptracegrpc.New(ctx)
	if err != nil {
		slog.Error("failed to create otlp exporter", "err", err)
		os.Exit(1)
	}
	tracerProvider := sdktrace.NewTracerProvider(
		sdktrace.WithBatcher(exporter),
		sdktrace.WithResource(resource.NewWithAttributes(
			semconv.SchemaURL,
			semconv.ServiceName("blog-go-api"),
		)),
	)
	defer func() { _ = tracerProvider.Shutdown(ctx) }()
	otel.SetTracerProvider(tracerProvider)
	// TraceContext propagation so a caller's traceparent header links our spans
	otel.SetTextMapPropagator(propagation.NewCompositeTextMapPropagator(
		propagation.TraceContext{},
		propagation.Baggage{},
	))

	cfg := config.Load()

	hackernewsClient := r2.NewClient(cfg.S3Endpoint, cfg.S3BucketBlogHackernews, cfg.AWSAccessKeyID, cfg.AWSSecretAccessKey, cfg.AWSRegion)
	hackernewsImagesClient := r2.NewClient(cfg.S3Endpoint, cfg.S3BucketBlogHackernewsImages, cfg.AWSAccessKeyID, cfg.AWSSecretAccessKey, cfg.AWSRegion)

	redis, err := redisclient.New(cfg.RedisURL)
	if err != nil {
		slog.Error("failed to connect to redis", "err", err)
		os.Exit(1)
	}
	defer redis.Close()

	openai := oaiservice.NewService(cfg.OpenAIAPIKey)
	statusManager := common.NewStatusManager()

	mux := http.NewServeMux()

	// Rename each request span to its route template (r.Pattern needs Go 1.22+)
	handle := func(pattern string, h http.HandlerFunc) {
		mux.HandleFunc(pattern, func(w http.ResponseWriter, r *http.Request) {
			trace.SpanFromContext(r.Context()).SetName(r.Pattern)
			h(w, r)
		})
	}

	// Health
	handle("GET /health", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "ok")
	})

	// Articles
	articles := handler.ArticlesHandler(cfg, hackernewsClient)
	handle("GET /hackernews", articles)
	handle("GET /hackernews/{date}", articles)

	// Pipeline status (R2-based)
	pipelineStatus := handler.PipelineStatusHandler(cfg, hackernewsClient)
	handle("GET /hackernews/status", pipelineStatus)
	handle("GET /hackernews/status/{date}", pipelineStatus)

	// Fetch content
	fetch := handler.FetchHandler(cfg, hackernewsClient, redis, statusManager)
	handle("POST /hackernews/fetch", fetch)
	handle("POST /hackernews/fetch/{date}", fetch)

	fetchStatus := handler.FetchStatusHandler(cfg, statusManager)
	handle("GET /hackernews/fetch/status", fetchStatus)
	handle("GET /hackernews/fetch/status/{date}", fetchStatus)

	// Summarize
	summarize := handler.SummarizeHandler(cfg, hackernewsClient, redis, openai, statusManager)
	handle("POST /hackernews/summarize", summarize)
	handle("POST /hackernews/summarize/{date}", summarize)

	summarizeStatus := handler.SummarizeStatusHandler(cfg, statusManager)
	handle("GET /hackernews/summarize/status", summarizeStatus)
	handle("GET /hackernews/summarize/status/{date}", summarizeStatus)

	// Translate
	translate := handler.TranslateHandler(cfg, hackernewsClient, redis, openai, statusManager)
	handle("POST /hackernews/translate", translate)
	handle("POST /hackernews/translate/{date}", translate)

	translateStatus := handler.TranslateStatusHandler(cfg, statusManager)
	handle("GET /hackernews/translate/status", translateStatus)
	handle("GET /hackernews/translate/status/{date}", translateStatus)

	// Draw
	draw := handler.DrawHandler(cfg, hackernewsClient, hackernewsImagesClient, openai, statusManager)
	handle("POST /hackernews/draw", draw)
	handle("POST /hackernews/draw/{date}", draw)

	// UR vacant rooms
	handle("GET /ur/listings", urhandler.ListingsHandler(cfg))

	// Inspect
	handle("GET /hackernews/inspect/json", handler.InspectJSONHandler(cfg, hackernewsClient))
	handle("GET /hackernews/inspect/webp", handler.InspectWebpHandler(cfg, hackernewsImagesClient))
	handle("GET /hackernews/inspect/db", handler.InspectDBHandler(cfg))

	slog.Info("go-api listening on :8003")
	slog.Error("server exited", "err", http.ListenAndServe(":8003",
		otelhttp.NewHandler(middleware.Logging(mux), "request")))
	os.Exit(1)
}
