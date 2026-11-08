# Fetch HN items for a date
POST {{goUrl}}/hackernews/fetch/{{hackerNewsDate}}
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
