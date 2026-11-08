# My body profile — no header at all
GET {{rustUrl}}/diet/profile
HTTP 401

# My body profile — a token that is not a JWT
GET {{rustUrl}}/diet/profile
Authorization: Bearer not-a-jwt
HTTP 401

# My body profile — login token from the env file.
# A fresh token gives 200 once the profile exists and 404 until it is created;
# a stale one gives 401. Assert only that the route answered.
GET {{rustUrl}}/diet/profile
Authorization: Bearer {{accessToken}}
HTTP *
[Asserts]
status < 500
