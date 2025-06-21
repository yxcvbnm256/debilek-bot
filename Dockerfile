
# Stage 2: Create a minimal image with just the binary
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev:arm64 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /app

# Copy binary
COPY  ./target/aarch64-unknown-linux-gnu/release/debilek-bot /usr/local/bin/debilek-bot
RUN chmod +x /usr/local/bin/debilek-bot

# Copy runtime files (assets and .env)
COPY --from=builder /usr/src/debilek-bot/assets ./assets
COPY --from=builder /usr/src/debilek-bot/.env ./.env

EXPOSE 3000

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/debilek-bot"]