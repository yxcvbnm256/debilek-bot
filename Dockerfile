FROM rust:1.87 AS builder
# Create a new empty shell project
WORKDIR /usr/src/debilek-bot

# Install required tools and dependencies
RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    build-essential \
    pkg-config

# Install CMake 3.27.0
RUN curl -L https://github.com/Kitware/CMake/releases/download/v3.27.0/cmake-3.27.0-linux-x86_64.tar.gz -o cmake.tar.gz && \
    tar -xzf cmake.tar.gz && \
    cp -r cmake-3.27.0-linux-x86_64/bin/* /usr/local/bin/ && \
    cp -r cmake-3.27.0-linux-x86_64/share/* /usr/local/share/ && \
    rm -rf cmake.tar.gz cmake-3.27.0-linux-x86_64

# Copy your source code
COPY . .



# Build the application in release mode
RUN cargo install --path .

# Stage 2: Create a minimal image with just the binary
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/debilek-bot/target/release/debilek-bot /usr/local/bin/debilek-bot/

COPY --from=builder /usr/src/debilek-bot/assets /usr/local/bin/assets
COPY --from=builder /usr/src/debilek-bot/.env /usr/local/bin/.env

ENTRYPOINT ["/usr/local/bin/debilek-bot"]