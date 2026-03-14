FROM golang:1.26 AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 go build -o blog-go-api .

FROM gcr.io/distroless/static
COPY --from=builder /app/blog-go-api /usr/local/bin/
EXPOSE 8003
CMD ["blog-go-api"]