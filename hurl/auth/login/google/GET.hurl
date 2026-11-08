# Start Google OAuth login. Redirects (302) to the provider's auth URL.
# Provider can be google, github, etc.
# No status assert: behaves differently per env (local 302 vs gateway routing).
GET {{authUrl}}/auth/login/google
