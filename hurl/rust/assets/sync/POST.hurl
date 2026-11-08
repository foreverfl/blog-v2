# POST /assets/sync — reconcile the asset table with R2 (source of truth).
# Rerunnable by design: the endpoint is idempotent, so after one real run a
# second run must report zero changes.
# Auth: shared API secret as a Bearer token (a user JWT also works).

# 1. No auth → 401
POST {{rustUrl}}/assets/sync
HTTP 401

# 2. Unknown bucket → 400
POST {{rustUrl}}/assets/sync?bucket=definitely-not-a-bucket
Authorization: Bearer {{apiSecret}}
HTTP 400

# 3. Real run
POST {{rustUrl}}/assets/sync
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.inserted" isInteger
jsonpath "$.deleted" isInteger

# 4. Idempotence: immediately after a real run there is nothing left to do
POST {{rustUrl}}/assets/sync
Authorization: Bearer {{apiSecret}}
HTTP 200
[Asserts]
jsonpath "$.inserted" == 0
jsonpath "$.deleted" == 0
