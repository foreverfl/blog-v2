# =============================================================================
# Blog v2 – Makefile
# =============================================================================

# ── Docker ──────────────────────────────────────────────────────────────────

COMPOSE_DIR := infra/docker
COMPOSE = set -a && . $(COMPOSE_DIR)/.env.local && set +a && docker-compose -f $(COMPOSE_DIR)/compose.local.yml

# ── Service aliases ─────────────────────────────────────────────────────────
#   make local-up auth haskell     (start auth + haskell)
#   make local-build auth          (clean rebuild auth)
#   make local-logs rust go        (tail rust + go logs)

SERVICE_TARGETS := local-up local-stop local-restart local-build local-logs

ifneq ($(filter $(SERVICE_TARGETS),$(MAKECMDGOALS)),)
  SERVICE_ARGS := $(filter-out $(SERVICE_TARGETS),$(MAKECMDGOALS))
endif

# short name → compose service name
resolve = $(patsubst $(1):%,%,$(filter $(1):%,auth:auth-api rust:rust-api go:go-api haskell:haskell-api))
SERVICES := $(foreach _s,$(SERVICE_ARGS),$(or $(call resolve,$(_s)),$(_s)))

# cache volumes per service
CACHE_auth    := auth_cargo_cache auth_target_cache
CACHE_rust    := rust_cargo_cache rust_target_cache
CACHE_go      := go_mod_cache
CACHE_haskell := haskell_cabal_cache haskell_cabal_store
ALL_CACHES    := $(CACHE_auth) $(CACHE_rust) $(CACHE_go) $(CACHE_haskell)

# swallow service names so make doesn't treat them as targets
ifneq ($(SERVICE_ARGS),)
$(SERVICE_ARGS):
	@:
endif

## local-up: Start services
.PHONY: local-up
local-up:
	$(COMPOSE) up -d $(SERVICES)

## local-down: Tear down all services and networks
.PHONY: local-down
local-down:
	$(COMPOSE) down

## local-stop: Stop services without removing
.PHONY: local-stop
local-stop:
	$(COMPOSE) stop $(SERVICES)

## local-restart: Rebuild and restart services
.PHONY: local-restart
local-restart:
	$(COMPOSE) rm -sf $(SERVICES) && $(COMPOSE) up -d --build $(SERVICES)

## local-build: Clean rebuild – remove containers, caches, pull fresh images
.PHONY: local-build
local-build:
	$(COMPOSE) rm -sf $(SERVICES)
	@VOLS="$(if $(SERVICE_ARGS),$(foreach _s,$(SERVICE_ARGS),$(CACHE_$(_s))),$(ALL_CACHES))"; \
	for v in $$VOLS; do \
		docker volume ls -q | grep "$$v" | xargs docker volume rm -f 2>/dev/null || true; \
	done
	$(COMPOSE) pull $(SERVICES)
	docker image prune -f

## local-logs: Tail logs for services
.PHONY: local-logs
local-logs:
	$(COMPOSE) logs -f $(SERVICES)

## prod-up: Start production environment
.PHONY: prod-up
prod-up:
	set -a && . $(COMPOSE_DIR)/.env.prod && set +a && docker-compose -f $(COMPOSE_DIR)/compose.prod.yml up -d

## prod-down: Stop production environment
.PHONY: prod-down
prod-down:
	set -a && . $(COMPOSE_DIR)/.env.prod && set +a && docker-compose -f $(COMPOSE_DIR)/compose.prod.yml down

# ── OpenAPI ──────────────────────────────────────────────────────────────────

OPENAPI_SPECS_DIR := contracts/openapi/specs
OPENAPI_DIST_DIR  := contracts/openapi/dist
OPENAPI_BUNDLE    := $(OPENAPI_DIST_DIR)/openapi.yaml
OPENAPI_HTML      := $(OPENAPI_DIST_DIR)/index.html

OPENAPI_SPECS := $(wildcard $(OPENAPI_SPECS_DIR)/*.yaml)

$(OPENAPI_DIST_DIR):
	mkdir -p $@

## openapi-bundle: Join all OpenAPI specs into a single file
.PHONY: openapi-bundle
openapi-bundle: $(OPENAPI_DIST_DIR)
	redocly join $(OPENAPI_SPECS) \
		--prefix-tags-with-filename \
		--prefix-components-with-info-prop x-api-id \
		-o $(OPENAPI_BUNDLE)
	@echo "Bundled → $(OPENAPI_BUNDLE)"

## openapi-lint: Lint individual OpenAPI specs
.PHONY: openapi-lint
openapi-lint:
	redocly lint $(OPENAPI_SPECS)

## openapi-preview: Build HTML docs from the bundled spec and open in browser
.PHONY: openapi-preview
openapi-preview: openapi-bundle
	redocly build-docs $(OPENAPI_BUNDLE) -o $(OPENAPI_HTML)
	@echo "Opening $(OPENAPI_HTML) in browser..."
	open $(OPENAPI_HTML)

## openapi-clean: Remove generated OpenAPI artifacts
.PHONY: openapi-clean
openapi-clean:
	rm -rf $(OPENAPI_DIST_DIR)

## help: Show available targets
.PHONY: help
help:
	@grep -E '^## ' $(MAKEFILE_LIST) | sed 's/^## //' | column -t -s ':'