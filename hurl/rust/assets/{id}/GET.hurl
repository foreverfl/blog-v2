# GET /assets/{id} — single asset (auth: API secret or user JWT)

# 1. Malformed uuid → 400 (Path<Uuid> parse failure, before auth/query)
GET {{rustUrl}}/assets/not-a-uuid
Authorization: Bearer {{apiSecret}}
HTTP 400

# 2. No auth → 401
GET {{rustUrl}}/assets/00000000-0000-0000-0000-000000000000
HTTP 401

# 3. Well-formed but unknown id → 404
GET {{rustUrl}}/assets/00000000-0000-0000-0000-000000000000
Authorization: Bearer {{apiSecret}}
HTTP 404

# 4. Happy path: grab an existing asset id from the list, then fetch it.
#    Needs at least one uploaded asset (uploads/POST.hurl seeds one).
GET {{rustUrl}}/assets?per_page=1
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
asset_id: jsonpath "$.items[0].id"

GET {{rustUrl}}/assets/{{asset_id}}
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{asset_id}}"
jsonpath "$.object_key" exists
