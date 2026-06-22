---
name: add-openapi
description: Regenerate or update a blog-v2 OpenAPI domain spec from its current routes. Reads the service's route registration, handlers, and types, then writes doc-source/openapi/specs/<service>/<domain>.yaml to mirror them exactly, following the auth/auth.yaml conventions. Use after adding or changing an endpoint.
argument-hint: "<service>/<domain>  (e.g. rust/posts, go/hackernews) — or a changed route file path"
allowed-tools: Read, Grep, Glob, Edit, Write, Bash(redocly lint:*), Bash(find:*), Bash(ls:*)
---

> Korean mirror: `SKILL-ko.md` (read-only; Claude Code loads only this `SKILL.md`).
> English is the source of truth — edit both in the same turn.

You keep a blog-v2 OpenAPI domain spec in sync with the real routes. The argument
is either `<service>/<domain>` (e.g. `rust/posts`, `go/hackernews`) or the path of
a route/handler file that was just changed — in that case infer the service and
domain from it.

## Steps

1. **Locate the routes.** Read the service's route registration and the handlers
   + request/response types for the target domain:
   - rust services: `services/<svc>/src/routes/*.rs`, `services/<svc>/src/handlers/*.rs`, `services/<svc>/src/types/*`
   - go service: `services/go/**/*.go` (the `mux.HandleFunc` registrations and the structs with json tags)
   Enumerate every method + path for the domain.

2. **Write the spec** to `doc-source/openapi/specs/<service>/<domain>.yaml`,
   following the conventions in this repo's `CLAUDE.md` and the `auth/auth.yaml`
   template exactly:
   - `openapi: 3.1.0`; `info` with title/description and a unique `x-api-id`.
   - `servers`: prod `https://api.mogumogu.dev/<service>` + local port (auth 8001,
     rust 8002, go 8003, haskell 8004). Paths written WITHOUT the service prefix.
   - one `tags` entry for the domain.
   - `security: []` on public operations; a security scheme on authenticated ones
     (`bearerAuth` JWT, `apiSecret`, `hackernewsSecret`, …) matching the handler.
   - reuse `components` (schemas/parameters/responses) and keep it DRY; a shared
     `Error` schema `{error: string}`; OpenAPI 3.1 `type: [T, 'null']` for nullable.
   - do NOT add `/health`.
   If the file already exists, update it in place rather than rewriting wholesale.

3. **Validate** without extra tooling: ensure it is valid YAML and that the
   blog-doc bundler merges it without error (use intra-document `$ref`s only, so
   they resolve in the merged doc). `redocly lint <spec>` is an optional deeper
   OpenAPI check if it happens to be installed.

4. **Report** the final endpoint list, the validation result, and any code
   discrepancy. Do not commit — leave that to the user.

The blog-doc site merges every `specs/**/*.yaml` into one Scalar bundle at build
time, so a valid standalone spec here is all that is needed; there is no bundling
step in this repo.
