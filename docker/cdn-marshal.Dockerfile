# Use a temporary builder image
FROM docker.io/rust:1.76-bookworm as BUILDER

# Set up the working directory
WORKDIR /build
COPY . .

# Install necessary dependencies
RUN apt-get update && apt-get install -y capnproto

# Build our example
RUN RUSTFLAGS='--cfg async_executor_impl="tokio" --cfg async_channel_impl="tokio"' cargo build --profile release-lto --example marshal-push-cdn

# Use a minimal image for the final build
FROM debian:bookworm as RUNNER

# Install necessary dependencies
RUN apt-get update && apt-get install libcurl4 -y

# Set the Rust log level
ENV RUST_LOG=info

# Copy the built binary from the builder image
COPY --from=BUILDER ./build/target/release-lto/examples/marshal-push-cdn /bin/marshal-push-cdn

# Set the entrypoint
ENTRYPOINT ["/bin/marshal-push-cdn"]