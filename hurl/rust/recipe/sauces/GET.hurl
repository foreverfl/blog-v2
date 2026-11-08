# List sauces (public). Optional filter: ?cuisine=JP
GET {{rustUrl}}/recipe/sauces?cuisine=JP
HTTP 200
[Asserts]
jsonpath "$" count >= 0
