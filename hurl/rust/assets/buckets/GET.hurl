# GET /assets/buckets — selectable buckets as logical names.
# Locally only dev-* buckets are listed, with the "dev-" prefix stripped,
# so the same logical names work in every environment.
# Auth: shared API secret as a Bearer token (a user JWT also works).

# 1. No auth → 401
GET {{rustUrl}}/assets/buckets
HTTP 401

# 2. Bucket list with the default included, no physical dev- names leaked
GET {{rustUrl}}/assets/buckets
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.default" == "blog-posts-assets"
jsonpath "$.buckets" includes "blog-posts-assets"
jsonpath "$.buckets" not includes "dev-blog-posts-assets"
