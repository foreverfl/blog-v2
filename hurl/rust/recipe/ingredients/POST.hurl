# POST /recipe/ingredients — shared-secret write + constraint mapping
# Rerunnable cases only. The 201 happy path persists a row; add it paired with
# DELETE /recipe/ingredients/{slug} once that endpoint exists.

# 1. Missing Bearer secret → 401 (auth checked before any insert)
POST {{rustUrl}}/recipe/ingredients
Content-Type: application/json
{"slug":"auth-test","name_ko":"x","name_ja":"x","name_en":"x"}
HTTP 401

# 2. Duplicate slug (seed 'tofu' always exists) → 409
POST {{rustUrl}}/recipe/ingredients
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"slug":"tofu","name_ko":"두부","name_ja":"豆腐","name_en":"Tofu","category":"vegetable"}
HTTP 409

# 3. Invalid slug format (CHECK violation) → 400
POST {{rustUrl}}/recipe/ingredients
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"slug":"Bad Slug","name_ko":"x","name_ja":"x","name_en":"x"}
HTTP 400

# 4. Happy path: create a throwaway row → 201, then delete it to stay rerunnable.
#    (If a prior run was interrupted between create and cleanup, rerun yields 409.)
POST {{rustUrl}}/recipe/ingredients
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"slug":"hurl-temp","name_ko":"임시","name_ja":"一時","name_en":"Temp","category":"seasoning"}
HTTP 201
[Asserts]
jsonpath "$.id" exists
jsonpath "$.slug" == "hurl-temp"
jsonpath "$.name_ko" == "임시"
[Captures]
temp_id: jsonpath "$.id"

# cleanup
DELETE {{rustUrl}}/recipe/ingredients/{{temp_id}}
Authorization: Bearer {{apiSecret}}
HTTP 204
