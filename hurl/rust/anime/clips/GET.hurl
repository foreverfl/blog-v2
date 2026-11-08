GET {{rustUrl}}/anime/clips?limit=10
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$" count >= 1
jsonpath "$[0].id" exists
jsonpath "$[0].thumbnail_url" contains ".webp"
