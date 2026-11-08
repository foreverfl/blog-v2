# Fetch HN items today
POST {{goUrl}}/hackernews/fetch
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
