# PATCH /assets/{id} — partial metadata update (file_name, status).
# object_key and stored bytes are immutable.
# Auth: shared API secret as a Bearer token (a user JWT also works).

# 1. No auth → 401
PATCH {{rustUrl}}/assets/00000000-0000-0000-0000-000000000000
Content-Type: application/json
{"file_name":"nope.png"}
HTTP 401

# 2. Well-formed but unknown id → 404
PATCH {{rustUrl}}/assets/00000000-0000-0000-0000-000000000000
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"file_name":"nope.png"}
HTTP 404

# 3. Happy path: grab an existing asset (needs at least one uploaded),
#    remember its name, rename, verify, then restore to stay rerunnable.
GET {{rustUrl}}/assets?per_page=1
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
asset_id: jsonpath "$.items[0].id"
orig_name: jsonpath "$.items[0].file_name"

PATCH {{rustUrl}}/assets/{{asset_id}}
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"file_name":"hurl-renamed.png","status":"active"}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{asset_id}}"
jsonpath "$.file_name" == "hurl-renamed.png"
jsonpath "$.status" == "active"
jsonpath "$.object_key" exists

# restore original name (partial body: status untouched)
PATCH {{rustUrl}}/assets/{{asset_id}}
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"file_name":"{{orig_name}}"}
HTTP 200
[Asserts]
jsonpath "$.file_name" == "{{orig_name}}"
