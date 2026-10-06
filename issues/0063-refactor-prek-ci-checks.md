# prek.toml のフック設定と CI の clippy 検査範囲を規約に合わせる

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-prek-ci-checks
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`prek.toml` の Git フック設定が shiguredo-rust 規約に合っていない（cargo-test が pre-commit で実行される・tombi フックがない）のと、CI の clippy が `--all-targets` なしでフックと検査範囲が食い違っている。規約準拠に修正する。

## 優先度根拠

Medium。

- 規約: 「`cargo test` は `pre-push` ステージだけで実行すること」「最低限 cargo fmt / cargo clippy / cargo test と tombi の lint / format をフックすること」
- フックと CI の検査範囲の不一致は、ローカルと CI で検出結果が食い違う原因

## 現状

- `prek.toml`: cargo-test フックに stage 指定がなく、既定の pre-commit で毎コミット実行される（規約は pre-push 限定）。`default_install_hook_types` も未設定のため pre-push フックがインストールされない可能性がある
- `prek.toml`: tombi の lint / format フックが存在しない
- `.github/workflows/ci.yml`: `cargo clippy --workspace --features source-build -- -D warnings`（`--all-targets` なし）。一方 `prek.toml` の clippy は `--all-targets` 付きで、テストコードの lint 範囲が食い違う

## 設計方針

- `prek.toml` の cargo-test フックに `stages = ["pre-push"]` を指定し、`default_install_hook_types = ["pre-commit", "pre-push"]` を設定する
- `prek.toml` に tombi の lint / format フックを追加する（テンプレート: `shiguredo-rust` スキルの参考設定を参照。Cargo.lock は対象外にする）
- `.github/workflows/ci.yml` の clippy に `--all-targets` を追加し、フックと CI の検査範囲を揃える

## 完了条件

- `prek.toml` の cargo-test が pre-push 限定になっていること
- `prek.toml` に tombi の lint / format フックが存在すること
- `prek validate-config` と `prek run --all-files` が成功すること
- `.github/workflows/ci.yml` の clippy が `--all-targets` 付きであること
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` が成功すること

## 解決方法

1. `prek.toml` を修正する（stage 指定・tombi フック・default_install_hook_types）
2. `.github/workflows/ci.yml` の clippy に `--all-targets` を追加する
3. `prek validate-config` と `prek run --all-files` で確認する
