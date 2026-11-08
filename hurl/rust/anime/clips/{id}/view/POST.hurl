# POST /anime/clips/{id}/view — count one viewing (auth: API secret)

# 1. No auth → 401
POST {{rustUrl}}/anime/clips/1/view
HTTP 401

# 2. Unknown id → 404
POST {{rustUrl}}/anime/clips/999999/view
Authorization: Bearer {{apiSecret}}
HTTP 404

# 3. Happy path: grab an existing clip id from the list, then record a view.
#    Needs at least one clip (clips/POST.hurl seeds one).
GET {{rustUrl}}/anime/clips?limit=1
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
clip_id: jsonpath "$[0].id"

POST {{rustUrl}}/anime/clips/{{clip_id}}/view
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.view_count" >= 1
jsonpath "$.last_viewed_at" exists
