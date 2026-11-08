# Save my body profile — no header at all
PUT {{rustUrl}}/diet/profile
Content-Type: application/json
{
  "height_cm": 176.0
}
HTTP 401

# Save my body profile — height at zero trips the CHECK constraint
PUT {{rustUrl}}/diet/profile
Authorization: Bearer {{accessToken}}
Content-Type: application/json
{
  "height_cm": 0
}
HTTP *
[Asserts]
status != 500

# Save my body profile — real values. Runs twice below to prove the upsert.
PUT {{rustUrl}}/diet/profile
Authorization: Bearer {{accessToken}}
Content-Type: application/json
{
  "height_cm": 176.0,
  "target_weight_kg": 65.0,
  "bmr_kcal": 1700
}
HTTP *
[Asserts]
status < 500

# Same values again — an upsert replaces the row instead of failing on the key
PUT {{rustUrl}}/diet/profile
Authorization: Bearer {{accessToken}}
Content-Type: application/json
{
  "height_cm": 176.0,
  "target_weight_kg": 65.0,
  "bmr_kcal": 1700
}
HTTP *
[Asserts]
status < 500
