# Update a post — authorized with the shared API_SECRET (CLI path)
PUT {{rustUrl}}/posts/dev/rust/hello-world
Authorization: Bearer {{apiSecret}}
Content-Type: application/json
{
  "contents": [
    {
      "lang": "ko",
      "content_type": "markdown",
      "title": "Hello World Updated",
      "excerpt": "An updated test post",
      "body_markdown": "# Hello\n\nThis post has been updated."
    }
  ]
}
HTTP 200
[Asserts]
jsonpath "$.slug" == "hello-world"
