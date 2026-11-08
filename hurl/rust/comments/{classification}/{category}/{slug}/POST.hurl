# Post a comment without a Bearer token → 401 (idempotent; nothing is created).
# The authenticated create is verified via the frontend until a DELETE endpoint
# exists to make a persisting create-then-delete round-trip rerunnable.
POST {{rustUrl}}/comments/development/lab/002-k8s-pod
Content-Type: application/json
{
  "content": "hurl unauth test"
}
HTTP 401
[Asserts]
jsonpath "$.error" exists
