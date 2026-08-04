# Mm21Image / Mt2tImage のバッファ検証がタイル配置・10bit パックを反映しておらず領域外読み出しが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-mm21-mt2t-tiled-validation
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`Mm21Image` / `Mt2tImage` が通常の NV12 と同じ線形検証（`define_nv_image!(2, 2)`）で検証される一方、libyuv の `MM21To*` / `MT2TToP010` はタイル配置・10bit パックで入力バッファを読み出す。MM21 は高さがタイル高の倍数でない場合、MT2T は高さ・幅の倍数に関わらずタイル行サイズ（10/8 倍パック）が線形サイズを超える場合に、検証済みサイズを超える読み出し（OOB）が発生する。タイル形式専用の検証を実装する。

## 優先度根拠

High。

- 検証を通過する入力で領域外読み出し（UB）が発生する
- `mt2t_to_p010` はタイル配置と 10/8 倍のパックサイズによる必要サイズの増大を検証に反映していない

## 現状

`src/lib.rs` の `Mm21Image` / `Mt2tImage` は `define_nv_image!(..., 2, 2)` で定義され、`validate` は線形 NV12 の要件（`y_stride >= width`、`uv_stride >= ceil(width / 2) * 2`、`len >= stride * ceil(height / 2)` 等）しか検査しない。

一方 libyuv の実装（commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）:

- `MM21ToNV12`（`convert.cc`）は `DetilePlane(src_y, ..., width, height, 32)` と `DetilePlane(src_uv, ..., (width + 1) & ~1, (height + sign) / 2, 16)` でタイル配置を読む。`MM21ToI420` は Y に `DetilePlane`、UV に `DetileSplitUVPlane`、`MM21ToYUY2` は `DetileToYUY2` を使う（読み出し規則は同一）。タイル配置では、1 タイル行の読み出し幅は幅を 16 の倍数に切り上げた値（padded width）× タイル高になり、タイル行間隔は stride × タイル高になる。線形サイズより大きくなりうる
- `MT2TToP010`（`convert.cc`）は `convert.h` のコメントにある通り stride がバイト単位で、タイル行ごとに `UnpackMT2T` が Y は `padded_width * 32 * 10 / 8` バイト、UV は `padded_width * 16 * 10 / 8` バイトを読む

`mm21_to_i420` / `mm21_to_nv12` / `mm21_to_yuy2` / `mt2t_to_p010`（`src/convert/hardware.rs`）はすべてこの不十分な `validate` を通して unsafe 呼び出しを行う。例: 1920x1080、stride=1920 の MT2T で検証は Y = 2,073,600 バイトで通過するが、C 側の最終読み出しは 2,104,320 バイト目に及ぶ。

## 設計方針

`Mm21Image` / `Mt2tImage` 用の専用検証を実装する（`define_nv_image` マクロの線形検証を流用しない）。

必要サイズは C 側の読み出しパターン（1 タイル行の読み出し幅 = 幅を 16 の倍数に切り上げた値 × タイル高、タイル行間隔 = stride × タイル高）から導出する。padded width は `width.div_ceil(16) * 16`、タイル行数は `height.div_ceil(32)` とする。UV のタイル行数は `(height + 1) / 2`（= ceil(height / 2)）のタイル高 16 分割で `height.div_ceil(32)` と等価（`ceil(ceil(h/2)/16) == ceil(h/32)`。`(height + 1).div_ceil(32)` ではない。height が 32 の倍数のとき 1 タイル行分過大になるため注意）:

- MM21 Y の必要サイズ: `(height.div_ceil(32) - 1) * y_stride * 32 + padded_width * 32`
- MM21 UV の必要サイズ: `(height.div_ceil(32) - 1) * uv_stride * 16 + padded_width * 16`（幅は `(width + 1) & !1` を 16 の倍数に切り上げる）
- MT2T Y の必要サイズ: `(height.div_ceil(32) - 1) * y_stride * 32 + padded_width * 32 * 10 / 8`
- MT2T UV の必要サイズ: `(height.div_ceil(32) - 1) * uv_stride * 16 + padded_width * 16 * 10 / 8`

注意点:
- 上記の式は安全側（過大要求）である。MM21 は実際には最終タイル列の余り分と最終タイル行の端数行分を読まないため、式は最大 480 バイト（32 行 × 15 バイト。行あたり 16 - (width % 16) バイトの過大）+ 端数行分過大。MT2T は部分タイル行でもフルサイズを読むため正確。検証式の根拠コメント（libyuv の `DetilePlane` / `DetileSplitUVPlane` / `DetileToYUY2` / `MT2TToP010` の行送り規則への参照）をコメントで明記し、MM21 側が保守的である旨も併記する
- height == 0 では `height.div_ceil(32) - 1` がアンダーフローするため、ゼロサイズは式の前に `Err` を返す（0064 のゼロサイズ統一方針と整合）
- 現行の線形検証が持つ stride 下限チェック（`y_stride >= width`、`uv_stride >= ceil(width / 2) * 2`）は専用検証でも維持する
- MT2T は stride がバイト単位であることを docstring に明記する

## 完了条件

- 高さがタイル高（Y: 32、UV: 16）の倍数でない入力、および幅が 16 の倍数でない入力で、必要サイズを下回るバッファが `Err` になること
- 必要サイズちょうどのバッファで `Ok` になり、1 バイト不足で `Err` になる境界が成立すること（例: 1920x1080、stride=1920 の MT2T で Y 必要サイズ 2,104,320 バイト）
- MT2T の 10bit パック（10/8 倍）を反映した必要サイズを下回るバッファが `Err` になること
- 検証式の根拠コメント（libyuv の行送り規則への参照）が付いていること
- `Mt2tImage` の docstring に stride がバイト単位であることが明記されていること
- ゼロサイズ入力（width == 0 / height == 0）で `Err` を返すこと
- `tests/test_convert.rs` に MM21 / MT2T の境界値テスト（タイル高倍数・非倍数 × 幅 16 倍数・非倍数）が追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `Mm21Image` / `Mt2tImage` 用の検証（タイル配置・10bit パック）を `src/lib.rs` に実装する
2. `Mm21Image` / `Mt2tImage` の定義を専用検証に切り替える（呼び出し側 `mm21_to_*` / `mt2t_to_p010` は `src.validate` のみで変更不要。`Mm21ImageMut` / `Mt2tImageMut` は MM21/MT2T を出力する変換関数が存在せず未使用のため、dst 検証は現行の線形検証のままとする）
3. `tests/test_convert.rs` にテストを追加する
4. `CHANGES.md` に `[FIX]` エントリを追加する
