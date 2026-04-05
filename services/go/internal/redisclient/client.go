package redisclient

import (
	"context"
	"time"

	"github.com/redis/go-redis/v9"
)

type Client struct {
	rdb *redis.Client
}

func New(redisURL string) (*Client, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	opts.DialTimeout = 3 * time.Second
	opts.ReadTimeout = 3 * time.Second
	opts.WriteTimeout = 3 * time.Second
	opts.MaxRetries = 1

	return &Client{rdb: redis.NewClient(opts)}, nil
}

func (client *Client) Ping(ctx context.Context) error {
	return client.rdb.Ping(ctx).Err()
}

func (client *Client) Set(ctx context.Context, key, value string, ttl time.Duration) error {
	return client.rdb.Set(ctx, key, value, ttl).Err()
}

func (client *Client) Get(ctx context.Context, key string) (string, error) {
	val, err := client.rdb.Get(ctx, key).Result()
	if err == redis.Nil {
		return "", nil
	}
	return val, err
}

func (client *Client) Del(ctx context.Context, keys ...string) error {
	return client.rdb.Del(ctx, keys...).Err()
}

func (client *Client) Keys(ctx context.Context, pattern string) ([]string, error) {
	return client.rdb.Keys(ctx, pattern).Result()
}

// DelByPattern deletes all keys matching the given pattern (e.g. "content:*").
func (client *Client) DelByPattern(ctx context.Context, pattern string) (int, error) {
	keys, err := client.rdb.Keys(ctx, pattern).Result()
	if err != nil {
		return 0, err
	}
	if len(keys) == 0 {
		return 0, nil
	}
	deleted, err := client.rdb.Del(ctx, keys...).Result()
	return int(deleted), err
}

func (client *Client) Close() error {
	return client.rdb.Close()
}
