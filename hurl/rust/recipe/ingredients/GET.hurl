# List ingredients (public). Optional filters: ?category= &slug=
GET {{rustUrl}}/recipe/ingredients?category=vegetable
HTTP 200
[Asserts]
jsonpath "$" count >= 0
