FROM golang:1.26 AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 go build -o blog-go-api .

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends chromium ca-certificates && \
    groupadd --system app && \
    useradd --system --gid app --create-home --home-dir /home/app app && \
    mkdir -p /home/app/.cache && \
    chown -R app:app /home/app && \
    rm -rf /var/lib/apt/lists/*
ENV CHROMIUM_PATH=/usr/bin/chromium
ENV HOME=/home/app
COPY --from=builder /app/blog-go-api /usr/local/bin/
USER app:app
EXPOSE 8003
CMD ["blog-go-api"]
