# Remove an ingredient from a sauce (secret or admin JWT)
# requires {{sauceId}} and {{ingredientId}}
DELETE {{rustUrl}}/recipe/sauces/{{sauceId}}/ingredients/{{ingredientId}}
Authorization: Bearer {{recipeSecret}}
HTTP 204
