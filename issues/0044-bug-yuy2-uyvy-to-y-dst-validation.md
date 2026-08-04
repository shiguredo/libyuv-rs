# yuy2_to_y / uyvy_to_y のデスティネーション検証が不十分で領域外書き込みが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-yuy2-uyvy-to-y-dst-validation
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/convert/packed.rs` の `yuy2_to_y` / `uyvy_to_y` のデスティネーションバッファ検証が非チェック乗算 1 行のみで、`require_c_int` と `stride >= width` チェックが欠落している。libyuv が各行 width バイトを書き込むため、不正な stride / サイズで領域外書き込み（ヒープ破壊）に至る。標準検証パターンに統一する。

## 優先度根拠

High。

- stride >= width チェックがないため、通常サイズの入力（例: width=100 に dst_stride_y=10）でもデバッグビルド・リリースビルドを問わず領域外書き込みが発生する
- さらにリリースビルドでは `dst_stride_y * size.height` が wrap して検証をすり抜け、`dst_stride_y as c_int` の切り詰めにより巨大な stride が負値に化けて渡る

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

`src/convert` 内の標準パターン（`require_c_int` → `stride >= width` → `checked_buf_size` → `len` 比較）に統一する。参照実装は `src/convert/hardware.rs` の `detile_plane` / `detile_plane_16`（CHANGES.md の `[FIX] detile_plane と detile_plane_16 の入力検証をプロジェクト標準パターンに統一する` で標準化済み）と同一にする。

- `src.validate` が width / height の `require_c_int` 検査を既に実施しているため、dst 側では `require_c_int(dst_stride_y)` のみ追加する
- エラーメッセージは現行の語句（"destination Y buffer too small" 等）を維持する。新設するチェックの語句は参照実装 `detile_plane` / `detile_plane_16` に合わせる（"destination stride exceeds c_int range" / "destination stride smaller than width"。0060 のエラーメッセージ維持方針と整合）
- ゼロサイズ入力（width == 0 / height == 0）は C 実装が `-1` を返し `Err` になる現状の挙動を維持する（`detile_plane` のような no-op 早期 return は取り込まない。0064 のゼロサイズ統一方針とも整合。エラーメッセージの統一形式は 0064 側の作業とする）。ゼロサイズと不正 stride の複合入力では標準パターンの検証順序により `require_c_int` 等が先に `Err` を返すため、ゼロサイズテストは stride が正常な入力で行う

## 完了条件

- `dst_stride_y` が `c_int` 範囲を超える場合に `Err` を返すこと
- `dst_stride_y < size.width` の場合に `Err` を返すこと
- `dst_stride_y * size.height` がオーバーフローする場合に `Err` を返すこと（`require_c_int` により 64-bit では到達不能なためテスト対象外）
- ゼロサイズ入力（width == 0 / height == 0）で `Err` を返すこと
- `tests/test_convert.rs` に `yuy2_to_y` / `uyvy_to_y` の境界値テストが追加されていること（0052 で計画される convert 系テスト拡充との重複分は本 issue でカバーする）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `yuy2_to_y` / `uyvy_to_y` の dst 検証を標準パターン（`require_c_int` / `stride >= width` / `checked_buf_size`）に置き換える
2. `tests/test_convert.rs` に境界値テスト（stride 不足・c_int 超過・バッファ不足・ゼロサイズ・正常系）を追加する
3. `CHANGES.md` に `[FIX]` エントリを追加する
