# Mark posts as indexed
PUT {{rustUrl}}/posts/mark-indexed
Authorization: Bearer {{accessToken}}
Content-Type: application/json
{
  "ids": ["00000000-0000-0000-0000-000000000000"]
}
HTTP *
[Asserts]
status >= 200
status < 300
