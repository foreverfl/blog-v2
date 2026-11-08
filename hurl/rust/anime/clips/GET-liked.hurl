# GET /anime/clips?liked=true — liked clips only, newest like first, paged with offset (auth: API secret)

# 1. No auth → 401
GET {{rustUrl}}/anime/clips?liked=true
HTTP 401

# 2. First page: liked rows, liked_at present
GET {{rustUrl}}/anime/clips?liked=true&limit=2&offset=0
Authorization: Bearer {{apiSecret}}
HTTP 200
[Captures]
first_id: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].liked" == true
jsonpath "$[0].liked_at" exists

# 3. Next page starts past the first one (no overlap)
GET {{rustUrl}}/anime/clips?liked=true&limit=2&offset=2
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$[0].id" != {{first_id}}
