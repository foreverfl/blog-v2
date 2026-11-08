# Create a post — authorized with the shared API_SECRET (CLI path)
POST {{rustUrl}}/posts
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{
  "classification": "dev",
  "category": "rust",
  "slug": "hello-world",
  "image": null,
  "contents": [
    {
      "lang": "ko",
      "content_type": "markdown",
      "title": "Hello World",
      "excerpt": "A test post",
      "body_markdown": "# Hello\n\nThis is a test post."
    }
  ]
}
HTTP 201
[Asserts]
jsonpath "$.classification" == "dev"
jsonpath "$.category" == "rust"
jsonpath "$.slug" == "hello-world"
