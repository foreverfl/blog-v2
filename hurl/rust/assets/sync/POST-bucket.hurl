# POST /assets/sync?bucket= — reconcile a specific (non-default) bucket.
# Logical bucket names are identical in every environment (the server adds
# the env prefix), so this file runs unchanged on local and production.
# Rerunnable: sync is idempotent — repeat runs simply report zero changes.
# Auth: shared API secret as a Bearer token (a user JWT also works).

# 1. Hackernews images bucket
POST {{rustUrl}}/assets/sync?bucket=blog-hackernews-images
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.inserted" isInteger
jsonpath "$.deleted" isInteger

# 2. Hackernews JSON bucket
POST {{rustUrl}}/assets/sync?bucket=blog-hackernews
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.inserted" isInteger
jsonpath "$.deleted" isInteger
