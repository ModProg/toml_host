FROM rust as build

RUN rustup default nightly

WORKDIR /usr/src
RUN USER=root cargo new toml_host
WORKDIR /usr/src/toml_host

# Caches build dependencies by writing placeholder lib and main files.
COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update
# RUN apt-get install -y libpq-dev libsqlite3-dev

COPY --from=build /usr/local/cargo/bin/toml_host /usr/local/bin/toml_host

EXPOSE 8000
CMD ["toml_host"]

