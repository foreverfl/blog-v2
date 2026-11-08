# Add an ingredient to a sauce (secret or admin JWT)
# requires {{sauceId}} and an existing {{ingredientId}}
POST {{rustUrl}}/recipe/sauces/{{sauceId}}/ingredients
Authorization: Bearer {{recipeSecret}}
Content-Type: application/json
{
  "ingredient_id": "{{ingredientId}}",
  "amount": 30,
  "unit": "g",
  "note": null,
  "sort_order": 0
}
HTTP 201
[Asserts]
jsonpath "$.id" exists
