# aws lambda create-function --function-name apex \
#   --handler doesnt.matter \
#   --zip-file fileb://./lambda.zip \
#   --runtime provided.al2 \
#   --role arn:aws:iam::691299561619:role/testLambda \
#   --environment Variables={RUST_BACKTRACE=1} \
#   --tracing-config Mode=Active \
#   --architectures arm64

aws lambda update-function-code \
  --function-name apex-lambda \
  --zip-file fileb://./lambda.zip \
  --architectures arm64 \
  --publish