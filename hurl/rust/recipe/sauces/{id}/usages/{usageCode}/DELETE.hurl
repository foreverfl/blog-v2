# Remove one usage from a sauce (secret or admin JWT)
# requires {{sauceId}} and {{usageCode}} (e.g. soup_base)
DELETE {{rustUrl}}/recipe/sauces/{{sauceId}}/usages/{{usageCode}}
Authorization: Bearer {{recipeSecret}}
HTTP 204
