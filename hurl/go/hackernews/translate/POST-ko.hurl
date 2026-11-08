# Translate today to ko
POST {{goUrl}}/hackernews/translate?lang=ko
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
