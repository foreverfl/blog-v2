# Summarize today (force refresh)
POST {{goUrl}}/hackernews/summarize?fresh=true
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
