FROM rust:1 as builder
RUN apt install libsqlite3-dev -y
RUN rustup default nightly
WORKDIR /usr/pygmy-backend
COPY . .
RUN cargo build

FROM debian:buster-slim
WORKDIR /usr/pygmy-backend
RUN apt update && apt install libsqlite3-0 -y && apt clean
COPY --from=builder /usr/pygmy-backend/target/debug/catalog-server    /usr/local/bin/catalog-server
COPY --from=builder /usr/pygmy-backend/target/debug/order-server      /usr/local/bin/order-server
COPY --from=builder /usr/pygmy-backend/bootstrap.sh                     .
COPY --from=builder /usr/pygmy-backend/db                               .
RUN chmod +x bootstrap.sh
ENTRYPOINT ["./bootstrap.sh"]