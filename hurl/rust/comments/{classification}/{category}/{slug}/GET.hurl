# List comments on an existing post → 200 array (possibly empty).
GET {{rustUrl}}/comments/development/lab/002-k8s-pod
HTTP 200
[Asserts]
jsonpath "$" isCollection

# Unknown post → 404.
GET {{rustUrl}}/comments/nope/nope/nope
HTTP 404
[Asserts]
jsonpath "$.error" exists
