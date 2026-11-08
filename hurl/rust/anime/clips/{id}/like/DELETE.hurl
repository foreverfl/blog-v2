# DELETE /anime/clips/{id}/like — unlike + move R2 object liked/ → feed/ (auth: API secret)

# 1. No auth → 401
DELETE {{rustUrl}}/anime/clips/1/like
HTTP 401

# 2. Unknown id → 404
DELETE {{rustUrl}}/anime/clips/999999/like
Authorization: Bearer {{apiSecret}}
HTTP 404

# 3. Happy path: grab an existing clip id, unlike it (idempotent when not liked).
GET {{rustUrl}}/anime/clips?limit=1
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
clip_id: jsonpath "$[0].id"

DELETE {{rustUrl}}/anime/clips/{{clip_id}}/like
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.liked" == false
jsonpath "$.r2_key" startsWith "feed/"
jsonpath "$.thumbnail_url" contains "feed/"
jsonpath "$.liked_at" == null
