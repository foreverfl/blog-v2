# List a sauce's ingredients (public)
# requires {{sauceId}}
GET {{rustUrl}}/recipe/sauces/{{sauceId}}/ingredients
HTTP 200
[Asserts]
jsonpath "$" count >= 0
