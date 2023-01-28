FROM rust:1.67.0

WORKDIR /auto-commits-api

COPY . .

EXPOSE ${ROCKET_PORT}

CMD ["cargo", "run"]
