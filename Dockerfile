FROM rust:alpine3.17 as build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.17.2 as runtime
WORKDIR /app
COPY --from=build /app/target/release/backup-sidecar .
ENTRYPOINT ["./backup-sidecar"]
CMD ["/data/dump.rdb"]
