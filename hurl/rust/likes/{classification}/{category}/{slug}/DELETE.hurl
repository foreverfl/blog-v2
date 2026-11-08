# Unlike a post without a Bearer token → 401 (idempotent; nothing is removed).
# The authenticated unlike is verified via the frontend.
DELETE {{rustUrl}}/likes/development/lab/002-k8s-pod
HTTP 401
[Asserts]
jsonpath "$.error" exists
