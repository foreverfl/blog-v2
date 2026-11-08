# POST /anime/clips/{id}/like — like + move R2 object feed/ → liked/ (auth: API secret)

# 1. No auth → 401
POST {{rustUrl}}/anime/clips/1/like
HTTP 401

# 2. Unknown id → 404
POST {{rustUrl}}/anime/clips/999999/like
Authorization: Bearer {{apiSecret}}
HTTP 404

# 3. Happy path: grab an existing clip id, like it (idempotent when already liked).
#    Needs at least one clip (clips/POST.hurl seeds one).
GET {{rustUrl}}/anime/clips?limit=1
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
clip_id: jsonpath "$[0].id"

POST {{rustUrl}}/anime/clips/{{clip_id}}/like
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.liked" == true
jsonpath "$.r2_key" startsWith "liked/"
jsonpath "$.thumbnail_url" contains "liked/"
jsonpath "$.liked_at" isString
