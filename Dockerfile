FROM rust:1.58.1 as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev
RUN update-ca-certificates

WORKDIR /app

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

# FROM debian
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/mysql-test /mysql-test
CMD ["/mysql-test"]