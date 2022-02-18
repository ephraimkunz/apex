aws lambda update-function-code \
  --function-name apex-lambda \
  --zip-file fileb://./lambda.zip \
  --architectures arm64 \
  --publish