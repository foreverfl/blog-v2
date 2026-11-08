# Fetch HN items today (force refresh)
POST {{goUrl}}/hackernews/fetch?fresh=true
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
