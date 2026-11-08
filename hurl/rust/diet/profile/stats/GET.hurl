# Derived numbers — no header at all
GET {{rustUrl}}/diet/profile/stats
HTTP 401

# Derived numbers — weight passed in, the way the edit modal previews it.
# 200 once a profile exists, 404 before that, 401 on a stale token.
GET {{rustUrl}}/diet/profile/stats
[Query]
weight: 80
Authorization: Bearer {{accessToken}}
HTTP *
[Asserts]
status < 500

# Derived numbers — a weight that cannot be
GET {{rustUrl}}/diet/profile/stats
[Query]
weight: 0
Authorization: Bearer {{accessToken}}
HTTP *
[Asserts]
status != 200
status < 500
