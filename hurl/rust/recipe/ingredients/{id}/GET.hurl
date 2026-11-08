# GET /recipe/ingredients/{id} — single ingredient, read-only (no auth)

# 1. Malformed uuid → 400 (Path<Uuid> parse failure, before any query)
GET {{rustUrl}}/recipe/ingredients/not-a-uuid
HTTP 400

# 2. Well-formed but unknown id → 404
GET {{rustUrl}}/recipe/ingredients/00000000-0000-0000-0000-000000000000
HTTP 404

# 3. Happy path: create a throwaway row, fetch it by id, then delete to stay
#    rerunnable. (Create/delete need the secret; the GET itself is public.)
POST {{rustUrl}}/recipe/ingredients
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"slug":"hurl-temp-get","name_ko":"임시","name_ja":"一時","name_en":"Temp","category":"seasoning"}
HTTP 201
[Captures]
get_id: jsonpath "$.id"

GET {{rustUrl}}/recipe/ingredients/{{get_id}}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{get_id}}"
jsonpath "$.slug" == "hurl-temp-get"
jsonpath "$.name_ja" == "一時"

# cleanup
DELETE {{rustUrl}}/recipe/ingredients/{{get_id}}
Authorization: Bearer {{apiSecret}}
HTTP 204
