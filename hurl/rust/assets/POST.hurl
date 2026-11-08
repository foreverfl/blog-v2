# Upload a file (image/video/audio/document). Deduplicated by SHA-256, so
# re-running returns the same asset — rerunnable.
# url comes from the per-bucket domain pattern: https://assets-{bucket}.mogumogu.dev/{key}.
# Auth: shared API secret as a Bearer token (a user JWT also works).
POST {{rustUrl}}/assets
Authorization: Bearer {{apiSecret}}
[MultipartFormData]
file: file,test.png; image/png
HTTP 201
[Asserts]
jsonpath "$[0].object_key" exists
jsonpath "$[0].kind" == "image"
jsonpath "$[0].url" matches "^https://assets-[a-z0-9-]+\\.mogumogu\\.dev/.+"

# Without auth the upload is rejected.
POST {{rustUrl}}/assets
[MultipartFormData]
file: file,test.png; image/png
HTTP 401
