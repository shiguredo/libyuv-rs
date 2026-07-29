# #[allow(...)] を #[expect(...)] に置換または削除する

- Priority: Medium
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/refactor-allow-to-expect-lint
- Polished: 2026-07-29
- Reporter: @voluntas

## 目的

shiguredo-rust 規約「lint 抑制は `#[expect(...)]` を使う（`#[allow(...)]` ではなく）」に違反している `#[allow(...)]` を `#[expect(...)]` に置換または削除する。`#[allow(...)]` では lint 項目が不要になっても気づけないため。

## 優先度根拠

Medium。規約違反であり動作への影響はない。ただし後述のとおりマクロ内の `#[allow(dead_code)]` 14 箇所は機械的置換が不可能であり、実装時に一律削除する（置換ではない）。

## 現状

以下のファイルに計 26 箇所の `#[allow(...)]` が存在する。

### クレートレベル (1 箇所) + モジュールレベル (4 箇所)

- `src/lib.rs:5` — `#![allow(clippy::too_many_arguments)]`（クレートルートの内部属性。クレート全体に適用される）
- `src/convert/` — 0036 で分割済み。`#![allow(clippy::too_many_arguments)]` は分割時に削除済み（対象外）
- `src/planar.rs:2` — 同上
- `src/rotate.rs:2` — 同上
- `src/scale.rs:2` — 同上

**注意**: クレートレベルの `#![expect(clippy::too_many_arguments)]` が lint を抑制した状態では、モジュールレベルの `#![expect(...)]` は lint を観測できず unfulfilled expectation になる。そのためクレートレベル 1 箇所だけを `#![expect(...)]` に置換し、モジュールレベル 4 箇所は**削除**する。

### sys.rs (7 箇所)

- `src/sys.rs:1-7` — `#![allow(non_upper_case_globals)]`, `#![allow(non_camel_case_types)]`, `#![allow(non_snake_case)]`, `#![allow(dead_code)]`, `#![allow(unused_imports)]`, `#![allow(unnecessary_transmutes)]`, `#![allow(clippy::all)]`

`sys.rs` は `include!` で build.rs が生成する `metadata.rs` と `bindings.rs` の両方を取り込んでいる。これらの allow は自動生成コード向けの抑制である。

### マクロ内 (14 箇所)

`src/lib.rs` の以下のマクロ定義内:

- `define_yuv_image` (L705, L747)
- `define_y_image` (L774, L809)
- `define_nv_image` (L845, L880)
- `define_packed_image` (L906, L943)
- `define_yuv_image16` (L985, L1027)
- `define_nv_image16` (L1058, L1093)
- `define_packed_image16` (L1119, L1156)

各行は `#[allow(dead_code)]` であり、マクロで生成される `pub(crate) fn validate(...)` に付与される。

**注意**: これらの `validate` メソッドは全画像型で実際に呼び出されており（コードベース内に `.validate()` 呼び出しが 500 箇所以上存在）、`dead_code` lint は発生しない。そのため `#[expect(dead_code)]` に機械的に置換すると「unfulfilled expectation」警告が発生し CI が失敗する。

## 設計方針

置換は対象ごとに 3 段階で行う。

### 1. クレートレベル + モジュールレベル (5 箇所)

`src/lib.rs:5` の `#![allow(clippy::too_many_arguments)]` を `#![expect(clippy::too_many_arguments)]` に置換する。クレート全体に適用されるため、各モジュールの多数引数関数で lint が実在し expect は充足される。

`src/planar.rs:2`, `src/rotate.rs:2`, `src/scale.rs:2` の 3 箇所はクレートレベルと冗長なため**削除**する（`#![expect(...)]` に置換すると unfulfilled expectation になる）。`src/convert.rs` は 0036 で `src/convert/` に分割され、`#![allow(clippy::too_many_arguments)]` は削除済み。

### 2. sys.rs (7 箇所)

`#![allow(...)]` → `#![expect(...)]` に置換する。自動生成コード（`metadata.rs` + `bindings.rs`）は該当 lint を実際にトリガーするため expect は充足される見込み。

**リスクとフォールバック**: bindgen のバージョン変更で生成コードの lint プロファイルが変わった場合、一部の expect が未充足になる可能性がある。これは `#[expect]` の目的（不要になった抑制の検出）に沿ったものであり許容する。ただし置換後に `cargo clippy` で unfulfilled expectation が発生した場合は、該当の属性を削除する（`#[allow]` に戻さない）。

### 3. マクロ内 (14 箇所)

`#[allow(dead_code)]` は **削除する**（置換ではない）。`validate` メソッドは全画像型で使用されており `dead_code` lint が発生しないため、属性自体が不要。属性を削除して実際のコンパイラ警告で不要なコードを検出できるようにする。

## 完了条件

- `src/lib.rs:5` が `#![expect(clippy::too_many_arguments)]` に置換されていること
- `src/planar.rs:2`, `src/rotate.rs:2`, `src/scale.rs:2` の `#![allow(clippy::too_many_arguments)]` が削除されていること（`src/convert.rs` は 0036 で分割済み・削除済み）
- `src/sys.rs` の 7 箇所が `#![expect(...)]` に置換されていること（unfulfilled expectation が発生した場合は該当属性を削除すること）
- マクロ内 14 箇所の `#[allow(dead_code)]` が削除されていること
- コードベース内のソースファイルに `#[allow(...)]` および `#![allow(...)]` が存在しないこと
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること

## 解決方法

1. クレートレベル: `src/lib.rs:5` の `#![allow(clippy::too_many_arguments)]` を `#![expect(clippy::too_many_arguments)]` に置換する
2. モジュールレベル: `src/planar.rs:2`, `src/rotate.rs:2`, `src/scale.rs:2` の `#![allow(clippy::too_many_arguments)]` を削除する（クレートレベルと冗長なため。`src/convert.rs` は 0036 で分割済み・削除済み）
3. sys.rs: `src/sys.rs:1-7` の 7 行の `#![allow(...)]` を `#![expect(...)]` に置換する（lint 名はそのまま）
4. マクロ定義: `src/lib.rs` の 7 つのマクロ定義内にある `#[allow(dead_code)]` 属性 14 箇所を削除する（行番号: L705, L747, L774, L809, L845, L880, L906, L943, L985, L1027, L1058, L1093, L1119, L1156）
5. `cargo clippy --all-targets --all-features -- -D warnings` を実行し、unfulfilled expectation が発生した場合は該当属性を削除して再実行する
6. `cargo test --workspace` が成功することを確認する
