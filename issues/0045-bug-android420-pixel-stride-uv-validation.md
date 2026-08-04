# android420_to_* の pixel_stride_uv == 2 で U/V プレーンの検証が不足し領域外読み出しが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-android420-pixel-stride-uv-validation
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`android420_to_argb` / `android420_to_abgr` / `android420_to_i420`（`src/convert/hardware.rs`）と `android420_to_i420_rotate`（`src/rotate.rs`）は、`pixel_stride_uv == 2`（UV インターリーブ）のときに libyuv が U/V プレーンを 1 行 width バイト（インターリーブ）で読み進めるにもかかわらず、U/V のストライド検証が `ceil(width / 2)` のままである。検証を通過する入力で領域外読み出し（UB）が発生する。

## 優先度根拠

High。

- 検証を通過する入力で OOB 読み出しが発生する
- `pixel_stride_uv` の値（1 / 2）の検証が rotate 版にしかなく、hardware.rs の 3 関数は任意の値を無検証で渡す

## 現状

libyuv の `Android420ToARGBMatrix`（`convert_argb.cc`）と `Android420ToI420Rotate`（`rotate.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は `pixel_stride_uv == 2` のとき、U/V をインターリーブデータとして 1 行 `2 * halfwidth` バイト読み進める。また C 側は `src_v - src_u`（`vu_off`）のポインタ減算を行い、`vu_off` が ±1 かつ `src_stride_u == src_stride_v` のときだけ高速パスに入る。それ以外はフォールバックで `pixel_stride_uv` 刻みの読み出しになる。

一方 Rust 側の検証（`src/lib.rs` の `validate_yuv_src_inner`）は `u_stride >= ceil(width / 2)` と `u.len() >= u_stride * ceil(height / 2)` を要求するだけなので:

- `pixel_stride_uv == 2` で u/v をインターリーブとして渡す場合、行幅が width バイト必要だが検証は半分で通過する → OOB 読み出し
- `pixel_stride_uv` に 1 / 2 以外の値（巨大値等）を渡すと、`as c_int` の切り詰めとフォールバック経路で広範囲の読み出しになる
- Rust API は u / v を別スライスで受け取るため、C 側の `src_v - src_u`（別オブジェクト間のポインタ減算）自体が C 標準では UB になる。この前提条件（u/v が同一バッファの分割）が docstring に一切記載されていない

## 設計方針

- `pixel_stride_uv` の値検証（1 または 2）と `require_c_int` を 4 関数すべてに追加する
- `pixel_stride_uv == 2` のときは U/V の実効行幅を `width` バイトとして検証する（`u_stride >= width`、`u.len() >= u_stride * ceil(height / 2)` 相当）
- docstring に「u / v は同一バッファの連続領域（インターリーブ）でなければならない」前提を明記する
- 別スライスを渡すことの危険性（C 側のポインタ減算 UB）を docstring で警告する

## 完了条件

- 4 関数すべてが `pixel_stride_uv` の 1 / 2 以外を `Err` にすること
- `pixel_stride_uv == 2` で U/V ストライド不足・バッファ不足の入力が `Err` になること
- 4 関数の docstring に同一バッファ前提と `pixel_stride_uv` の意味が明記されていること
- `tests/test_convert.rs` / `tests/test_rotate.rs` に境界値テストが追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `pixel_stride_uv` の値検証（1 / 2）と `require_c_int` を 4 関数に追加する
2. `pixel_stride_uv == 2` の場合の U/V ストライド・バッファサイズ検証をインターリーブ前提に強化する
3. 4 関数の docstring に前提条件を明記する
4. `tests/test_convert.rs` / `tests/test_rotate.rs` にテストを追加する
5. `CHANGES.md` に `[FIX]` エントリを追加する
