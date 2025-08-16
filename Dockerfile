FROM rust:1.83-bookworm

WORKDIR /application

COPY application/ .

RUN cargo build --release

CMD ["cargo", "test"]