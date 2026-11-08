# Import HN items from Cloudflare R2 (hackernews/{YYMMDD}.json),
# from importFromDate (YYMMDD) to today. Requires Authorization: Bearer <API_SECRET>.
# Response returns job_id -> set it as importJobId to use get-import-job.hurl.
POST {{rustUrl}}/import/json?from={{importFromDate}}
Authorization: Bearer {{apiSecret}}
HTTP *
[Asserts]
status >= 200
status < 300
