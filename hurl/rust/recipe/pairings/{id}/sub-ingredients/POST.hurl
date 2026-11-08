# Add a sub-ingredient to a pairing (secret or admin JWT)
# requires {{pairingId}} and an existing {{ingredientId}}
POST {{rustUrl}}/recipe/pairings/{{pairingId}}/sub-ingredients
Authorization: Bearer {{recipeSecret}}
Content-Type: application/json
{
  "ingredient_id": "{{ingredientId}}",
  "amount": 100,
  "unit": "g",
  "note": null,
  "sort_order": 0
}
HTTP 201
[Asserts]
jsonpath "$.pairing_id" == "{{pairingId}}"
