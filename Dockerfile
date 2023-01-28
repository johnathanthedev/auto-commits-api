FROM rust:1.67.0

WORKDIR /auto-commits-api

COPY . .

EXPOSE ${ROCKET_PORT}

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run", "-w", "src"]