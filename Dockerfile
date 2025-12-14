# Multi-stage build for charta-vm
# Note: charta-vm is a library, but we can create a minimal container for testing
FROM rust:1.75-slim as builder

WORKDIR /build

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy charta-core (dependency)
COPY charta-core ./charta-core

# Copy VM
COPY charta-vm/Cargo.toml charta-vm/Cargo.toml
COPY charta-vm/src ./charta-vm/src

# Build the VM library (for testing/validation)
WORKDIR /build/charta-vm
RUN cargo build --release

# Runtime stage - minimal container with built artifacts
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy built library (if needed for embedding)
COPY --from=builder /build/target/release/libcharta_vm*.rlib /usr/local/lib/

WORKDIR /app

# This is primarily a library, but we can use it for validation
CMD ["/bin/sh"]

