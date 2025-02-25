# Use the official Rust image as a base
FROM rust:latest

# Create a new directory for the action
WORKDIR /usr/src/fibbot

# Copy the Rust project files into the container
COPY . .

# Build the Rust project
RUN cargo install --path .

# Set the entry point for the action
ENTRYPOINT ["fibbot"]
