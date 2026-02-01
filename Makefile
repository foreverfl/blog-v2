# =============================================================================
# Blog v2 – Makefile
# =============================================================================

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