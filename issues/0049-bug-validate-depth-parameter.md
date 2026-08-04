# planar の 16bit 変換関数群で depth パラメータが未検証のまま libyuv に渡される

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-validate-depth-parameter
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/planar.rs` の 16bit 変換関数群が `depth` パラメータ（8..=16 が有効）を検証せずに libyuv へ渡している。libyuv 側は `1 << depth` などのシフト演算を行うため、範囲外の `depth` で未定義動作（UB）を誘発する。Rust 側で `depth` の範囲検証を追加する。

## 優先度根拠

High。

- 範囲外の `depth`（32 以上、10 未満等）で libyuv のシフトが C 標準上 UB になる
- libyuv 側の `assert(depth >= 8); assert(depth <= 16);` は Release（NDEBUG）ビルドで消滅し、実効的なガードがない

## 現状

以下の関数（`src/planar.rs`）が `depth: i32` を無検証でネイティブに渡している:

- `merge_uv_plane_16`
- `split_uv_plane_16`
- `merge_ar64_plane`
- `merge_xr30_plane`
- `merge_argb16_to_8_plane`
- `convert_to_lsb_plane_16`
- `convert_to_msb_plane_16`

libyuv の実装（commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）:

- `ConvertToLSBPlane_16`（`planar_functions.cc`）: `int scale = 1 << depth;`（depth >= 32 で UB）
- `ConvertToMSBPlane_16`: `int scale = 1 << (16 - depth);`（depth > 16 で負シフト UB）
- `MergeUVRow_16_C` / `SplitUVRow_16_C`（`row_common.cc`）: `int shift = 16 - depth;`（depth > 16 で負シフト UB）
- `MergeAR64Row_C`: `int shift = 16 - depth; int max = (1 << depth) - 1;`
- `MergeXR30Row_C`: `int shift = depth - 10;`（depth < 10 で負シフト UB）

C 側の `assert(depth >= 8); assert(depth <= 16);` は `build.rs` の CMake Release プロファイル（NDEBUG）によりコンパイル時に除去される。

## 設計方針

7 関数すべてに `depth` の範囲検証（8..=16）を追加し、範囲外は `Error::with_reason(-1, function, "depth must be between 8 and 16")` 形式で `Err` を返す。docstring に `depth` の有効範囲を明記する。

## 完了条件

- 7 関数すべてが 8..=16 以外の `depth` で `Err` を返すこと（境界値: 7 / 8 / 16 / 17、および負値）
- 7 関数の docstring に `depth` の有効範囲（8..=16）が明記されていること
- `tests/test_planar.rs` に境界値テストが追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. 7 関数に `depth` の範囲検証（8..=16）を追加する
2. docstring に有効範囲を明記する
3. `tests/test_planar.rs` に境界値テストを追加する
4. `CHANGES.md` に `[FIX]` エントリを追加する
