# prek.toml のフック設定と CI の clippy 検査範囲を規約に合わせる

- Priority: Medium
- Created: 2026-08-04
- Completed: 2026-10-06
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

1. `prek.toml` を shiguredo-rust 規約と兄弟リポジトリの標準に合わせて書き換えた
   - `fail_fast` / `default_stages` / `default_install_hook_types = ["pre-commit", "pre-push"]` / `exclude` (`target/**` / `fuzz/target/**`) を設定した
   - builtin フックを 3 個から 16 個に拡充した（BOM 除去 / YAML / JSON / マージコンフリクト / 大文字小文字衝突 / Windows 予約名 / 大容量ファイル / 秘密鍵 / 改行コード / シンボリックリンク / shebang）
   - tombi の lint / format フックを追加した（`rev = "v1.7.2"`、`Cargo.lock` は対象外）
   - markdownlint-cli2 フックを追加した（`rev = "v0.23.3"`、リポジトリ既存の `.markdownlint.jsonc` を使用）
   - `cargo-test` に `stages = ["pre-push"]` を指定して pre-push 限定にした
2. `.github/workflows/ci.yml` の clippy を `cargo clippy --workspace --all-targets --features source-build -- -D warnings` に変更し、フックと検査範囲を揃えた
3. 追加した tombi フックで差分が出ないよう、`Cargo.toml` / `fuzz/Cargo.toml` を tombi で整形した
4. 追加した markdownlint-cli2 フックの既存違反 419 件（59 ファイル）を解消した
   - 自動修正（テーブル・空行・コードフェンス言語・番号リスト）と、300 文字超の行 62 件の句読点折り返しを行った
   - 自動修正が日本語文中の `*` / `_`（`width * 2` や `_12 / _16`）を行頭空白コードスパンと同様に壊すため、`.markdownlint.jsonc` に MD037 / MD038 と MD029 の無効化（理由コメント付き）と `line_length` を 300 に変更する設定を加えた
5. `prek update` で tombi-pre-commit / markdownlint-cli2 の rev を最新に揃えた（両方とも既に最新だった）
6. 検証結果
   - `prek validate-config` が成功した
   - `prek run --all-files` が全フック成功した
   - `prek install --prepare-hooks` で pre-commit / pre-push フックをインストールした
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings` が成功した
   - `cargo test --workspace --features source-build` が成功した
   - `npx markdownlint-cli2@0.23.3` が全 83 ファイルで 0 issues になった
