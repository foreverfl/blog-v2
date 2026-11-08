# DELETE /assets/{id} — removes the R2 object first, then the DB row.
# Rerunnable: uploads test.png (copy of the shared fixture; sha256 dedup may
# hand back the shared seed row) and deletes it — POST.hurl recreates the seed.
# Auth: shared API secret as a Bearer token (a user JWT also works).

# 1. No auth → 401
DELETE {{rustUrl}}/assets/00000000-0000-0000-0000-000000000000
HTTP 401

# 2. Well-formed but unknown id → 404
DELETE {{rustUrl}}/assets/00000000-0000-0000-0000-000000000000
Authorization: Bearer {{apiSecret}}
HTTP 404

# 3. Happy path: upload a throwaway asset, delete it, verify it is gone.
POST {{rustUrl}}/assets
Authorization: Bearer {{apiSecret}}
[MultipartFormData]
file: file,test.png; image/png
HTTP 201
[Captures]
victim_id: jsonpath "$[0].id"

DELETE {{rustUrl}}/assets/{{victim_id}}
Authorization: Bearer {{apiSecret}}
HTTP 204

GET {{rustUrl}}/assets/{{victim_id}}
Authorization: Bearer {{apiSecret}}
HTTP 404
