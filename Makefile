# =============================================================================
# Blog v2 – Makefile
# =============================================================================

# ── Docker ──────────────────────────────────────────────────────────────────

COMPOSE_DIR := infra/docker

## docker-ready: Start OrbStack if Docker is not answering, then wait for it
.PHONY: docker-ready
docker-ready:
	@docker info >/dev/null 2>&1 || { orb start; until docker info >/dev/null 2>&1; do sleep 1; done; }

## local-up: Start local dev environment
.PHONY: local-up
local-up: docker-ready
	docker network inspect blog-local >/dev/null 2>&1 || docker network create blog-local
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml up -d
	@echo "waiting for web on http://localhost:3000/ko (npm ci runs first) ..."
	@for i in $$(seq 300); do \
		curl -sf -o /dev/null http://localhost:3000/ko && { echo "✓ ready: http://localhost:3000/ko"; exit 0; }; \
		sleep 1; \
	done; \
	echo "web did not answer in 5 minutes — see: docker logs blog-web"; exit 1

## local-up-N: Start instance N — APIs only, ports (N+3)001–004, CORS for :300N
local-up-%: docker-ready
	@test -f $(COMPOSE_DIR)/.env.local || { echo "$(COMPOSE_DIR)/.env.local not found — copy it from the main checkout"; exit 1; }
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && \
		N=$$(($* + 3)) && \
		AUTH_PORT=$${N}001 RUST_PORT=$${N}002 GO_PORT=$${N}003 HASKELL_PORT=$${N}004 \
		STACK_PREFIX=blog-i$* FRONTEND_URL=http://localhost:300$* \
		docker-compose -p blog-i$* -f $(COMPOSE_DIR)/compose.local.yml up -d --no-deps auth-api rust-api go-api haskell-api

## local-down-N: Stop instance N
local-down-%:
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && \
		STACK_PREFIX=blog-i$* docker-compose -p blog-i$* -f $(COMPOSE_DIR)/compose.local.yml down

## local-down: Stop local dev environment
.PHONY: local-down
local-down:
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml down

## local-restart: Stop → rebuild → start local dev environment
.PHONY: local-restart
local-restart:
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && \
		docker-compose -f $(COMPOSE_DIR)/compose.local.yml down && \
		docker-compose -f $(COMPOSE_DIR)/compose.local.yml up -d --build

## local-logs-auth: Tail logs for auth-api only
.PHONY: local-logs-auth
local-logs-auth:
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml logs -f auth-api

## local-flyway: Run Flyway migrations against the local database
.PHONY: local-flyway
local-flyway: docker-ready
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml run --rm flyway

RUST_LOCAL := http://localhost:8002

## hackernews-sync: Import Hacker News posts from R2 into the local DB, from the day after the newest one (or FROM=YYMMDD)
.PHONY: hackernews-sync
hackernews-sync:
	@set -a && . $(COMPOSE_DIR)/.env.local && set +a && \
	for i in $$(seq 60); do curl -sf -o /dev/null $(RUST_LOCAL)/health && break; sleep 1; done && \
	from="$(FROM)" && \
	if [ -z "$$from" ]; then \
		latest=$$(docker exec blog-postgres sh -c "psql -U \"\$$POSTGRES_USER\" -d \"\$$POSTGRES_DB\" -tAc \"SELECT max(slug) FROM posts WHERE classification = 'trends' AND category = 'hackernews'\""); \
		[ -n "$$latest" ] || { echo "no hacker news post in the local DB — run: make hackernews-sync FROM=YYMMDD"; exit 1; }; \
		from=$$(date -j -v+1d -f %y%m%d "$$latest" +%y%m%d); \
		[ "$$from" -le "$$(TZ=Asia/Tokyo date +%y%m%d)" ] || { echo "already up to date: $$latest"; exit 0; }; \
	fi && \
	echo "importing from $$from ..." && \
	response=$$(curl -s -X POST -H "Authorization: Bearer $$API_SECRET" "$(RUST_LOCAL)/import/json?from=$$from") && \
	job=$$(echo "$$response" | jq -r '.job_id // empty') && \
	{ [ -n "$$job" ] || { echo "$$response"; exit 1; }; } && \
	until result=$$(curl -sf -H "Authorization: Bearer $$API_SECRET" "$(RUST_LOCAL)/import/jobs/$$job") && \
		[ "$$(echo "$$result" | jq -r .status)" = completed ]; do sleep 2; done && \
	echo "$$result" | jq '{from, imported, skipped, error_count}'

## prod-up: Start production environment
.PHONY: prod-up
prod-up:
	set -a && . $(COMPOSE_DIR)/.env.prod && set +a && docker-compose -f $(COMPOSE_DIR)/compose.prod.yml up -d

## prod-down: Stop production environment
.PHONY: prod-down
prod-down:
	set -a && . $(COMPOSE_DIR)/.env.prod && set +a && docker-compose -f $(COMPOSE_DIR)/compose.prod.yml down

# ── Test ────────────────────────────────────────────────────────────────────

## test: Run the tests of every service under services/
.PHONY: test
test:
	cd services/auth && cargo test
	cd services/rust && cargo test
	cd services/go && go vet ./... && go test -short ./...
	cd services/go-cli && go vet ./... && go test ./...
	cd services/haskell && cabal test
	cd web && npm test

## help: Show available targets
.PHONY: help
help:
	@grep -E '^## ' $(MAKEFILE_LIST) | sed 's/^## //' | column -t -s ':'