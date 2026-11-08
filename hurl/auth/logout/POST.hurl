# Revoke the refresh token from Redis and clear the cookie.
POST {{authUrl}}/auth/logout
Cookie: refresh_token={{refreshToken}}
HTTP 200
