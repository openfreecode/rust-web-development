GET http://localhost:3030/questions

HTTP 200
[Asserts]
jsonpath "$[0].id" == "1"

OPTIONS http://127.0.0.1:3030/questions

Access-Control-Request-Method: PUT
Access-Control-Request-Headers: content-type
Origin: https://not-origin.io

HTTP 200
[Asserts]
header "Access-Control-Allow-Origin" == "https://not-origin.io"
header "Access-Control-Allow-Headers" == "content-type"

POST http://localhost:3030/questions

Content-Type: application/json

{
  "title": "What is the meaning of life?",
  "content": "42"
}

HTTP 422

POST http://localhost:3030/questions

Content-Type: application/json

{
  "id": "2",
  "title": "What is the meaning of life?",
  "content": "42"
}

HTTP 201


GET http://localhost:3030/questions/2

HTTP 200
[Asserts]
jsonpath "$.id" == "2"
jsonpath "$.title" == "What is the meaning of life?"
jsonpath "$.content" == "42"


PUT http://localhost:3030/questions

Content-Type: application/json

{
  "id": "3",
  "title": "What is the meaning of life?",
  "content": "42"
}

HTTP 405

PUT http://localhost:3030/questions/111

Content-Type: application/json

{
  "id": "111",
  "title": "What is the meaning of life?",
  "content": "42"
}

HTTP 404

PUT http://localhost:3030/questions/1

Content-Type: application/json

{
  "id": "1",
  "title": "What is the meaning of life?",
  "content": "42"
}

HTTP 200

PUT http://localhost:3030/questions/1

Content-Type: application/json

{
  "id": "1",
  "content": "42"
}

HTTP 422

DELETE http://localhost:3030/questions/2

HTTP 200

GET http://localhost:3030/questions/1

HTTP 200
[Asserts]
jsonpath "$.id" == "1"
jsonpath "$.content" == "42"