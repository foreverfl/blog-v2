# POST /anime/clips/{id}/playback-event — log one playback problem (auth: API secret)
# Stores nothing; the line lands in Loki as service_name="blog-rust-api".

# 1. No auth → 401
POST {{rustUrl}}/anime/clips/1/playback-event
Content-Type: application/json
{"event":"stall"}
HTTP 401

# 2. Body without event → 422 (axum rejects it before the handler runs)
POST {{rustUrl}}/anime/clips/1/playback-event
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"buffer_left_sec":0}
HTTP 422

# 3. Happy path: a stall on a slow line. Unknown ids are fine — nothing is read
#    from the database, the id only rides along in the log line.
POST {{rustUrl}}/anime/clips/1/playback-event
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{"event":"stall","buffer_left_sec":0.2,"downlink":0.4,"effective_type":"3g","stall_ms":4200,"url_age_sec":120,"muted":false,"volume":1,"session_id":"hurl-session"}
HTTP 204
