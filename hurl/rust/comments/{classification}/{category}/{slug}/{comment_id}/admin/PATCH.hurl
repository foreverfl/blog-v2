# Set an admin reply without a Bearer token → 401 (idempotent).
# Admin-authorized behavior is verified via the frontend.
PATCH {{rustUrl}}/comments/development/lab/002-k8s-pod/00000000-0000-0000-0000-000000000000/admin
Content-Type: application/json
{
  "reply": "hurl unauth reply"
}
HTTP 401
[Asserts]
jsonpath "$.error" exists
