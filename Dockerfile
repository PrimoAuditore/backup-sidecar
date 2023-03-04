FROM rust as build
WORKDIR /app
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/backup-sidecar"]
