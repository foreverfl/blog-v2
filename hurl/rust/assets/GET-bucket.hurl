# GET /assets?bucket= — list filtered to one bucket (logical name).
# The server maps logical → physical ("dev-" locally) before the DB WHERE,
# so a filter can only ever return rows from its own physical bucket.
# Auth: shared API secret as a Bearer token (a user JWT also works).

# 1. Filter by the default bucket: no rows from other buckets may appear
GET {{rustUrl}}/assets?bucket=blog-posts-assets
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.items[*].bucket" not includes "dev-blog-hackernews-images"
jsonpath "$.total" isInteger

# 2. Filter by another bucket: rows from the default bucket must be excluded
GET {{rustUrl}}/assets?bucket=blog-hackernews-images
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.items[*].bucket" not includes "dev-blog-posts-assets"
