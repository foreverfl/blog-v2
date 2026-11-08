# Cooking method types (public, read-only) ordered by code
GET {{rustUrl}}/recipe/cooking-method-types
HTTP 200
[Asserts]
jsonpath "$" count > 0
jsonpath "$[0].code" exists
jsonpath "$[0].name_ko" exists
jsonpath "$[0].name_ja" exists
jsonpath "$[0].name_en" exists
