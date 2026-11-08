# Translate a date to ko (force refresh)
POST {{goUrl}}/hackernews/translate/{{hackerNewsDate}}?lang=ko&fresh=true
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
