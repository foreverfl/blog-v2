# Delete a comment without a Bearer token → 401 (idempotent).
# The authenticated owner-only delete is verified via the frontend.
DELETE {{rustUrl}}/comments/development/lab/002-k8s-pod/00000000-0000-0000-0000-000000000000
HTTP 401
[Asserts]
jsonpath "$.error" exists
