# 残存する #[allow(...)] を #[expect(...)] に置換し、デッド validate メソッドを整理する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-allow-to-expect
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/lib.rs` の `#[allow(dead_code)]` 14 箇所と `src/sys.rs` の `#![allow(clippy::all)]` が shiguredo-rust 規約（「`#[allow(...)]` を使わないこと（例外なし）。必ず `#[expect(...)]` を使うこと」）に違反したまま残っている。`#[expect]` に置換し、不要になった抑制は削除する。あわせて `CHANGES.md` の誤記載（「lint 抑制を allow から expect に置換する」完了済み）を修正する。

## 優先度根拠

Medium。

- 規約違反の残骸であり、lint が不要になったときに気づけない状態
- `CHANGES.md` の記載（置換完了）と現状（allow 残存）が矛盾している

## 現状

- `src/lib.rs`: マクロ（`define_yuv_image` 等）で生成される `pub(crate) fn validate` の直前に `#[allow(dead_code)]` が 14 箇所ある
- 使用されずに死んでいる `validate` メソッド: `Nv16Image` / `P210Image` / `Nv24ImageMut` / `Nv24Image` は「validate が呼ばれず手書き検証に置き換えられた」状態（issue 0048 で解消される）。また型自体が未使用の 10 型（`Ab30Image` / `AyuvImageMut` / `Yuv24Image` / `Nv16ImageMut` / `H010ImageMut` / `U010ImageMut` / `H210ImageMut` / `U210ImageMut` / `P212Image` / `P410Image`）の validate は完全にデッド（型の削除は issue 0061 で対応）
- `src/sys.rs`: `#![allow(clippy::all)]` が残っている（他の抑制は `#![expect(...)]` 化済み）
- `CHANGES.md` の develop セクション: 「[UPDATE] lint 抑制を #[allow(...)] から #[expect(...)] に置換する」と記載されているが、現状は上記の通り置換しきれていない

## 設計方針

- 使用されている `validate` の `#[allow(dead_code)]` は `#[expect(dead_code)]` に置換する
- デッド `validate`（使用されない型・呼び出されないメソッド）は `#[expect(dead_code)]` ではなく削除する（shiguredo-rust 規約: 公開 API 経由で到達できないコードはデッドコードとして削除を検討する）。ただし型自体は公開 API のため、型の削除は issue 0061 で判断する
- `src/sys.rs` の `#![allow(clippy::all)]` は `#![expect(clippy::all)]` に置換する（未充足の lint があれば clippy が検出する）
- `CHANGES.md` の記載を実態（置換が完了していること）に合わせて修正する

## 完了条件

- `src/` に `#[allow(...)]` が存在しないこと（grep で確認）
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` が成功すること（`#[expect]` の未充足警告がないこと）
- `CHANGES.md` の誤記載が修正されていること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. 使用されている `validate` の `#[allow(dead_code)]` を `#[expect(dead_code)]` に置換する
2. デッド `validate` メソッドを削除する（型は issue 0061 と調整して判断）
3. `src/sys.rs` の `#![allow(clippy::all)]` を `#![expect(clippy::all)]` に置換する
4. `CHANGES.md` の誤記載を修正し、`### misc` にエントリを追加する
