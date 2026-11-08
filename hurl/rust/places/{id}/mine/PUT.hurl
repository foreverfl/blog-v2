# ホテルハーヴェスト那須: write applied_count 2 and a visit, then put it back
# to 0 / null, so the file can run again and again.
# The 401 without a token cannot be checked locally (DEV_USER_ID).
PUT {{rustUrl}}/places/ca849e85-5a0a-4a8a-a264-3724f70f2bbd/mine
Authorization: Bearer {{accessToken}}
Content-Type: application/json
{
  "applied_count": 2,
  "last_applied_on": "2026-10-13",
  "visited_on": "2026-12-25",
  "visited_with": "家族",
  "review": "test"
}
HTTP 200
[Asserts]
jsonpath "$.place_id" == "ca849e85-5a0a-4a8a-a264-3724f70f2bbd"
jsonpath "$.applied_count" == 2
jsonpath "$.visited_on" == "2026-12-25"

# Back to an empty record
PUT {{rustUrl}}/places/ca849e85-5a0a-4a8a-a264-3724f70f2bbd/mine
Authorization: Bearer {{accessToken}}
Content-Type: application/json
{
  "applied_count": 0,
  "last_applied_on": null,
  "visited_on": null,
  "visited_with": null,
  "review": null
}
HTTP 200
[Asserts]
jsonpath "$.applied_count" == 0
jsonpath "$.visited_on" == null

# Unknown place id → 404
PUT {{rustUrl}}/places/00000000-0000-0000-0000-000000000000/mine
Authorization: Bearer {{accessToken}}
Content-Type: application/json
{
  "applied_count": 1
}
HTTP 404
