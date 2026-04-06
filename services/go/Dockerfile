FROM golang:1.25 AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 go build -o blog-go-api .

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends chromium ca-certificates && \
    rm -rf /var/lib/apt/lists/*
ENV CHROMIUM_PATH=/usr/bin/chromium
COPY --from=builder /app/blog-go-api /usr/local/bin/
EXPOSE 8003
CMD ["blog-go-api"]
