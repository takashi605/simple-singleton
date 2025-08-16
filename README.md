## このプロジェクトについて
GoF のシングルトンパターンを Rust で実装した、学習用プロジェクトです。
具体的な振る舞いは[結合テスト](application/tests/singleton_works.rs)を確認してください。

## リポジトリの使い方
リポジトリのクローン後、`docker compose up` を実行するとテストが実行され、シングルトンとしての振る舞いが実装されていることが確認できます。※Docker の実行環境が存在することが前提

```sh
docker compose up
```
