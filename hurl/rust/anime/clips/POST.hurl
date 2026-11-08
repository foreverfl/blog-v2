POST {{rustUrl}}/anime/clips
Authorization: Bearer {{apiSecret}}
[Multipart]
series_slug: hurl-sample
series_title: ハール見本
episode: S01E01
start_sec: 12.5
duration_sec: 1
file: file,sample.mp4; video/mp4
HTTP 201
[Asserts]
jsonpath "$.r2_key" == "feed/hurl-sample-s01e01-13.mp4"
jsonpath "$.series_slug" == "hurl-sample"
jsonpath "$.series_title" == "ハール見本"
jsonpath "$.view_count" == 0
jsonpath "$.url" startsWith "https://assets-"
