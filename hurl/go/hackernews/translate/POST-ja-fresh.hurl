# Translate today to ja (force refresh)
POST {{goUrl}}/hackernews/translate?lang=ja&fresh=true
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
