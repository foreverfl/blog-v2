# Create a pairing (dish combination) (secret or admin JWT)
# requires existing {{sauceId}} and {{ingredientId}} (main ingredient)
POST {{rustUrl}}/recipe/pairings
Authorization: Bearer {{recipeSecret}}
Content-Type: application/json
{
  "sauce_id": "{{sauceId}}",
  "ingredient_id": "{{ingredientId}}",
  "cooking_method_type_code": "grill",
  "score": 4,
  "note": null
}
HTTP 201
[Asserts]
jsonpath "$.id" exists
