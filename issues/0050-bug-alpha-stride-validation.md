# アルファプレーンの stride 検証（c_int 範囲・stride >= width）が欠落している

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-alpha-stride-validation
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/convert/i420.rs` の `validate_alpha_src` / `validate_alpha_dst` はバッファサイズのみを検証し、`src_stride_a` / `dst_stride_a` の `c_int` 範囲チェックと `stride >= width` チェックが欠落している。`as c_int` の切り詰めで巨大な stride が負値に化けて libyuv に渡り、領域外アクセスにつながる。検証を追加する。

## 優先度根拠

High。

- `c_int` 範囲を超える stride が負値に切り詰められ、libyuv は負の stride を「上下反転」として解釈するためスライス範囲外の読み書きにつながりうる
- `stride = 0` で `height > 1` の入力でも検証を通過し、C 側が各行 width バイトを読むため OOB 読み出しになる

## 現状

`validate_alpha_src` / `validate_alpha_dst`（`src/convert/i420.rs`）は:

```rust
let required = src_stride_a * (size.height - 1) + size.width;
if src_a.len() < required { ... }
```

のみで、以下が欠落している:

- `require_c_int(src_stride_a, ...)` / `require_c_int(dst_stride_a, ...)`（呼び出し側で `src_stride_a as c_int` のまま渡している）
- `src_stride_a >= size.width` / `dst_stride_a >= size.width` チェック

影響を受ける公開 API:

- `i420_alpha_to_argb` / `i420_alpha_to_abgr`
- `i422_alpha_to_argb` / `i422_alpha_to_abgr`
- `i444_alpha_to_argb` / `i444_alpha_to_abgr`
- `argb_to_i420_alpha`

なお `size.height == 0` の panic は issue 0026 で対応済み（pending）。本 issue は 0026 が「別 issue で対応する」と明記した stride 検証の残り部分を担当する。

## 設計方針

`validate_alpha_src` / `validate_alpha_dst` に、他の検証ヘルパー（`src/lib.rs` の `require_c_int` / `checked_buf_size`）と同じパターンで:

- `require_c_int(src_stride_a)` / `require_c_int(dst_stride_a)` を追加
- `stride >= width` チェックを追加
- サイズ計算を `checked_mul` / `checked_add` でオーバーフロー安全にする

## 完了条件

- `src_stride_a` / `dst_stride_a` が `c_int` 範囲を超える場合に `Err` を返すこと
- `src_stride_a < size.width` / `dst_stride_a < size.width` の場合に `Err` を返すこと
- 上記 7 関数の境界値テストが `tests/test_convert.rs` に追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `validate_alpha_src` / `validate_alpha_dst` に `require_c_int` と `stride >= width` チェックを追加する
2. サイズ計算を `checked_mul` / `checked_add` に変更する
3. `tests/test_convert.rs` にテストを追加する
4. `CHANGES.md` に `[FIX]` エントリを追加する
