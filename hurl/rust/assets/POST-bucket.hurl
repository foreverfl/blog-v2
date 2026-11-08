# POST /assets?bucket= — upload into a selected bucket (logical name).
# The server maps logical → physical ("dev-" prefix locally) and verifies the
# bucket exists in R2; unknown names are rejected.
# Rerunnable: uses its own file (bucket-test.png — same image as test.png but
# distinct sha256; identical bytes would dedup into the default-bucket seed)
# and deletes the asset after.

# 1. Unknown bucket → 400 before any upload happens
POST {{rustUrl}}/assets?bucket=definitely-not-a-bucket
Authorization: Bearer {{apiSecret}}
[MultipartFormData]
file: file,bucket-test.png; image/png
HTTP 400

# 2. Valid logical bucket → stored in the physical dev- bucket locally
POST {{rustUrl}}/assets?bucket=blog-hackernews-images
Authorization: Bearer {{apiSecret}}
[MultipartFormData]
file: file,bucket-test.png; image/png
HTTP 201
[Captures]
bucket_asset_id: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].bucket" == "dev-blog-hackernews-images"

# cleanup so reruns exercise a fresh upload instead of the dedup path
DELETE {{rustUrl}}/assets/{{bucket_asset_id}}
Authorization: Bearer {{apiSecret}}
HTTP 204
