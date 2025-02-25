# Use the official Rust image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/fibbot

# Copy the current directory contents into the container at /usr/src/fibbot
COPY . .

# Build the Rust application
RUN cargo build --release

# Run the application
ENTRYPOINT ["/usr/src/fibbot/target/release/fibbot"]
