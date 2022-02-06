FROM rust as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian
COPY --from=builder /app/target/release/mysql-test /mysql-test
CMD ["/mysql-test"]