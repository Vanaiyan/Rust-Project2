# Use an official Rust image as a builder
FROM rust:1.82 AS builder

WORKDIR /app

# Copy only the necessary files for dependency resolution
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY entity ./entity
COPY migration ./migration

# Build the application
RUN cargo build --release

# Stage 5: Create minimal runtime image
FROM ubuntu:22.04 AS runtime

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/product_service /usr/local/bin/product_service

# Copy .env file (optional, ensure it's not in .dockerignore)
COPY .env /app/

# Set environment variable for database connection
ENV DATABASE_URL="postgres://postgres:V.Abinajan30@34.55.1.248:5432/postgres"
ENV PORT=8080

# Expose the port your application listens on (adjust if different)
EXPOSE ${PORT}

# Set the startup command using JSON syntax
CMD ["/usr/local/bin/product_service"]