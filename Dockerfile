################################################################
#                            build                             #
################################################################
FROM rust:1.67.0-slim-buster as build

WORKDIR /app

COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build
RUN rm src/*.rs

################################################################
#                            app                               #
################################################################
FROM rust:1.67.0-slim-buster

COPY --from=build ./app/target/debug/auto-commits-api .

CMD ["./auto-commits-api"]
