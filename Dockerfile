FROM rust:1.87 AS builder
# Create a new empty shell project
WORKDIR /usr/src/debilek-bot

# Copy your source code
COPY . .

RUN cargo install cross --git https://github.com/cross-rs/cross

RUN apt-get update && apt-get install -y docker.io

RUN systemctl start docker

# Build the application in release mode
RUN cross build --target aarch64-unknown-linux-gnu --release



# Stage 2: Create a minimal image with just the binary
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /app

# Copy binary
COPY --from=builder /usr/src/debilek-bot/target/aarch64-unknown-linux-gnu/release/debilek-bot /usr/local/bin/debilek-bot
RUN chmod +x /usr/local/bin/debilek-bot

# Copy runtime files (assets and .env)
COPY --from=builder /usr/src/debilek-bot/assets ./assets
COPY --from=builder /usr/src/debilek-bot/.env ./.env

EXPOSE 3000

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/debilek-bot"]