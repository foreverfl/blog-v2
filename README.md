# blog-v2

A polyglot monorepo for my personal blog platform. Migrating from [Next.js blog](https://github.com/foreverfl/blog) to a more modular, contract-driven architecture.

## Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Frontend | Astro | Static site generation, minimal JS |
| API | Rust (Axum) | High-performance REST/gRPC API |
| Worker | Go | Background jobs, cron, queue consumer |
| Experimental | Haskell | Learning playground |
| Contracts | OpenAPI, Protocol Buffers | API-first development |
| Infrastructure | Docker, Terraform | Container orchestration, IaC |

## Project Structure

```
.
├── apps/
│   └── web/                 # Astro frontend
├── services/
│   ├── rust/                # Rust API (Axum)
│   ├── go/                  # Go worker/cron/queue
│   └── haskell/             # Haskell experimental service
├── contracts/
│   ├── openapi/             # OpenAPI specs
│   └── proto/               # Protocol Buffer definitions
├── infra/
│   ├── docker/              # Docker Compose files
│   └── terraform/           # Infrastructure as Code
├── docs/
│   ├── adr/                 # Architecture Decision Records
│   └── diagrams/            # Mermaid diagrams
├── scripts/
│   ├── dev/                 # Local development scripts
│   ├── deploy/              # Deployment scripts
│   └── db/                  # Database migrations/backups
└── .github/
    └── workflows/           # CI/CD pipelines
```

## Prerequisites

```bash
# Contract tools
brew install redocly-cli      # OpenAPI lint/bundle
brew install bufbuild/buf/buf # Proto lint/generate
brew install grpcurl          # gRPC debugging
pipx install schemathesis     # Contract-based API testing
```

## Contract Workflow

### OpenAPI

```bash
# Lint spec
redocly lint contracts/openapi/openapi.yaml

# Bundle into single file
redocly bundle contracts/openapi/openapi.yaml -o dist/openapi.bundle.yaml

# Generate HTML docs
redocly build-docs dist/openapi.bundle.yaml -o dist/api-docs.html

# Run contract tests against running server
schemathesis run dist/openapi.bundle.yaml --base-url http://localhost:8080
```

### gRPC (Protocol Buffers)

```bash
# Format & lint
buf format -w
buf lint

# Breaking change detection (CI)
buf breaking --against '.git#branch=main'

# Generate code
buf generate

# Smoke test (server must be running)
grpcurl -plaintext localhost:50051 list
grpcurl -plaintext -d '{"id":"123"}' localhost:50051 blog.v1.PostService/GetPost
```

## Development

```bash
# Start all services
docker compose -f infra/docker/compose.dev.yml up

# Run specific service
cd services/rust && cargo run
cd services/go && go run ./cmd/worker
cd apps/web && npm run dev
```

## Documentation

- [ADR Template](docs/adr/0000-template.md) - Architecture Decision Records format

## License

MIT