# Replace a sauce's usage set (secret or admin JWT). delete-then-insert.
# requires {{sauceId}}
PUT {{rustUrl}}/recipe/sauces/{{sauceId}}/usages
Authorization: Bearer {{recipeSecret}}
Content-Type: application/json
{
  "sauce_usage_type_codes": ["soup_base", "marinade"]
}
HTTP 200
[Asserts]
jsonpath "$" count >= 0
