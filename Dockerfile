FROM rust:1.82 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/TyutyunShopBackend /usr/local/bin/

WORKDIR /app

CMD ["/usr/local/bin/TyutyunShopBackend"]
