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
├── doc-source/
│   └── openapi/specs/       # OpenAPI specs (one standalone doc per domain)
├── infra/
│   ├── db/migrations/       # Flyway SQL migrations (V0001, V0002, …)
│   ├── docker/              # Docker Compose files
│   └── terraform/           # Infrastructure as Code
├── docs/
│   ├── adr/                 # Architecture Decision Records
│   └── diagrams/            # Mermaid diagrams
├── scripts/
│   ├── dev/                 # Local development scripts
│   ├── deploy/              # Deployment scripts
│   └── db/                  # Database helper scripts
└── .github/
    ├── workflows/           # CI/CD pipelines
    └── PULL_REQUEST_TEMPLATE/  # PR templates per branch type
```

## Prerequisites

```bash
# Contract tools
brew install bufbuild/buf/buf # Proto lint/generate
brew install grpcurl          # gRPC debugging
pipx install schemathesis     # Contract-based API testing
```

## Contract Workflow

### OpenAPI

Specs live under `doc-source/openapi/specs/<service>/`, one standalone OpenAPI
3.1 document per backend domain (e.g. `auth/auth.yaml`, `rust/posts.yaml`,
`go/hackernews.yaml`). The blog-doc docs site reads these specs and merges them
into a single Scalar reference at its build time, so this repo carries no
bundling toolchain.

```bash
# Contract tests against a running service
schemathesis run doc-source/openapi/specs/auth/auth.yaml --base-url http://localhost:8001
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

## Database

PostgreSQL, migrated with Flyway. Migrations live in `infra/db/migrations` and
are named `V####__snake_case_description.sql`; Flyway applies them in order and
records a checksum, so an applied file must never be edited in place — add the
next version instead.

```bash
# Apply pending migrations to the local database
make local-flyway
```

Most tables sit in `public`; the recipe domain has its own `recipe` schema.

| Version | Adds |
|---------|------|
| `V0001` | `users`, `posts`, `likes`, `comments`, `api_usage`, `anime`, `visitor_fingerprint` |
| `V0002` | Performance indexes for the base tables |
| `V0003` | `post_contents`, `assets`, `post_assets` |
| `V0004` | Unique constraint on `assets.sha256` |
| `V0005` | Renames `posts.body` to `posts.image` |
| `V0006` | `hackernews_likes` |
| `V0007` | `recipe` schema + `cuisines`, `sauce_usage_types`, `cooking_method_types` (seeded) |
| `V0008` | `recipe.ingredients` (seeded) |
| `V0009` | `diet_profiles` — one body profile per user |
| `V0010` | `diet_tdee_cases`, `diet_daily_logs` — the burn side of calorie tracking |
| `V0011` | `diet_dishes`, `diet_meals` — the intake side, with a dish seed |

## Environment Variables

### Auth Service (`services/auth`)

| Variable | Required | Description |
|----------|----------|-------------|
| `DATABASE_URL` | Yes | PostgreSQL connection string |
| `REDIS_URL` | No | Redis connection string (default: `redis://127.0.0.1:6379`) |
| `JWT_SECRET` | Yes (production) | Secret key for signing/verifying JWT access tokens |
| `ACCESS_TOKEN_TTL` | No | Access token lifetime in seconds (default: `900`) |
| `REFRESH_TOKEN_TTL` | No | Refresh token lifetime in seconds (default: `604800`) |
| `FRONTEND_URL` | No | Frontend origin URL (default: `http://localhost:3000`) |
| `BACKEND_AUTH_URL` | No | Auth server URL (default: `http://localhost:8001`) |
| `{PROVIDER}_CLIENT_ID` | No | OAuth client ID (`GOOGLE`, `GITHUB`, `APPLE`, `LINE`, `KAKAO`) |
| `{PROVIDER}_CLIENT_SECRET` | No | OAuth client secret |

#### Generating `JWT_SECRET`

```bash
openssl rand -base64 32
```

### Rust API (`services/rust`)

| Variable | Required | Description |
|----------|----------|-------------|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | No | OTLP gRPC endpoint for trace export (default: `http://localhost:4317`) |
| `RUST_LOG` | No | Log filter directives (default: `blog_rust_api=debug,info`) |

Traces are exported over OTLP/gRPC under the service name `blog-rust-api`, and
each request log line carries a `trace_id` field so Loki can link logs to the
matching Tempo trace.

## Development

```bash
# Start all services
docker compose -f infra/docker/compose.dev.yml up

# Run specific service
cd services/rust && cargo run
cd services/go && go run ./cmd/worker
cd apps/web && npm run dev
```

## Branch Naming Convention

| Prefix | Purpose | Example |
|--------|---------|---------|
| `feat/` | New feature | `feat/user-authentication` |
| `fix/` | Bug fix | `fix/login-redirect-loop` |
| `refactor/` | Code refactoring (no behavior change) | `refactor/extract-post-service` |
| `chore/` | Maintenance (deps, config, CI) | `chore/upgrade-axum-0.8` |
| `docs/` | Documentation only | `docs/api-endpoint-guide` |
| `test/` | Test additions/modifications | `test/post-service-unit` |
| `perf/` | Performance improvements | `perf/query-optimization` |
| `spike/` | Investigation/experiment (may be discarded) | `spike/grpc-streaming` |

## Contributing

1. Create a branch following the naming convention above
2. Make your changes
3. Open a PR using the appropriate template (auto-selected by branch prefix)

```bash
# Example workflow
git checkout -b feat/user-profile
# ... make changes ...
git push -u origin feat/user-profile
gh pr create --template feat.md
```

## License

MIT