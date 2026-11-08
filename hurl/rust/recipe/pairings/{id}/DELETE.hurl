# Delete a pairing (secret or admin JWT). Sub-ingredients cascade.
# requires {{pairingId}}
DELETE {{rustUrl}}/recipe/pairings/{{pairingId}}
Authorization: Bearer {{recipeSecret}}
HTTP 204
