# Use the official Rust image as a base
FROM rust:1.71 as builder

# Clone your repository
RUN git clone https://github.com/why-arong/heart-beat.git /heat
WORKDIR /heat

# Build your application
RUN cargo build --release

# Add execute permission to all files in the directory
RUN chmod -R +x /heat

# Final stage, using a newer Ubuntu version
FROM ubuntu:22.04
COPY --from=builder /heat/target/release/heat /usr/local/bin/heat
COPY --from=builder /heat/check /usr/local/bin/check
COPY --from=builder /heat/failure.sh /usr/local/bin/failure.sh
COPY --from=builder /heat/recovery.sh /usr/local/bin/recovery.sh

# Install curl in the Ubuntu container
RUN apt-get update && apt-get install -y curl

# Set the ENTRYPOINT to your binary
ENTRYPOINT ["/usr/local/bin/heat"]
