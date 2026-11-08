# Recommend pairings (public, core). "두부 반모, 일본식으로 뭐 해먹지?"
# Filters (all optional): ?ingredient=tofu &cuisine=JP &method=grill
GET {{rustUrl}}/recipe/pairings?ingredient=tofu&cuisine=JP
HTTP 200
[Asserts]
jsonpath "$" count >= 0
