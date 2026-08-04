# detile_plane / detile_plane_16 / detile_to_yuy2 のソースバッファ検証が線形サイズのままで領域外読み出しが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-detile-tiled-size-validation
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/convert/hardware.rs` の `detile_plane` / `detile_plane_16` / `detile_to_yuy2` のソースバッファ検証が線形サイズ（`stride * height`）のままで、libyuv がタイル配置で読み出す実アクセス範囲を反映していない。高さがタイル高の倍数でない場合に検証済みサイズを超える読み出し（OOB）が発生する。`detile_split_uv_plane` と同じタイル配置のサイズ計算に統一する。

## 優先度根拠

High。

- 検証を通過する入力で領域外読み出し（UB）が発生する
- 同一ファイルの `detile_split_uv_plane` は正しいタイル公式（`stride * ceil(height / tile_height) * tile_height`）を使う一方で、3 関数だけが線形サイズのままと不整合

## 現状

libyuv の `DetilePlane`（`planar_functions.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は行ごとに src を 16 バイト進め、`tile_height` 行ごとに `src_y - src_tile_stride + src_stride_y * tile_height`（`src_tile_stride = 16 * tile_height`）で次のタイル行へジャンプする。タイル配置の必要サイズは `div_ceil(height, tile_height) * src_stride * tile_height` であり、線形サイズ（`src_stride * height`）より大きくなりうる。例: width=32、height=17、tile_height=16、stride=32 で実読み出しの最大オフセットは 784 バイトなのに対し、線形検証は 544 バイトしか要求しない。

対象と問題:

- `detile_plane`（`src/convert/hardware.rs`）: ソース検証が `checked_buf_size(src_stride, size.height)` の線形サイズ
- `detile_plane_16`: 同上（u16 要素数ベースの線形サイズ）
- `detile_to_yuy2`: `Nv12Image::validate`（線形）のみ。さらに `tile_height` の 2 累乗検証・`require_c_int(tile_height)` が欠落しており、libyuv は `y & (tile_height - 1)` でマスクするため非 2 累乗の `tile_height` で壊れた挙動になる

## 設計方針

`detile_split_uv_plane` の式（`src_stride * div_ceil(height, tile_height) * tile_height`）に統一する。

- `detile_plane` / `detile_plane_16`: ソースサイズをタイル配置で計算し検証する
- `detile_to_yuy2`: `tile_height` の 2 累乗検証と `require_c_int` を追加し、ソース（Y / UV）もタイル配置サイズで検証する
- 検証式の根拠（libyuv の行送り規則）をコメントで明記する

## 完了条件

- 高さが `tile_height` の倍数でない入力で、必要サイズを下回るソースバッファが `Err` になること
- `detile_to_yuy2` が非 2 累乗の `tile_height` で `Err` を返すこと
- `detile_to_yuy2` が `c_int` 範囲を超える `tile_height` で `Err` を返すこと
- `tests/test_convert.rs` に 3 関数の境界値テストが追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `detile_plane` / `detile_plane_16` のソースサイズ検証をタイル配置（`div_ceil`）ベースに変更する
2. `detile_to_yuy2` に `tile_height` の 2 累乗検証と `require_c_int` を追加し、ソースサイズ検証をタイル配置ベースに変更する
3. 検証式の根拠コメントを追加する
4. `tests/test_convert.rs` にテストを追加する
5. `CHANGES.md` に `[FIX]` エントリを追加する
