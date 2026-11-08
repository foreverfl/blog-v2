# Update a sauce (secret or admin JWT)
# requires {{sauceId}}
PATCH {{rustUrl}}/recipe/sauces/{{sauceId}}
Authorization: Bearer {{recipeSecret}}
Content-Type: application/json
{
  "name_ko": "백미소"
}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{sauceId}}"
