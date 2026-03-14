---
applyTo: "**"
---

# Copilot Review Instructions

## Language
- Always write review comments in Korean (한국어).
- Keep technical terms (e.g., Docker, Flyway, GHCR, CI/CD) in their original English form.

## Review Scope
- Focus only on issues that could cause build failures, runtime errors, or security vulnerabilities.
- Do not suggest stylistic improvements, refactoring, or "nice-to-have" enhancements.
- Do not flag missing tests unless the change is clearly untested and risky.
- Respect the principle of minimal changes — only comment on what is actually broken or dangerous.

## Severity
- Clearly indicate severity: 🔴 (must fix), 🟡 (should fix), 🟢 (optional).
