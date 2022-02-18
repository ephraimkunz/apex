FROM amazonlinux:2

# Setup build environment
RUN mkdir -p /build/src && \
    yum update -y && \
# Add required packages
    yum groupinstall -y "Development Tools" && \
# Install rust with rustup
    curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal && \
    source $HOME/.cargo/env && \
    cargo install --no-default-features --force cargo-make

# Build environment setting
WORKDIR /build
ENV PATH=/root/.cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin
# Default build command
CMD \
  rm -f lambda.zip && \
  cargo make build && \
  cp target/release/apex bootstrap && \
  strip --strip-all bootstrap && \
  zip -r lambda.zip bootstrap src static templates && \
  rm bootstrap
