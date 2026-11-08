# Summarize a date (force refresh)
POST {{goUrl}}/hackernews/summarize/{{hackerNewsDate}}?fresh=true
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
