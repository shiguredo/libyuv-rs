# android420_to_* の pixel_stride_uv == 2 で U/V プレーンの検証が不足し領域外読み出しが発生する

- Priority: High
- Created: 2026-08-04
- Completed: 2026-08-06
- Model: DeepSeek V4 Flash
- Branch: feature/fix-android420-pixel-stride-uv-validation
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`android420_to_argb` / `android420_to_abgr` / `android420_to_i420`（`src/convert/hardware.rs`）と `android420_to_i420_rotate`（`src/rotate.rs`）は、`pixel_stride_uv == 2`（UV インターリーブ）のときに libyuv が U/V プレーンを 1 行 `2 * halfwidth` バイト（インターリーブ。奇数幅では width + 1 バイト）で読み進めるにもかかわらず、U/V のストライド検証が `ceil(width / 2)` のままである。検証を通過する入力で領域外読み出し（UB）が発生する。

## 優先度根拠

High。

- 検証を通過する入力で OOB 読み出しが発生する
- `pixel_stride_uv` の値（1 / 2）の検証が rotate 版にしかなく、hardware.rs の 3 関数は任意の値を無検証で渡す

## 現状

libyuv の `Android420ToARGBMatrix`（`convert_argb.cc`）と `Android420ToI420Rotate`（`rotate.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は `pixel_stride_uv == 2` のとき、U/V をインターリーブデータとして 1 行 `2 * halfwidth` バイト読み進める（奇数幅では width + 1 バイト。高速パス `NV12/NV21ToARGBMatrix` の行関数と `SplitRotateUV` が該当）。また C 側は `src_v - src_u`（`vu_off`）のポインタ減算を行い、`vu_off` が ±1 かつ `src_stride_u == src_stride_v` のときだけ高速パスに入る。フォールバックは `WeavePixels` / `SplitPixels` による `pixel_stride_uv` 刻みの読み出しになる（`Android420ToI420Rotate` のフォールバックは `rotation == 0` のときのみ。それ以外の回転では C 側が `-1` を返す）。

一方 Rust 側の検証（`src/lib.rs` の `validate_yuv_src_inner`）は `u_stride >= ceil(width / 2)` と `u.len() >= u_stride * ceil(height / 2)` を要求するだけなので:

- `pixel_stride_uv == 2` で u/v をインターリーブとして渡す場合、行幅が `2 * halfwidth` バイト必要だが検証は半分で通過する → OOB 読み出し
- `pixel_stride_uv` に 1 / 2 以外の値（巨大値等）を渡すと、`as c_int` の切り詰めとフォールバック経路で広範囲の読み出しになる
- Rust API は u / v を別スライスで受け取るため、C 側の `src_v - src_u`（別オブジェクト間のポインタ減算）自体が C 標準では UB になる。この前提条件（u/v が同一バッファの分割）が docstring に一切記載されていない

## 設計方針

- `pixel_stride_uv` の値検証（1 または 2 以外は `Err`）を 4 関数すべてに追加する（hardware.rs の 3 関数に追加、rotate.rs は既存の検証を維持。値が 1 / 2 に限定されるため `require_c_int(pixel_stride_uv)` は不要）
- `pixel_stride_uv == 2` のときは U/V の実効行幅を `2 * ceil(width / 2)` バイトとして検証する（`u_stride >= width.div_ceil(2) * 2`、`u.len() >= u_stride * ceil(height / 2)` 相当。`validate_nv_src_inner` のインターリーブ最小幅と同じ式）
- 検証の実装場所は 4 関数の関数内（`require_c_int` / `checked_buf_size` パターン）とする。`validate_yuv_src_inner` は全 YUV 画像型共通のため変更しない（共通ヘルパー化が必要なら将来のヘルパー整理の課題とする）
- docstring に「`pixel_stride_uv == 2` のとき、u / v は同一バッファの連続領域（インターリーブ）でなければならない。別スライスを渡すと C 側のポインタ減算（`src_v - src_u`）が未定義動作になる」前提を明記する（C 側は `pixel_stride_uv` の値によらず減算式を評価するため、`pixel_stride_uv == 1` のプラナーでも別スライスは形式上 UB になる。ただし結果は使用されないため実害はない旨を併記する）。また、検証は安全側に `len() >= stride * ceil(height / 2)` を要求するため、同一バッファの連続領域（u は先頭から、v は 1 バイトずらして末尾まで）で渡す場合、v 側のバッファ長が要求を満たすよう末尾にパディングを確保することも明記する

## 完了条件

- 4 関数すべてが `pixel_stride_uv` の 1 / 2 以外を `Err` にすること
- `pixel_stride_uv == 2` で U/V ストライド不足・バッファ不足の入力が `Err` になること（奇数幅: u / v とも `stride == width` は `Err`、`stride == width.div_ceil(2) * 2` は `Ok` になる境界を含む）
- 4 関数の docstring に同一バッファ前提（`pixel_stride_uv == 2` 限定）とパディング要件が明記されていること
- 検証式（最小行幅 `2 * ceil(width / 2)`）の根拠コメント（libyuv の高速パス・フォールバックの行読み出し規則への参照）が付いていること
- `tests/test_convert.rs` / `tests/test_rotate.rs` に境界値テスト（偶数幅・奇数幅 × 奇数高 × `pixel_stride_uv` 1 / 2 / その他の値）が追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

実装済み（2026-08-06）。以下のとおり対応した。

- `src/convert/hardware.rs` の `android420_to_argb` / `android420_to_abgr` / `android420_to_i420` と `src/rotate.rs` の `android420_to_i420_rotate` に `pixel_stride_uv` の値検証（1 / 2 以外は Err）を追加する
- `pixel_stride_uv == 2` のとき、U/V のストライドをインターリーブ前提（最小行幅 `2 * ceil(width / 2)`）で検証する。バッファ長は `src.validate` が同一式で検査済みのため関数内では検証しない（1 周目に追加したデッドコードのバッファ検証はレビューで判明し削除した）
- docstring に前提条件を明記する: `pixel_stride_uv == 2` は同一バッファの連続領域必須（C 側のポインタ減算 `src_v - src_u` のため）、共有バッファ末尾に 1 バイトのパディング確保（NV12 では v 側、NV21 では u 側のバッファ長が要求を満たす）、ストライド不一致時の回転制約（0 度のみフォールバック有効）
- `tests/test_convert.rs` / `tests/test_rotate.rs` にテスト 13 件を追加する: 値検証（4 関数）、インターリーブストライド境界（奇数幅: halfwidth / width / 2*halfwidth、偶数幅）、V 側ストライド不足、NV12 変換との一致（正常系）、ストライド不一致のフォールバック（正常系）
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加する
