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
}

func NewClient(endpoint, bucket, accessKeyID, secretAccessKey, region string) *Client {
	s3Client := s3.New(s3.Options{
		BaseEndpoint: aws.String(endpoint),
		Region:       region,
		Credentials:  credentials.NewStaticCredentialsProvider(accessKeyID, secretAccessKey, ""),
	})

	return &Client{
		s3:     s3Client,
		bucket: bucket,
	}
}

func (c *Client) Bucket() string {
	return c.bucket
}

func (c *Client) Get(key string) (json.RawMessage, error) {
	var raw json.RawMessage
	err := retry(3, time.Second, func() error {
		out, err := c.s3.GetObject(context.TODO(), &s3.GetObjectInput{
			Bucket: aws.String(c.bucket),
			Key:    aws.String(key),
		})
		if err != nil {
			var noSuchKey *types.NoSuchKey
			if errors.As(err, &noSuchKey) {
				raw = nil
				return nil
			}
			return fmt.Errorf("S3 GET %s/%s: %w", c.bucket, key, err)
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

func (c *Client) GetArticles(key string) ([]map[string]any, error) {
	raw, err := c.Get(key)
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

func (c *Client) Put(key string, data []byte, contentType string) error {
	log.Printf("[R2 PUT] Uploading: %s/%s (%s)", c.bucket, key, contentType)

	return retry(3, time.Second, func() error {
		_, err := c.s3.PutObject(context.TODO(), &s3.PutObjectInput{
			Bucket:      aws.String(c.bucket),
			Key:         aws.String(key),
			Body:        bytes.NewReader(data),
			ContentType: aws.String(contentType),
		})
		if err != nil {
			return fmt.Errorf("S3 PUT %s/%s: %w", c.bucket, key, err)
		}
		return nil
	})
}

func (c *Client) Exists(key string) (bool, error) {
	_, err := c.s3.HeadObject(context.TODO(), &s3.HeadObjectInput{
		Bucket: aws.String(c.bucket),
		Key:    aws.String(key),
	})
	if err != nil {
		var noSuchKey *types.NoSuchKey
		var notFound *types.NotFound
		if errors.As(err, &noSuchKey) || errors.As(err, &notFound) {
			return false, nil
		}
		// HeadObject returns 404 as a generic smithy error; check HTTP status
		var respErr interface{ HTTPStatusCode() int }
		if errors.As(err, &respErr) && respErr.HTTPStatusCode() == 404 {
			return false, nil
		}
		return false, fmt.Errorf("S3 HEAD %s/%s: %w", c.bucket, key, err)
	}
	return true, nil
}

func (c *Client) PutJSON(key string, v any) error {
	data, err := json.Marshal(v)
	if err != nil {
		return fmt.Errorf("marshal: %w", err)
	}
	return c.Put(key, data, "application/json")
}

func (c *Client) PutBytes(key string, data []byte, contentType string) error {
	return c.Put(key, data, contentType)
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