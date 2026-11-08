# Summarize a date
POST {{goUrl}}/hackernews/summarize/{{hackerNewsDate}}
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
