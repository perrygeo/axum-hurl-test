GET {{baseurl}}
HTTP 200
[Asserts]
header "Content-Type" contains "text/html"
header "Content-Type" contains "utf-8"

GET {{baseurl}}/does-not-exist-probably
HTTP 404
