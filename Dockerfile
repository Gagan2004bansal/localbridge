# Use official Rust image with cargo
FROM rust:1.76

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build the release binary
RUN cargo build --release

# Expose the port your tunnel server will run on
EXPOSE 8080

# Run the server on port 8080
CMD ["./target/release/localbridge", "server", "--port", "8080"]
