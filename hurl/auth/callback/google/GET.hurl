# OAuth callback. Normally hit by the provider after authorization.
# Replace code/state with real values before firing.
GET {{authUrl}}/auth/callback/google?code=AUTH_CODE&state=CSRF_STATE
# No status assert: placeholder code/state won't succeed.
