# Use the official Rust image as a base
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to build the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies to cache them
RUN cargo build --release && rm -rf src

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use the official Debian image as a base for the runtime
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/rustathon .

# Copy the static assets
COPY static ./static

# Copy the templates
COPY templates ./templates

# Expose the port the app runs on
EXPOSE 3000

# Set the startup command to run the binary
CMD ["./rustathon"]
