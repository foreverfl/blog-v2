# Summarize today
POST {{goUrl}}/hackernews/summarize
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
