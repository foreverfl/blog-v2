# Remove a sub-ingredient from a pairing (secret or admin JWT)
# requires {{pairingId}} and {{ingredientId}}
DELETE {{rustUrl}}/recipe/pairings/{{pairingId}}/sub-ingredients/{{ingredientId}}
Authorization: Bearer {{recipeSecret}}
HTTP 204
