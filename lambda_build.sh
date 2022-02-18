
# Build the container for building a lambda function in.
docker build -t lambda-builder .

# Cleanup any dangling images.
docker rmi $(docker images -f "dangling=true" -q)

# Build the lambda function.
docker run -it --rm \
  -v ~/.cargo/registry:/root/.cargo/registry:z \
  -v "${PWD}":/build:z \
  lambda-builder
  