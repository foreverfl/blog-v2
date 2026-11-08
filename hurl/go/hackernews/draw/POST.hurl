# Draw images today
POST {{goUrl}}/hackernews/draw
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
