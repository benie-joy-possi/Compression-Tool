FROM rust:latest AS builder

# Set the working directory to /app
WORKDIR /app

# Copy the Rust code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

FROM alpine:latest

# Set the working directory to /app
WORKDIR /app

# Copy the compiled binary into the container
COPY --from=builder /app/target/ .

# Set the entry point for the application
CMD ["./rust-compressor"]


