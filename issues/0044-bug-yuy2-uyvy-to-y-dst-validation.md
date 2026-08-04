# yuy2_to_y / uyvy_to_y のデスティネーション検証が不十分で領域外書き込みが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-yuy2-uyvy-to-y-dst-validation
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/convert/packed.rs` の `yuy2_to_y` / `uyvy_to_y` のデスティネーションバッファ検証が非チェック乗算 1 行のみで、`require_c_int` と `stride >= width` チェックが欠落している。libyuv が各行 width バイトを書き込むため、不正な stride / サイズで領域外書き込み（ヒープ破壊）に至る。標準検証パターンに統一する。

## 優先度根拠

High。

- リリースビルドで `dst_stride_y * size.height` が wrap して検証をすり抜け、OOB 書き込みが発生する
- `dst_stride_y as c_int` の切り詰めにより、巨大な stride が負値に化けて渡る

## 現状

`src/convert/packed.rs` の `yuy2_to_y` / `uyvy_to_y` の dst 検証は:

```rust
if dst_y.len() < dst_stride_y * size.height { ... }
```

のみである。以下のすべてが欠落している:

- `require_c_int(dst_stride_y, ...)`（`dst_stride_y as c_int` の切り詰め）
- `dst_stride_y >= size.width` チェック
- `checked_buf_size` によるオーバーフロー安全な乗算

libyuv の `YUY2ToY` / `UYVYToY`（`planar_functions.cc`）は各行 `width` バイトを書き込むため、`width=100, dst_stride_y=10, height=10, dst_y.len()=100` のような入力（現状の検証は通過）でバッファ外への書き込みが起きる。

## 設計方針

`src/convert` 内の標準パターン（`require_c_int` → `stride >= width` → `checked_buf_size` → `len` 比較）に統一する。他関数（例: `i420_to_y` 相当の関数群）と同一の実装にする。

## 完了条件

- `dst_stride_y` が `c_int` 範囲を超える場合に `Err` を返すこと
- `dst_stride_y < size.width` の場合に `Err` を返すこと
- `dst_stride_y * size.height` がオーバーフローする場合に `Err` を返すこと
- `tests/test_convert.rs` に `yuy2_to_y` / `uyvy_to_y` の境界値テストが追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `yuy2_to_y` / `uyvy_to_y` の dst 検証を標準パターン（`require_c_int` / `stride >= width` / `checked_buf_size`）に置き換える
2. `tests/test_convert.rs` に境界値テスト（stride 不足・c_int 超過・バッファ不足・正常系）を追加する
3. `CHANGES.md` に `[FIX]` エントリを追加する
