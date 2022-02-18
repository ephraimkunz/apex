export AWS_PAGER=""

aws lambda update-function-code \
  --function-name apex-lambda \
  --zip-file fileb://./lambda.zip \
  --architectures arm64 \
  --publish

echo "\nAccess the website:"
aws apigatewayv2 get-apis --no-paginate | jq -r '.Items[0].ApiEndpoint'
echo "\n"