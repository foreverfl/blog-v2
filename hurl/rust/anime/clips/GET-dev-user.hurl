# Passes only with DEV_USER_ID set in blog-v2 infra/docker/.env.local — no token, still 200.
GET {{rustUrl}}/anime/clips?limit=10
Origin: http://MacBook-Pro.local:3000
HTTP 200
[Asserts]
header "Access-Control-Allow-Origin" == "http://MacBook-Pro.local:3000"
jsonpath "$" count >= 1
jsonpath "$[?(@.liked == true)]" count >= 1
