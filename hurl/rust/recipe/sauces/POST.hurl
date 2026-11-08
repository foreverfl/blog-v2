# Create a sauce (secret or admin JWT)
POST {{rustUrl}}/recipe/sauces
Authorization: Bearer {{recipeSecret}}
Content-Type: application/json
{
  "cuisine_code": "JP",
  "name_ko": "미소",
  "name_ja": "味噌",
  "name_en": "miso"
}
HTTP 201
[Asserts]
jsonpath "$.id" exists
