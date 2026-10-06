# src/convert/mod.rs を src/convert.rs にリネームする

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-convert-mod-rs
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/convert/mod.rs` が shiguredo-rust 規約（「`mod.rs` を使わないこと。モジュールは `<module>.rs` で書くこと。サブモジュールを持つ場合も `<module>.rs` + `<module>/<submodule>.rs` の構成にすること」）に違反している。`src/convert.rs` にリネームして規約準拠にする。

## 優先度根拠

Medium。

- プロジェクト規約違反の残骸
- rename のみでコードの挙動は変わらない

## 現状

- `src/convert/mod.rs` が存在し、`mod argb;` 等のサブモジュール宣言と `pub use argb::*;` 等の re-export を含む
- サブモジュール（`src/convert/argb.rs` 等 10 ファイル）はすでに `<module>/<submodule>.rs` 形式で配置済み

## 設計方針

`git mv src/convert/mod.rs src/convert.rs` でリネームする。`src/lib.rs` の `mod convert;` はそのままで解決される（Rust は `convert.rs` と `convert/` ディレクトリの組をサポートする）。

## 完了条件

- `src/convert.rs` が存在し、`src/convert/mod.rs` が存在しないこと
- `cargo check --workspace` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. `src/convert/mod.rs` を `src/convert.rs` にリネームする
2. ビルド・テストで確認する
3. `CHANGES.md` の `### misc` にエントリを追加する
