FROM rust:1.44 as build

COPY ./ ./

RUN apt-get update && apt-get -y install libssl-dev

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --release

FROM debian:buster-slim

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build target/release/shakespearean-pokemon /

EXPOSE 8080

CMD ["/shakespearean-pokemon"]