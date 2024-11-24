FROM rust:1.82 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY src ./src

RUN cargo build --release

FROM rust:1.82

COPY --from=builder /usr/src/app/target/release/TyutyunShopBackend /app/

WORKDIR /app

CMD ["/app/TyutyunShopBackend"]
