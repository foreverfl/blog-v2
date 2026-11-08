# POST /assets/sync?bucket=blog-posts-images — reconcile the post-images bucket.
# R2 is the source of truth: rows whose object was deleted directly in R2 (not
# via DELETE /assets/{id}) are removed here. Idempotent — rerun reports zeros.
# Auth: shared API secret as a Bearer token.

POST {{rustUrl}}/assets/sync?bucket=blog-posts-images
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.bucket" exists
jsonpath "$.inserted" isInteger
jsonpath "$.deleted" isInteger
