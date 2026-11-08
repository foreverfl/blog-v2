# Translate a date to ko
POST {{goUrl}}/hackernews/translate/{{hackerNewsDate}}?lang=ko
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
