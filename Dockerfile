FROM rust:1.72.0-buster as build 
RUN rustup component add rustfmt clippy
WORKDIR /app
ENV DEBIAN_FRONTEND noninteractive

RUN cargo install --root / sqlx-cli --no-default-features --features native-tls,sqlite
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN echo "fn main(){}" | tee rest.rs # create dummy file for cache depandanceis
RUN sed -i 's#src/bin/rest.rs#rest.rs#' Cargo.toml

RUN cargo build --release

COPY migrations ./migrations
COPY ./Cargo.toml ./Cargo.toml
COPY src ./src

RUN cargo build --release

#------------rest-------------
FROM ubuntu:22.04 as rest
COPY --from=build /bin/sqlx /bin
COPY  ./config /config
COPY --from=build /app/migrations /migrations
COPY --from=build /app/target/release/rest /

ENV RUST_LOG="info,sqlx=warn"
ENV RUN_MODE=production

EXPOSE 50051
CMD ["/bin/bash", "-c", "sqlx database create && sqlx migrate run && /rest"]

