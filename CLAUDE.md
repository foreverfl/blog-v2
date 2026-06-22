# blog-v2 — project instructions

> Korean mirror: `CLAUDE-ko.md`. English is the source of truth; when you edit
> one, mirror the change in the other in the same turn.

## OpenAPI spec upkeep

Every endpoint you add or change MUST have its OpenAPI spec updated to match — do
this as part of the work, not only when asked (symmetric to the global `.hurl`
upkeep rule).

- Specs live under `doc-source/openapi/specs/<service>/<domain>.yaml`, one
  standalone OpenAPI 3.1 document per backend domain — e.g. `rust/posts.yaml`,
  `go/hackernews.yaml`, `auth/auth.yaml`. Update the domain file whose route
  changed; add a new `<service>/<domain>.yaml` when a new domain appears.
- Mirror the route **exactly**: path, method, params, request/response shape,
  status codes, auth. Read the actual handler and types — do not guess.
- Follow the existing `auth/auth.yaml` conventions:
  - `openapi: 3.1.0`; `info` with a unique `x-api-id`.
  - `servers`: the service base — prod `https://api.mogumogu.dev/<service>` plus
    the local port (auth 8001, rust 8002, go 8003, haskell 8004). Write paths
    WITHOUT the service prefix; the server URL carries it.
  - `security: []` on public operations; a security scheme (`bearerAuth` JWT,
    `apiSecret`, `hackernewsSecret`, …) on authenticated ones.
  - A shared `Error` schema `{error: string}`; OpenAPI 3.1 `type: [T, 'null']`
    for nullable fields.
- Do NOT add `/health` to the specs — it collides across services in the merged
  document and is infra, not API surface.
- The blog-doc docs site merges these specs into one Scalar reference at build
  time (`blog-doc/scripts/bundle-openapi.js`, reading `OPENAPI_SPECS_SRC` or the
  sibling checkout). blog-v2 has no redocly/bundling step of its own.
- Validate a changed spec without extra tooling: it must be valid YAML and the
  blog-doc bundler must merge it without error (`blog-doc/scripts/bundle-openapi.js`
  parses every spec at build — a malformed file fails there). Keep all `$ref`s
  intra-document (`#/components/...`) so they resolve in the merged doc. `redocly
  lint` is an optional deeper OpenAPI check, not required.
- Run `/add-openapi <service>/<domain>` to regenerate a domain spec from its
  current routes.
