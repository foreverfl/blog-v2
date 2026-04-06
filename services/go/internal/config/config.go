package config

import (
	"log"
	"os"
)

type Config struct {
	// S3-compatible (Cloudflare R2)
	S3Endpoint                   string
	S3BucketBlogHackernews       string
	S3BucketBlogHackernewsImages string
	AWSAccessKeyID               string
	AWSSecretAccessKey           string
	AWSRegion                    string

	// Redis
	RedisURL string

	// OpenAI
	OpenAIAPIKey string

	// App
	HackernewsSecret string
	RustAPIURL       string
}

func Load() *Config {
	cfg := &Config{
		S3Endpoint:                   getEnv("S3_ENDPOINT", ""),
		S3BucketBlogHackernews:       getEnv("S3_BUCKET_BLOG_HACKERNEWS", ""),
		S3BucketBlogHackernewsImages: getEnv("S3_BUCKET_BLOG_HACKERNEWS_IMAGES", ""),
		AWSAccessKeyID:               getEnv("AWS_ACCESS_KEY_ID", ""),
		AWSSecretAccessKey:           getEnv("AWS_SECRET_ACCESS_KEY", ""),
		AWSRegion:                    getEnv("AWS_REGION", "auto"),
		RedisURL:                     getEnv("REDIS_URL", "redis://localhost:6379"),
		OpenAIAPIKey:                 getEnv("OPENAI_API_KEY", ""),
		HackernewsSecret:             getEnv("HACKERNEWS_SECRET", ""),
		RustAPIURL:                   getEnv("RUST_API_URL", "http://localhost:8002"),
	}

	required := map[string]string{
		"HACKERNEWS_SECRET":                cfg.HackernewsSecret,
		"OPENAI_API_KEY":                   cfg.OpenAIAPIKey,
		"S3_BUCKET_BLOG_HACKERNEWS":        cfg.S3BucketBlogHackernews,
		"S3_BUCKET_BLOG_HACKERNEWS_IMAGES": cfg.S3BucketBlogHackernewsImages,
	}
	for name, val := range required {
		if val == "" {
			log.Fatalf("Required environment variable %s is not set", name)
		}
	}

	return cfg
}

func getEnv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
