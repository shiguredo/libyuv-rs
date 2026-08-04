# アルファプレーンの stride 検証（c_int 範囲・stride >= width）が欠落している

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-alpha-stride-validation
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/convert/i420.rs` の `validate_alpha_src` / `validate_alpha_dst` はバッファサイズのみを検証し、`src_stride_a` / `dst_stride_a` の `c_int` 範囲チェックと `stride >= width` チェックが欠落している。`as c_int` の切り詰めで巨大な stride が負値に化けて libyuv に渡り、領域外アクセスにつながる。検証を追加する。

## 優先度根拠

High。

- `c_int` 範囲を超える stride が負値に切り詰められ、C 側の行ポインタ加算（読み出し系は `src_a += src_stride_a`、書き込み系は `ARGBExtractAlpha` が `dst_a` に書き込み。ARGBToI420Alpha の USE_EXTRACTALPHA 版は 2 行単位で進む）でアルファプレーンのみ逆進する。読み出し系（`i420_alpha_to_argb` 等 6 関数）はスライス先頭より手前の領域外読み出し、書き込み系（`argb_to_i420_alpha`）は領域外書き込み（メモリ破壊）になる（負の height による反転機構とは別物。反転処理はアルファプレーンではなく画像プレーン側にのみ適用される: I420AlphaToARGB 系では dst、ARGBToI420Alpha では src）
- リリースビルドで `src_stride_a * (size.height - 1) + size.width` が wrap して検証をすり抜けると OOB アクセスになる（`require_c_int(stride)` と画像側 `validate` が保証する `size.height <= c_int::MAX` の両方が揃えば 64-bit では積が usize を超えない）

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

なお `size.height == 0` の panic とサイズ計算の `checked_mul` / `checked_add` 化は issue 0026（pending、未実装）の担当範囲である。本 issue は 0026 が「別 issue で対応する」と明記した stride 検証（`c_int` 範囲チェックと `stride >= width` チェック）の残り部分を担当する。0026 を先に完了させるか、本 issue を先に実装する場合は 0026 完了後に checked 化が行われる前提とする。なお `size.height == 0` は 0026 未実装の間は panic するため、本 issue のテストには含めない。

## 設計方針

`validate_alpha_src` / `validate_alpha_dst` に、他の検証ヘルパー（`src/lib.rs` の `require_c_int`、`validate_yuv_src_inner` の `stride >= width` チェック）と同じパターンで:

- `require_c_int(src_stride_a)` / `require_c_int(dst_stride_a)` を追加（サイズ計算より前に置く）
- `stride >= width` チェックを追加（サイズ計算より前に置く。バッファサイズ計算が先だと、stride 不足の入力が「バッファ不足」の `Err` で先に弾かれ、stride 不足検証がテストで意図した経路を検証できない）（`validate_yuv_src_inner` の `y_stride < size.width` と同じ仕様。C は stride < width でも行が重なるだけで OOB にはならないため、このチェックは安全性より既存検証パターンとの統一を目的とする）
- サイズ計算の `checked_mul` / `checked_add` 化は 0026 の担当（本 issue では行わない）

## 完了条件

- `src_stride_a` / `dst_stride_a` が `c_int` 範囲を超える場合に `Err` を返すこと（境界値: `c_int::MAX` で `Ok`、`c_int::MAX + 1` で `Err`）
- `src_stride_a < size.width` / `dst_stride_a < size.width` の場合に `Err` を返すこと（境界値: `width - 1` で `Err`、`width` で `Ok`）
- 上記 7 関数の境界値テストが `tests/test_convert.rs` に追加されていること（stride 境界のみ。`size.height == 0` のケースは 0026 のスコープのため含めない。画像側 `validate` が先に落ちないよう src/dst 画像の stride・バッファを正しく設定すること。`c_int::MAX` の `Ok` テストは height=1 で行う（required = width になる）。C 側 SIMD のアラインメント切り上げ読み出しに備え、アルファバッファは必要サイズより余裕を持って確保すること。正常系・バッファ不足の代表カバーは 0052 のスコープ）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `validate_alpha_src` / `validate_alpha_dst` に `require_c_int` と `stride >= width` チェックを追加する
2. `tests/test_convert.rs` にテストを追加する
3. `CHANGES.md` に `[FIX]` エントリを追加する
