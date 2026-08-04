# 残存する #[allow(...)] を除去・置換して lint 抑制を整理する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-allow-to-expect
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/lib.rs` の `#[allow(dead_code)]` 14 箇所と `src/sys.rs` の `#![allow(clippy::all)]` が shiguredo-rust 規約（「lint 警告を抑制する必要があるときは `#[allow(...)]` ではなく `#[expect(...)]` を使うこと」）に違反したまま残っている。ただし `#[expect(dead_code)]` は lint の発火を期待する属性であり、**使用されている** validate では発火しないため `unfulfilled_lint_expectations` 警告になる（実機検証済み）。正しい終状態は、0048 と 0061 の完了後にマクロ内の 14 箇所の属性を**すべて削除**することであり、本 issue はその順序で lint 抑制を整理する。あわせて `CHANGES.md` の誤記載（「lint 抑制を allow から expect に置換する」完了済み）を実態へ合わせて修正する。

## 優先度根拠

Medium。

- 規約違反の残骸であり、lint が不要になったときに気づけない状態
- `CHANGES.md` の記載（置換完了）と現状（allow 残存）が矛盾している

## 現状

- `src/lib.rs`: マクロ（`define_yuv_image` 等 7 マクロ）で生成される `pub(crate) fn validate` の直前に `#[allow(dead_code)]` が 14 箇所ある（マクロ定義本体。全画像型に無差別に適用されるため、型ごとの属性変更はできない）
- デッド `validate` は計 21 個（未使用 19 型分 + `Nv16Image` / `P210Image` の 2 型分。実機 clippy 検証で `method 'validate' is never used` が 21 件出ることを確認済み）。`Nv16Image` / `P210Image` は `nv16_to_nv24` / `p210_to_p410` が手書き検証のためデッドであり、issue 0048 で生成済み validate に置換され使用済みになる。`Nv24Image` / `Nv24ImageMut` の validate は `nv24_scale` で**使用済み**でありデッドではない
- 型自体が未使用の 19 型（`Ab30Image` / `AyuvImageMut` / `Yuv24Image` / `Nv16ImageMut` / `H010ImageMut` / `U010ImageMut` / `H210ImageMut` / `U210ImageMut` / `P212Image` / `P410Image` の 10 型に加え、`Mm21ImageMut` / `Mt2tImageMut` / `Android420ImageMut` / `H420ImageMut` / `U420ImageMut` / `U422ImageMut` / `H422ImageMut` / `H444ImageMut` / `U444ImageMut` の 9 型。実測）の validate は完全にデッド。型の削除は issue 0061 で対応（0061 の列挙 10 型に加えて 9 型の追加が必要）
- `src/sys.rs`: `#![allow(clippy::all)]` が残っている（他の抑制は `#![expect(...)]` 化済み。なお 0032 で一度 expect 化されたが allow に戻された経緯がある。現行 bindings では clippy::all が発火するため expect 化は macOS arm64 / rustc 1.97.0 の `cargo clippy --workspace --features source-build -- -D warnings` で実機検証済み。bindgen の生成コードはプラットフォーム依存のため、CI の他プラットフォームでは未充足が出る可能性がある点に注意）
- `CHANGES.md` の develop セクション: 「[UPDATE] lint 抑制を #[allow(...)] から #[expect(...)] に置換する」と記載されているが、現状は上記の通り置換しきれていない（0032 の実装時に完了形のエントリが残された）

## 設計方針

- 使用されている `validate` の `#[allow(dead_code)]` は `#[expect(dead_code)]` ではなく**属性の削除**とする（`#[expect]` は使用中のコードでは unfulfilled 警告になるため）。0048（`Nv16Image` / `P210Image` の validate を使用済みにする）と 0061（未使用 19 型の削除）の完了後は、マクロ内 14 箇所の属性がすべて不要になり、一括削除できる。**0057 は 0048 と 0061 の完了後に実施する（順序依存。完了前は未使用 validate の dead_code 警告が出るため、属性削除で clippy の `-D warnings` が失敗する）**
- デッド `validate` の個別削除はマクロ構造上不可能（全型に無条件生成）であり、型の削除（0061）とともに消える。0057 は型削除を伴わない作業（属性の削除）のみを担当する
- `src/sys.rs` の `#![allow(clippy::all)]` は `#![expect(clippy::all)]` に置換する（macOS arm64 / rustc 1.97.0 の `cargo clippy --workspace --features source-build -- -D warnings` で実機検証済み・未充足警告なし。0032 で戻された経緯があるが、現行 bindings では clippy::all が発火するため充足される。万一 CI の他プラットフォームで未充足になった場合は `#[allow]` に戻さず、該当の属性を削除する）
- `CHANGES.md` の記載を置換完了後の実態に合わせて修正する（既存エントリ「[UPDATE] lint 抑制を allow から expect に置換する」は 0032 の実施分（sys.rs の置換）しか言い表しておらず、0057 完了後は lib.rs 側が削除になるため言い回しの見直しが必要。0057 の新規エントリ追加とは別に、既存エントリを実態に合わせて修正する）

## 完了条件

- 0048 と 0061 が完了していること（本 issue の前提。0061 は未使用 19 型すべての削除を意味し、0061 の列挙に 9 型が反映されていることを確認する。9 型の追加は 0061 の実施時に 0061 側で行う）
- `src/` に `#[allow(...)]` が存在しないこと（rg で確認）
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること（CI と同一コマンド。`#[expect]` の未充足警告がないこと）
- `cargo fmt --all --check` が成功すること
- `CHANGES.md` の誤記載が修正されていること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[UPDATE]` エントリを追加すること

## 解決方法

1. 0048 / 0061 を完了させる（本 issue の前提。0061 は実施時に未使用型列挙へ 9 型を追加して 19 型すべての削除を確定させる。追加の依頼は 0061 の実装者に対して行う）
2. マクロ内 14 箇所の `#[allow(dead_code)]` を削除する（使用・未使用の区別なく、0048 / 0061 完了後は全 validate が使用済みのため）
3. `src/sys.rs` の `#![allow(clippy::all)]` を `#![expect(clippy::all)]` に置換する
4. `CHANGES.md` の誤記載を実態に合わせて修正し、`### misc` に `[UPDATE]` エントリを追加する
