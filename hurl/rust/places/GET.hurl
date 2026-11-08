# Every place on /map (public, no auth), ordered by name
GET {{rustUrl}}/places
HTTP 200
[Asserts]
jsonpath "$" count >= 109
jsonpath "$[0].id" exists
jsonpath "$[0].lat" exists

# Only the 41 ITS resorts, with the ITS-only fields filled
GET {{rustUrl}}/places?subcategory=its
HTTP 200
[Asserts]
jsonpath "$" count == 41
jsonpath "$[0].category" == "stay"
jsonpath "$[0].kind" exists

# Only the whisky distilleries; ITS-only fields are null
GET {{rustUrl}}/places?category=brewery&subcategory=whisky
HTTP 200
[Asserts]
jsonpath "$" count == 68
jsonpath "$[0].kind" == null
