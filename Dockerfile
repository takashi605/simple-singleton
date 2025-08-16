FROM rust:1.83-bookworm

ENV CARGO_TERM_COLOR=always

WORKDIR /application

COPY application/ .

RUN cargo build --release

CMD ["cargo", "test"]