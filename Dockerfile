FROM rust:1.87 AS builder
# Create a new empty shell project
WORKDIR /usr/src/debilek-bot

# Install required tools and dependencies
RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    build-essential \
    pkg-config

# ➕ Add ARM64 support
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y libc6:arm64

RUN curl -L https://github.com/Kitware/CMake/releases/download/v3.27.0/cmake-3.27.0-linux-aarch64.tar.gz -o cmake.tar.gz && \
    tar -xzf cmake.tar.gz && \
    cp -r cmake-3.27.0-linux-aarch64/bin/* /usr/local/bin/ && \
    cp -r cmake-3.27.0-linux-aarch64/share/* /usr/local/share/ && \
    rm -rf cmake.tar.gz cmake-3.27.0-linux-aarch64

# Copy your source code
COPY . .



# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a minimal image with just the binary
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /app

# Copy binary
COPY --from=builder /usr/src/debilek-bot/target/release/debilek-bot /usr/local/bin/debilek-bot
RUN chmod +x /usr/local/bin/debilek-bot

# Copy runtime files (assets and .env)
COPY --from=builder /usr/src/debilek-bot/assets ./assets
COPY --from=builder /usr/src/debilek-bot/.env ./.env

EXPOSE 3000

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/debilek-bot"]