# PATCH /recipe/ingredients/{id} — shared-secret partial update

# 1. Missing Bearer secret → 401 (auth checked before any update)
PATCH {{rustUrl}}/recipe/ingredients/00000000-0000-0000-0000-000000000000
Content-Type: application/json
{"name_ko":"x"}
HTTP 401

# 2. Authorized but unknown id → 404
PATCH {{rustUrl}}/recipe/ingredients/00000000-0000-0000-0000-000000000000
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"name_ko":"x"}
HTTP 404

# 3. Happy path: create a throwaway row, patch only name_ko, confirm the
#    omitted fields keep their value (COALESCE), then delete to stay rerunnable.
POST {{rustUrl}}/recipe/ingredients
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"slug":"hurl-temp-patch","name_ko":"임시","name_ja":"一時","name_en":"Temp","category":"seasoning"}
HTTP 201
[Captures]
patch_id: jsonpath "$.id"

PATCH {{rustUrl}}/recipe/ingredients/{{patch_id}}
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"name_ko":"바뀐임시"}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{patch_id}}"
jsonpath "$.name_ko" == "바뀐임시"
jsonpath "$.name_ja" == "一時"
jsonpath "$.slug" == "hurl-temp-patch"

# cleanup
DELETE {{rustUrl}}/recipe/ingredients/{{patch_id}}
Authorization: Bearer {{apiSecret}}
HTTP 204
