package config

import "os"

type Config struct {
	// S3-compatible (Cloudflare R2)
	S3Endpoint         string
	S3Bucket           string
	S3Prefix           string
	AWSAccessKeyID     string
	AWSSecretAccessKey string
	AWSRegion          string

	// Redis
	RedisURL string

	// OpenAI
	OpenAIAPIKey string

	// App
	HackernewsSecret string
}

func Load() *Config {
	return &Config{
		S3Endpoint:         getEnv("S3_ENDPOINT", ""),
		S3Bucket:           getEnv("S3_BUCKET", ""),
		S3Prefix:           getEnv("S3_PREFIX", ""),
		AWSAccessKeyID:     getEnv("AWS_ACCESS_KEY_ID", ""),
		AWSSecretAccessKey: getEnv("AWS_SECRET_ACCESS_KEY", ""),
		AWSRegion:          getEnv("AWS_REGION", "auto"),
		RedisURL:           getEnv("REDIS_URL", "redis://localhost:6379"),
		OpenAIAPIKey:       getEnv("OPENAI_API_KEY", ""),
		HackernewsSecret:   getEnv("HACKERNEWS_SECRET", ""),
	}
}

func getEnv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
