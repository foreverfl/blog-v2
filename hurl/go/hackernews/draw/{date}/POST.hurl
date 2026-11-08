# Draw images for a date
POST {{goUrl}}/hackernews/draw/{{hackerNewsDate}}
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
