### マイグレーション

```sh
# マイグレーションファイルの生成
diesel migration generate ファイル名

# マイグレーション実行
diesel migration run

# down, upの同時確認
diesel migration redo
```

### 開発用サーバー立ち上げ

```sh
cargo watch -x run
```
