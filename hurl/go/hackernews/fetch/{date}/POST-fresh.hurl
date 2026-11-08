# Fetch HN items for a date (force refresh)
POST {{goUrl}}/hackernews/fetch/{{hackerNewsDate}}?fresh=true
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
