FROM rust:latest

# Set working directory
WORKDIR /app

# Copy Cargo files and download dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy project files
COPY . ./

# Build the application
RUN cargo build --release

# Expose the port (matches SERVICE_PORT in .env)
EXPOSE 8081

# Command to run the application
CMD ["./target/release/image_converter_service"]
