# Clear an admin reply without a Bearer token → 401 (idempotent).
# Admin-authorized behavior is verified via the frontend.
DELETE {{rustUrl}}/comments/development/lab/002-k8s-pod/00000000-0000-0000-0000-000000000000/admin
HTTP 401
[Asserts]
jsonpath "$.error" exists
