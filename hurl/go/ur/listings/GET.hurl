# UR vacant rooms passing the default filter (crawls live, takes ~1 min)
GET {{goUrl}}/ur/listings
Authorization: Bearer {{apiSecret}}
[Options]
connect-timeout: 10
HTTP 200
[Asserts]
jsonpath "$" isCollection
