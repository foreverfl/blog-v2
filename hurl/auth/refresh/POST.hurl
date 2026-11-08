# Send refresh_token cookie to get a new access_token.
# Set refreshToken in the env file after login.
POST {{authUrl}}/auth/refresh
Cookie: refresh_token={{refreshToken}}
HTTP 200
