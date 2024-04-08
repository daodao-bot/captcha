FROM rust:latest as builder
COPY . .
RUN cargo build --release

FROM debian:latest
COPY --from=builder /target/release/captcha .
ENTRYPOINT ["/captcha"]
