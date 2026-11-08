# Get one sauce by id (public) — composed with ingredients[] and usages[]
# requires {{sauceId}}
GET {{rustUrl}}/recipe/sauces/{{sauceId}}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{sauceId}}"
jsonpath "$.ingredients" exists
jsonpath "$.usages" exists
