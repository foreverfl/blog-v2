# DELETE /anime/clips/{id} — delete the R2 object and the row together (auth: API secret)
# Happy path is destructive (a real clip disappears), so this file keeps only
# the idempotent checks; the real deletion is exercised by clip-nightly.sh.

# 1. No auth → 401
DELETE {{rustUrl}}/anime/clips/1
HTTP 401

# 2. Unknown id → 404
DELETE {{rustUrl}}/anime/clips/999999
Authorization: Bearer {{apiSecret}}
HTTP 404
