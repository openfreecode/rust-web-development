GET http://localhost:3030/questions

HTTP 200
[Asserts]
jsonpath "$.id" == "1"

OPTIONS http://127.0.0.1:3030/questions

Access-Control-Request-Method: PUT
Access-Control-Request-Headers: content-type
Origin: https://not-origin.io

HTTP 200
[Asserts]
header "Access-Control-Allow-Origin" == "https://not-origin.io"
header "Access-Control-Allow-Methods" == "PUT, GET, POST, DELETE"
header "Access-Control-Allow-Headers" == "content-type"
