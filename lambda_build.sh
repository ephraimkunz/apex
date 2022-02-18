# From https://github.com/hanabu/lambda-web

docker build -t lambda-builder .

docker rmi $(docker images -f "dangling=true" -q)

docker run -it --rm \
  -v ~/.cargo/registry:/root/.cargo/registry:z \
  -v "${PWD}":/build:z \
  lambda-builder
  