# Edit a comment without a Bearer token → 401 (idempotent).
# The authenticated owner-only edit is verified via the frontend.
PATCH {{rustUrl}}/comments/development/lab/002-k8s-pod/00000000-0000-0000-0000-000000000000
Content-Type: application/json
{
  "content": "hurl unauth edit"
}
HTTP 401
[Asserts]
jsonpath "$.error" exists
