# detile_plane / detile_plane_16 の入力検証をプロジェクト標準パターンに統一する

- Priority: Medium
- Created: 2026-07-29
- Completed: {YYYY-MM-DD}
- Model: qwen3.8-max-preview
- Branch: feature/fix-detile-plane-validation
- Polished: 2026-07-29
- Reporter: @voluntas

## 目的

`src/convert.rs` の `detile_plane` と `detile_plane_16` が `require_c_int` / `checked_buf_size` を使わない手動の簡易バッファチェックのみであり、プロジェクト標準の検証パターンに準拠していない問題を修正する。

## 優先度根拠

Medium。`src_stride * size.height` の乗算がオーバーフローチェックなしで行われており、巨大な stride / height でオーバーフローが発生するリスクがある。また `tile_height` の 2 の累乗チェックも欠落している。ただし `detile_split_uv_plane` と同様に呼び出し側はコードベース内に存在しないため、実害は限定的。

## 現状

`detile_plane` (src/convert.rs:3990-4026) と `detile_plane_16` (src/convert.rs:4029-4065) は以下の問題を持つ:

1. **`require_c_int` がない**: width, height, src_stride, dst_stride, tile_height の c_int 範囲チェックが一切ない
2. **`src_stride * size.height` がオーバーフロー未チェック**: `checked_mul` を使わず直接乗算している
3. **`tile_height` の 2 の累乗チェックがない**: libyuv 内部でビットマスクを使用するため 2 の累乗が必須
4. **stride >= width チェックがない**: src_stride / dst_stride が width 未満でも検証しない

同一セクションの `detile_split_uv_plane` は既に `require_c_int` / `checked_mul` / `tile_height.is_power_of_two()` / stride チェックを実装済み。

## 設計方針

`detile_split_uv_plane` と同じ検証パターンを適用する:

1. `require_c_int` で width, height, src_stride, dst_stride, tile_height を c_int 範囲チェック
2. `size.width == 0 || size.height == 0` で `Ok(())` を早期 return
3. `tile_height.is_power_of_two()` で 2 の累乗をチェック
4. `src_stride >= size.width`、`dst_stride >= size.width` の stride チェック
5. `checked_buf_size` でバッファサイズをオーバーフロー安全に計算

`detile_plane_16` は `&[u16]` を扱うため、バッファサイズ計算は要素数ベースで行う（バイト数ではない）。

## 完了条件

- `detile_plane` / `detile_plane_16` に `require_c_int` による c_int 範囲チェックが追加されていること
- `tile_height` の 2 の累乗チェックが追加されていること
- stride >= width チェックが追加されていること
- バッファサイズ計算が `checked_buf_size` でオーバーフロー安全に行われていること
- `tests/test_convert.rs` にテストが追加されていること
- `cargo fmt --all --check` / `cargo clippy --all-targets --all-features -- -D warnings` / `cargo test --workspace` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追記すること（@voluntas 署名付き）
