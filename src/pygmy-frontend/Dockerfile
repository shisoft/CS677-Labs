FROM rust:1 as builder
RUN rustup default nightly
WORKDIR /usr/pygmy-frontend
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/pygmy-frontend
COPY --from=builder /usr/pygmy-frontend/target/release/pygmy-frontend    /usr/local/bin/pygmy-frontend
ENTRYPOINT ["pygmy-frontend"]