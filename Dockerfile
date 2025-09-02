# Build stage
FROM rust:1.75 as builder

# Create app directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Copy tests for build validation
COPY tests ./tests

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install CA certificates for HTTPS requests
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false linebot

# Create app directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/linebot-rs /app/linebot-rs

# Change ownership to app user
RUN chown linebot:linebot /app/linebot-rs

# Switch to non-root user
USER linebot

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["./linebot-rs"]