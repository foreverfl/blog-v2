# Translate today to ja
POST {{goUrl}}/hackernews/translate?lang=ja
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
