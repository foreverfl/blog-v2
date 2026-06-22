# =============================================================================
# Blog v2 – Makefile
# =============================================================================

# ── Docker ──────────────────────────────────────────────────────────────────

COMPOSE_DIR := infra/docker

## local-up: Start local dev environment
.PHONY: local-up
local-up:
	set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml up -d

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