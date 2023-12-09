# Use the official Rust image as a base
FROM rust:1.71 as builder

# Clone repository
RUN git clone https://github.com/why-arong/heart-beat.git /heat
WORKDIR /heat

# Build application
RUN cargo build --release

# Final stage, using a newer Ubuntu version
FROM ubuntu:22.04

# Copy the built application
COPY --from=builder /heat/target/release/heat /usr/local/bin/heat

# Copy the check file (assuming it's in the heart-beat repository)
# and make it executable
COPY --from=builder /heat/check /usr/local/bin/heat/check
RUN chmod +x /usr/local/bin/heat/check

# Set the ENTRYPOINT to your binary
ENTRYPOINT ["/usr/local/bin/heat"]
