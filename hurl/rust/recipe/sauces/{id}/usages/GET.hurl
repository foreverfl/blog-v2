# List a sauce's usages (public)
# requires {{sauceId}}
GET {{rustUrl}}/recipe/sauces/{{sauceId}}/usages
HTTP 200
[Asserts]
jsonpath "$" count >= 0
