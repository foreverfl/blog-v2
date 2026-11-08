# My visit records with a user JWT → 200, array (empty when none).
# The 401 without a token cannot be checked locally: DEV_USER_ID makes every
# local request the dev user. Verified in production only.
GET {{rustUrl}}/places/mine
Authorization: Bearer {{accessToken}}
HTTP 200
[Asserts]
jsonpath "$" count >= 0
