package r2

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"time"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/credentials"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/aws/aws-sdk-go-v2/service/s3/types"
)

type Client struct {
	s3     *s3.Client
	bucket string
	prefix string
}

func NewClient(endpoint, bucket, prefix, accessKeyID, secretAccessKey, region string) *Client {
	s3Client := s3.New(s3.Options{
		BaseEndpoint: aws.String(endpoint),
		Region:       region,
		Credentials:  credentials.NewStaticCredentialsProvider(accessKeyID, secretAccessKey, ""),
	})

	return &Client{
		s3:     s3Client,
		bucket: bucket,
		prefix: prefix,
	}
}

func (c *Client) objectKey(bucket, key string) string {
	if c.prefix != "" {
		return fmt.Sprintf("%s/%s/%s", c.prefix, bucket, key)
	}
	return fmt.Sprintf("%s/%s", bucket, key)
}

func (c *Client) Get(bucket, key string) (json.RawMessage, error) {
	objKey := c.objectKey(bucket, key)

	var raw json.RawMessage
	err := retry(3, time.Second, func() error {
		out, err := c.s3.GetObject(context.TODO(), &s3.GetObjectInput{
			Bucket: aws.String(c.bucket),
			Key:    aws.String(objKey),
		})
		if err != nil {
			var noSuchKey *types.NoSuchKey
			if errors.As(err, &noSuchKey) {
				raw = nil
				return nil
			}
			return fmt.Errorf("S3 GET %s: %w", objKey, err)
		}
		defer out.Body.Close()

		body, err := io.ReadAll(out.Body)
		if err != nil {
			return err
		}
		raw = body
		return nil
	})
	return raw, err
}

func (c *Client) GetArticles(bucket, key string) ([]map[string]any, error) {
	raw, err := c.Get(bucket, key)
	if err != nil {
		return nil, err
	}
	if raw == nil {
		return nil, nil
	}
	var articles []map[string]any
	if err := json.Unmarshal(raw, &articles); err != nil {
		return nil, fmt.Errorf("unmarshal articles: %w", err)
	}
	return articles, nil
}

func (c *Client) Put(bucket, key string, data []byte, contentType string) error {
	objKey := c.objectKey(bucket, key)
	log.Printf("[R2 PUT] Uploading: %s (%s)", objKey, contentType)

	return retry(3, time.Second, func() error {
		_, err := c.s3.PutObject(context.TODO(), &s3.PutObjectInput{
			Bucket:      aws.String(c.bucket),
			Key:         aws.String(objKey),
			Body:        bytes.NewReader(data),
			ContentType: aws.String(contentType),
		})
		if err != nil {
			return fmt.Errorf("S3 PUT %s: %w", objKey, err)
		}
		return nil
	})
}

func (c *Client) PutJSON(bucket, key string, v any) error {
	data, err := json.Marshal(v)
	if err != nil {
		return fmt.Errorf("marshal: %w", err)
	}
	return c.Put(bucket, key, data, "application/json")
}

func (c *Client) PutBytes(bucket, key string, data []byte, contentType string) error {
	return c.Put(bucket, key, data, contentType)
}

func retry(attempts int, sleep time.Duration, fn func() error) error {
	var err error
	for i := 0; i < attempts; i++ {
		if err = fn(); err == nil {
			return nil
		}
		log.Printf("Retry %d/%d failed: %v", i+1, attempts, err)
		if i < attempts-1 {
			time.Sleep(sleep)
		}
	}
	return err
}
