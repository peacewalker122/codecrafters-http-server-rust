FROM rust:1.73-slim as builder
RUN USER=root cargo new --bin http-server-starter-rust

COPY . /app
WORKDIR /app

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/http-server-starter-rust ./server

EXPOSE 4221

CMD [ "/app/server" ,"-p", "4221"]
