# DELETE /recipe/ingredients/{id} — shared-secret delete

# 1. Missing Bearer secret → 401 (auth checked before any delete)
DELETE {{rustUrl}}/recipe/ingredients/00000000-0000-0000-0000-000000000000
HTTP 401

# 2. Authorized but unknown id → 404
DELETE {{rustUrl}}/recipe/ingredients/00000000-0000-0000-0000-000000000000
Authorization: Bearer {{apiSecret}}
HTTP 404

# 3. Happy path: create a throwaway row, delete it → 204, then re-delete → 404
#    (confirms both the delete and its idempotency).
POST {{rustUrl}}/recipe/ingredients
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"slug":"hurl-temp-del","name_ko":"임시","name_ja":"一時","name_en":"Temp","category":"seasoning"}
HTTP 201
[Captures]
del_id: jsonpath "$.id"

DELETE {{rustUrl}}/recipe/ingredients/{{del_id}}
Authorization: Bearer {{apiSecret}}
HTTP 204

DELETE {{rustUrl}}/recipe/ingredients/{{del_id}}
Authorization: Bearer {{apiSecret}}
HTTP 404
