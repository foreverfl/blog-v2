# Like status on an existing post → 200 { like_count, liked }.
# No token → liked is false (public read).
GET {{rustUrl}}/likes/development/lab/002-k8s-pod
HTTP 200
[Asserts]
jsonpath "$.like_count" isInteger
jsonpath "$.liked" == false

# Unknown post → 404.
GET {{rustUrl}}/likes/nope/nope/nope
HTTP 404
[Asserts]
jsonpath "$.error" exists
