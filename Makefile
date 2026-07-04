# =============================================================================
# Blog v2 – Makefile
# =============================================================================

# ── Docker ──────────────────────────────────────────────────────────────────

COMPOSE_DIR := infra/docker

## local-up: Start local dev environment
.PHONY: local-up
local-up:
	docker network inspect blog-local >/dev/null 2>&1 || docker network create blog-local
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml up -d

## local-up-N: Start instance N — APIs only, ports (N+3)001–004, CORS for :300N
local-up-%:
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
local-flyway:
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml run --rm flyway

## prod-up: Start production environment
.PHONY: prod-up
prod-up:
	set -a && . $(COMPOSE_DIR)/.env.prod && set +a && docker-compose -f $(COMPOSE_DIR)/compose.prod.yml up -d

## prod-down: Stop production environment
.PHONY: prod-down
prod-down:
	set -a && . $(COMPOSE_DIR)/.env.prod && set +a && docker-compose -f $(COMPOSE_DIR)/compose.prod.yml down

## help: Show available targets
.PHONY: help
help:
	@grep -E '^## ' $(MAKEFILE_LIST) | sed 's/^## //' | column -t -s ':'