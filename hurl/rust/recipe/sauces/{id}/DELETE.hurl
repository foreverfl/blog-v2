# Delete a sauce (secret or admin JWT). Children (ingredients/usages/pairings) cascade.
# requires {{sauceId}}
DELETE {{rustUrl}}/recipe/sauces/{{sauceId}}
Authorization: Bearer {{recipeSecret}}
HTTP 204
