# Like a post without a Bearer token → 401 (idempotent; nothing is added).
# The authenticated like is verified via the frontend.
POST {{rustUrl}}/likes/development/lab/002-k8s-pod
HTTP 401
[Asserts]
jsonpath "$.error" exists
