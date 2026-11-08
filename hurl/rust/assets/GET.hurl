# List uploaded assets (paginated, newest first).
# Auth: shared API secret as a Bearer token (a user JWT also works).
# Response: { items: [AssetResponse], total, page, per_page }.
GET {{rustUrl}}/assets?page=1&per_page=20
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.items" isCollection
jsonpath "$.total" isInteger
jsonpath "$.page" == 1
jsonpath "$.per_page" == 20

# Without auth the listing is rejected.
GET {{rustUrl}}/assets
HTTP 401
