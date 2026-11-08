# POST /anime/clips/{id}/thumbnail — put a webp next to the clip's video (auth: API secret)
# Multipart rides on every case — a non-multipart request is rejected before auth.

# 1. No auth → 401
POST {{rustUrl}}/anime/clips/1/thumbnail
[Multipart]
file: file,sample.webp; image/webp
HTTP 401

# 2. Unknown id → 404
POST {{rustUrl}}/anime/clips/999999/thumbnail
Authorization: Bearer {{apiSecret}}
[Multipart]
file: file,sample.webp; image/webp
HTTP 404

# 3. Happy path: grab an existing clip id, upload its thumbnail (overwrite-safe).
#    Needs at least one clip (clips/POST.hurl seeds one).
GET {{rustUrl}}/anime/clips?limit=1
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
clip_id: jsonpath "$[0].id"

POST {{rustUrl}}/anime/clips/{{clip_id}}/thumbnail
Authorization: Bearer {{apiSecret}}
[Multipart]
file: file,sample.webp; image/webp
HTTP 204
