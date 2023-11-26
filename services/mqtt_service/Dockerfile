# Stage 1: Build the Rust application
FROM rust:1.74 as build

WORKDIR /usr/src/mqtt_service

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev cmake make gcc && \
    rm -rf /var/lib/apt/lists/*

COPY . .

# Build the Rust application
RUN cargo install --path .

# Stage 2: Create a minimal image with only the built binary
FROM debian:bookworm-slim

# Copy the binary from the build stage
COPY --from=build /usr/local/cargo/bin/mqtt_service /usr/local/bin/mqtt_service

RUN apt update && \
    apt install -y libssl3

# Set the default command to run the binary
CMD ["mqtt_service"]
