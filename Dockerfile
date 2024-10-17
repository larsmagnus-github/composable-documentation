# Use the same base image for both build and runtime stages
FROM rust:1.75-bullseye AS builder

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy any other files needed (e.g., static files)
COPY static ./static

# Build the application in release mode
RUN cargo build --release

# Use the same base image for the runtime stage
FROM rust:1.75-bullseye

WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/composable-documentation .

# Copy any static files if needed
COPY --from=builder /usr/src/app/static ./static

# Expose the port your application listens on (e.g., 8080)
EXPOSE 8080

# Set the command to run the application
CMD ["./composable-documentation"]