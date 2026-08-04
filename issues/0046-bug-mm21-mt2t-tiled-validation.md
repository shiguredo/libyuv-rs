# Mm21Image / Mt2tImage のバッファ検証がタイル配置・10bit パックを反映しておらず領域外読み出しが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-mm21-mt2t-tiled-validation
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`Mm21Image` / `Mt2tImage` が通常の NV12 と同じ線形検証（`define_nv_image!(2, 2)`）で検証される一方、libyuv の `MM21To*` / `MT2TToP010` はタイル配置・10bit パックで入力バッファを読み出す。高さがタイル高の倍数でない場合に検証済みサイズを超える読み出し（OOB）が発生する。タイル形式専用の検証を実装する。

## 優先度根拠

High。

- 検証を通過する入力で領域外読み出し（UB）が発生する
- `mt2t_to_p010` は stride がバイト単位・Y プレーンが 10/8 倍サイズという仕様を完全に無視している

## 現状

`src/lib.rs` の `Mm21Image` / `Mt2tImage` は `define_nv_image!(..., 2, 2)` で定義され、`validate` は線形 NV12 の要件（`y_stride >= width`、`uv_stride >= ceil(width / 2) * 2`、`len >= stride * ceil(height / 2)` 等）しか検査しない。

一方 libyuv の実装（commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）:

- `MM21ToNV12` / `MM21ToI420` / `MM21ToYUY2`（`convert.cc`）は `DetilePlane(src_y, ..., width, height, 32)` と `DetilePlane(src_uv, ..., (width + 1) & ~1, (height + sign) / 2, 16)` でタイル配置を読む。タイル配置の必要サイズは `div_ceil(height, tile_height) * stride * tile_height` であり、線形サイズより大きくなりうる
- `MT2TToP010`（`convert.cc`）は `convert.h` のコメントにある通り stride がバイト単位で、Y はタイル行ごとに `padded_width * 32 * 10 / 8` バイト、UV は `padded_width * 16 * 10 / 8` バイトを読む

`mm21_to_i420` / `mm21_to_nv12` / `mm21_to_yuy2` / `mt2t_to_p010`（`src/convert/hardware.rs`）はすべてこの不十分な `validate` を通して unsafe 呼び出しを行う。例: 1920x1080、stride=1920 の MT2T で検証は Y = 約 2,073,600 バイトで通過するが、C 側は 2,104,320 バイト先まで読む。

## 設計方針

`Mm21Image` / `Mt2tImage` 用の専用検証を実装する（`define_nv_image` マクロの線形検証を流用しない）。

- MM21: Y は `div_ceil(height, 32) * y_stride * 32`、UV は `div_ceil(ceil(height / 2), 16) * uv_stride * 16` を必要サイズとして検証する
- MT2T: stride がバイト単位であることを docstring に明記し、Y / UV のタイル行サイズ（`padded_width * tile_height * 10 / 8`）ベースで検証する
- 検証式の根拠（libyuv の `DetilePlane` / `MT2TToP010` の行送り規則）をコメントで明記する
- `div_ceil` で幅を 16 の倍数に丸める（タイル幅 16）ことも考慮する

## 完了条件

- 高さがタイル高（Y: 32、UV: 16）の倍数でない入力で、必要サイズを下回るバッファが `Err` になること
- MT2T の 10bit パックサイズ（10/8 倍）を下回るバッファが `Err` になること
- 検証式の根拠コメント（libyuv の行送り規則への参照）が付いていること
- `tests/test_convert.rs` に MM21 / MT2T の境界値テストが追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `Mm21Image` / `Mt2tImage` 用の検証（タイル配置・10bit パック）を `src/lib.rs` に実装する
2. `Mm21Image` / `Mt2tImage` の定義を専用検証に切り替える
3. 必要に応じて `mm21_to_*` / `mt2t_to_p010` の呼び出し側を修正する
4. `tests/test_convert.rs` にテストを追加する
5. `CHANGES.md` に `[FIX]` エントリを追加する
