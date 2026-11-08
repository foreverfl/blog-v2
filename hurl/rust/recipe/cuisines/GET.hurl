# Cuisine classifications (public, read-only) ordered by code
GET {{rustUrl}}/recipe/cuisines
HTTP 200
[Asserts]
jsonpath "$" count > 0
jsonpath "$[0].code" exists
jsonpath "$[0].name_ko" exists
