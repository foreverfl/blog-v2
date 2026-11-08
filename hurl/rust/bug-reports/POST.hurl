# Submit a bug report with an invalid Turnstile token → rejected.
# Idempotent: a bogus token always fails verification, so no Discord DM is sent.
POST {{rustUrl}}/bug-reports
Content-Type: application/json
{
  "turnstileToken": "invalid-token-for-test",
  "title": "hurl test",
  "content": "this should be rejected by turnstile"
}
HTTP 403
[Asserts]
jsonpath "$.error" exists
