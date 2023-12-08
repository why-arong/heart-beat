# Use the official Rust image as a base
FROM rust:1.71 as builder

# Clone your repository
RUN git clone https://github.com/why-arong/heart-beat.git /heat
WORKDIR /heat

# Build your application
RUN cargo build --release

# Final stage, using a newer Ubuntu version
FROM ubuntu:22.04
COPY --from=builder /heat/target/release/heat /usr/local/bin/heat

# Set the ENTRYPOINT to your binary
ENTRYPOINT ["/usr/local/bin/heat"]