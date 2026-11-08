# PATCH /anime/clips/{id} — fill jellyfin_item·series_title on an existing clip (auth: API secret)

# 1. No auth → 401
PATCH {{rustUrl}}/anime/clips/1
Content-Type: application/json
{"jellyfin_item":"hurl-item"}
HTTP 401

# 2. Unknown id → 404
PATCH {{rustUrl}}/anime/clips/999999
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"jellyfin_item":"hurl-item"}
HTTP 404

# 3. Happy path: patch the hurl-sample seed clip (clips/POST.hurl seeds it),
#    so no real clip's jellyfin_item gets overwritten.
GET {{rustUrl}}/anime/clips?limit=1000
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
clip_id: jsonpath "$[?(@.series_slug=='hurl-sample')].id" nth 0

PATCH {{rustUrl}}/anime/clips/{{clip_id}}
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"jellyfin_item":"hurl-item"}
HTTP 200
[Asserts]
jsonpath "$.jellyfin_item" == "hurl-item"
jsonpath "$.series_slug" == "hurl-sample"

# 4. Patching only the title leaves jellyfin_item as it was.
PATCH {{rustUrl}}/anime/clips/{{clip_id}}
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"series_title":"ハール別名"}
HTTP 200
[Asserts]
jsonpath "$.series_title" == "ハール別名"
jsonpath "$.jellyfin_item" == "hurl-item"

# 5. Nothing to patch → 400
PATCH {{rustUrl}}/anime/clips/{{clip_id}}
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{}
HTTP 400
