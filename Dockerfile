# ビルドステージ
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /application

# 依存関係のキャッシュ最適化
COPY application/Cargo.toml application/Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 実際のソースコードをコピーしてビルド
COPY application/ .
RUN cargo build --release && \
    cargo test --release --no-run

# テスト実行ステージ（軽量版）
FROM rust:1.83-slim-bookworm

ENV CARGO_TERM_COLOR=always

WORKDIR /application

# ビルド成果物とソースをコピー
COPY --from=builder /application/target /application/target
COPY --from=builder /application/Cargo.toml /application/Cargo.lock ./
COPY --from=builder /application/src ./src
COPY --from=builder /application/tests ./tests

CMD ["cargo", "test", "--color=always"]