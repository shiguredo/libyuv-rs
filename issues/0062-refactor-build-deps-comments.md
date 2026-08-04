# build-dependencies の用途コメントと DOCS_RS の rerun-if-env-changed を追加する

- Priority: Low
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-build-deps-comments
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`Cargo.toml` の build-dependencies（`bindgen` / `shiguredo_cmake` / `shiguredo_toml`）に用途コメントがなく、`build.rs` に `cargo::rerun-if-env-changed=DOCS_RS` の宣言がない。規約準拠とビルドキャッシュの正しさのために追加する。

## 優先度根拠

Low。

- 用途コメントの欠落は規約違反（shiguredo-rust: 依存ライブラリには用途をコメントで明記すること）
- `DOCS_RS` の rerun-if-env-changed 欠落は、同一 target ディレクトリで `DOCS_RS` の有無を切り替えても build.rs が再実行されず、stale な bindings でビルドが進む問題の原因

## 現状

- `Cargo.toml` の `[build-dependencies]`: `bindgen = "0.72"` / `shiguredo_cmake = "4.3"` / `shiguredo_toml = "2026.2"` に用途コメントがない（外部依存メタデータにはコメントがあるのに build-dependencies だけ欠落）
- `build.rs` の `main` 冒頭: `cargo::rerun-if-changed=Cargo.toml` / `cargo::rerun-if-changed=build.rs` / `cargo::rerun-if-env-changed=CARGO_FEATURE_SOURCE_BUILD` / `cargo::rerun-if-env-changed=LIBYUV_TARGET` は宣言済みだが、`DOCS_RS`（`build.rs` 内で `if env::var("DOCS_RS").is_ok()` により出力を切り替える）が宣言されていない

## 設計方針

- `Cargo.toml` の build-dependencies に用途コメント（例: `# bindgen: libyuv ヘッダから FFI バインディングを生成する`）を追加する
- `build.rs` に `cargo::rerun-if-env-changed=DOCS_RS` を追加する

## 完了条件

- build-dependencies の 3 つすべてに用途コメントが付いていること
- `build.rs` に `cargo::rerun-if-env-changed=DOCS_RS` が宣言されていること
- `DOCS_RS` の有無を切り替えたときに build.rs が再実行されること
- `cargo build` が成功すること

## 解決方法

1. `Cargo.toml` の build-dependencies に用途コメントを追加する
2. `build.rs` に `cargo::rerun-if-env-changed=DOCS_RS` を追加する
3. ビルドで確認する
