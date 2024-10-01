# デモ

## 実装している機能

- **バックエンド**
  - `POST /api/v1/run`: 与えられたプログラムを実行する
  - `POST /api/v1/grade`: 与えられた問題に対して、与えられたプログラムを実行して、その結果を返す
- **フロントエンド**
  - `/problems/sample-problem`: サンプルの問題ページ
    - [x] 問題文を表示
    - [x] プログラムを入力するエディタ
    - [x] プログラムを実行するボタン
    - [x] テストの実行結果を表示

## 開発

### 必要なもの

- [Rust](https://www.rust-lang.org/)
- [Bun](https://bun.sh)

### 起動方法

1. **バックエンドサーバーを起動する**

   ```sh
   cd backend
   cargo run --release
   ```

2. **フロントエンドサーバーを起動する**

   ```sh
   cd frontend
   bun install --frozen-lockfile
   bun run dev
   ```
