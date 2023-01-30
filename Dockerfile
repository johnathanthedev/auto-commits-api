FROM rust:1.67.0-slim-buster as build

WORKDIR /app

COPY . .

RUN cargo build

FROM rust:1.67.0-slim-buster

RUN apt-get update -qq \
	&& apt-get install -y pkg-config libssl-dev make \
	&& cargo install sea-orm-cli \
	&& sea-orm-cli migrate refresh \
	&& rustup component add rustfmt

COPY --from=build /app/makefile .

RUN chmod +x makefile

COPY --from=build /app/target/debug/auto-commits-api .

COPY --from=build /app/migration/src /migration/src

COPY --from=build /app/migration/Cargo.lock /migration/Cargo.lock

COPY --from=build /app/migration/Cargo.toml /migration/Cargo.toml

COPY --from=build /app/src/entities /src/entities

CMD ["./auto-commits-api"]
