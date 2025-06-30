# Use the official Rust image as a parent image
FROM rust:1.75 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/axum-server /app/axum-server

# Change ownership to the non-root user
RUN chown appuser:appuser /app/axum-server

# Switch to the non-root user
USER appuser

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["./axum-server"] 