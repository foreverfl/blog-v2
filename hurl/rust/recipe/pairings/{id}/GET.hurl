# Get one pairing by id (public) — composed with sub_ingredients[]
# requires {{pairingId}}
GET {{rustUrl}}/recipe/pairings/{{pairingId}}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{pairingId}}"
jsonpath "$.sub_ingredients" exists
