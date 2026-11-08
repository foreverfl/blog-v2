# Job status requires auth (JWT or API_SECRET) — no token must be rejected.
# The auth check runs before the Redis lookup, so a random id still 401s.
GET {{rustUrl}}/import/jobs/00000000-0000-0000-0000-000000000000
HTTP 401
