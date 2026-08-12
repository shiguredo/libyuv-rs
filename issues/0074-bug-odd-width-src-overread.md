# 奇数幅の変換で libyuv の SIMD 余り処理がバッファ行終端を読み書き越す

- Priority: Medium
- Created: 2026-08-06
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-odd-width-src-overread
- Polished: 2026-08-12
- Reporter: @voluntas

## 目的

YUY2 / UYVY 系の変換関数で、width が奇数のとき libyuv の行関数ラッパー（`ANY11` / `ANY31` 等の余り処理）が 1 行あたり `width * 2 + 2` バイト読み書きするため、最終行でバッファの末尾を 2 バイト超える。Rust 側の検証（`len >= stride * height`）を通過する入力で C 側の領域外アクセス（未定義動作）が発生しうる。ソース / デスティネーションの検証を強化する。

対象関数（`src/convert/packed.rs`）:

- ソース読み越し: `yuy2_to_y` / `uyvy_to_y`
- デスティネーション書き込み越え: `i420_to_yuy2` / `i420_to_uyvy` / `i422_to_yuy2` / `i422_to_uyvy`

## 優先度根拠

Medium（ユーザー確認済み: 検証強化で対応）。

- 読み越しは未定義動作だが、libyuv の伝統的な仕様（行関数の余り処理がバッファ末尾の数バイトを読むことを許容）に基づく既知の挙動。実際のメモリレイアウトでは隣接領域を読むだけでクラッシュは稀だが、ASAN 等では検出される
- 書き込み越え（`i420_to_yuy2` / `i420_to_uyvy` / `i422_to_yuy2` / `i422_to_uyvy`）は読み越しより深刻（隣接領域の上書きの可能性）だが、実際のメモリレイアウトではクラッシュは稀

## 現状

libyuv の行関数ラッパーの余り処理は、SIMD のチャンク単位で読み書きする:

- `ANY11`（`row_any.cc`、`SS(width, shift) = (width + (1 << shift) - 1) >> shift`）: 余り処理で `memcpy(vin, src_ptr + (n >> UVSHIFT) * SBPP, SS(r, UVSHIFT) * SBPP)` により読み出す。`YUY2ToYRow_Any_SSE2`（`ANY11(..., 1, 4, 1, 15)`）等の YUY2 系は width が奇数のとき読み量が行の残り（`width * 2` バイト）より 2 バイト多い。`UYVYToYRow_Any_SSE2` / `_NEON` / `_LSX` / `_LASX` も同様だが、`UYVYToYRow_Any_AVX2`（`ANY11(..., 0, 2, 1, 31)`）は読み量が行の残りちょうどで読み越えない
- `ANY31`（`row_any.cc`）: 余り処理でデスティネーションに `memcpy(dst_ptr + (n >> DUVSHIFT) * BPP, vout, SS(r, DUVSHIFT) * BPP)` により書き込む。`I422ToYUY2Row_Any_*` / `I422ToUYVYRow_Any_*`（`ANY31(..., 1, 1, 4, 15)` 等）は width が奇数のとき書き込み量が行の残りより 2 バイト多い
- libyuv の変換関数は SIMD 有効時に Any ラッパーを使用するため、実環境（SIMD 対応 CPU）でこの読み書き越えが発生する。`YUY2ToY` / `UYVYToY`（`planar_functions.cc`）は行を合体（Coalesce）する: `src_stride == width * 2 && dst_stride == width && width * height <= INT_MAX` のとき 1 行に合体して `width * height` の 1 行として処理するため、合体時は `width * height` が奇数のときのみ読み越す。`I420ToYUY2` / `I420ToUYVY`（`convert_from.cc`）は Coalesce を持たず 2 行単位ループで、`I422ToYUY2` / `I422ToUYVY`（`convert_from.cc`）は Coalesce 条件（`src_stride_u * 2 == width` 等）が width 奇数では成立しないため、どちらも width が奇数のとき常に行単位の書き込み越えが発生する
- 同じ構造の書き込み越えは、`ARGBToYUY2` / `ARGBToUYVY`（`convert_from_argb.cc`）がデスティネーション書き込みに `I422ToYUY2Row_Any_*` / `I422ToUYVYRow_Any_*` を使うため、`argb_to_yuy2` / `argb_to_uyvy` にも存在する（完了条件の調査項目で適用範囲を確定する）
- Rust 側の検証は、`src/convert/packed.rs` の各関数が `validate`（`require_c_int` / stride 下限 / `checked_buf_size` による `len >= stride * height`）でバッファサイズを検査するが、この読み書き越えを防げない
- 例（Coalesce 非発動条件: dst_stride_y = 6 など `dst_stride_y != width`）: width = 5, src_stride = 10, src.len() = 20, height = 2 のとき、`yuy2_to_y` は `YUY2ToYRow_Any_SSE2` が最終行 `src[10..22]` を読む（バッファ 20 バイトを 2 バイト超過）。Coalesce 発動条件（`dst_stride_y == width`）では 1 行に合体され `width * height = 10`（偶数）のため読み越えない
- 偶数幅では読み書き越えは発生しない（余り処理のチャンク数が行幅に収まる）

## 設計方針

検証強化で対応する（ユーザー確認済み）。width が奇数のとき、最終行の読み書き量は `width * 2 + 2` バイトになるため、ソース / デスティネーションの必要サイズを引き上げる:

- `i420_to_yuy2` / `i420_to_uyvy` / `i422_to_yuy2` / `i422_to_uyvy`: デスティネーション必要サイズ = `dst_stride * (height - 1) + width * 2 + 2`（width が奇数のとき）。これらの関数は Coalesce が発動しない（または width 奇数で成立しない）ため、この式は正確
- `yuy2_to_y` / `uyvy_to_y`: ソース必要サイズは Coalesce を考慮して判定する。Coalesce 発動条件（`src_stride == width * 2 && dst_stride == width && width * height <= INT_MAX`。C 側の実条件と同じ）を満たすときは合体後の `width * height` が奇数の場合のみ `+2` を要求する（必要サイズ = `src_stride * height + 2`）、満たさないときは width が奇数の場合に `+2` を要求する（必要サイズ = `src_stride * (height - 1) + width * 2 + 2`。Coalesce 発動 + `width * height` 偶数では読み越えが起きないため、`+2` を要求しない。`width * height > INT_MAX` のときは Coalesce が発動しないため、width 奇数なら `+2` を要求する）
- 偶数幅では従来どおり `stride * height`（`+2` は不要）
- `uyvy_to_y` の `UYVYToYRow_Any_AVX2` は読み越えないが、検証はプラットフォーム非依存（CPU ディスパッチを考慮しない）で最悪ケース（SSE2 / NEON 等）を想定して `+2` を要求する（AVX2 環境では過大要求になるが安全側）

検証は関数ごとのインライン検証として追加する（共通の `validate` を変更すると YUY2 / UYVY を入出力する全関数に影響し、読み書き越えの有無・量が関数ごとに異なるため、関数ごとの検証で正確に扱う）。サイズ計算は `checked_mul` / `checked_add` でオーバーフロー安全に行い、`height - 1` は `checked_sub` で行う。`checked_sub` が None を返す場合（height == 0）はインライン検証をスキップして現行どおり C 経由の Err に委ねる（0064 のゼロサイズ統一方針と整合）。エラーメッセージは既存の `"source buffer too small"` / `"destination buffer too small"` を流用する。

`+2` の根拠は libyuv の行関数ラッパーの実装（`row_any.cc` の `ANY11` / `ANY31`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）であり、libyuv 更新時に見直すべき箇所としてコメントに明記する。

## 完了条件

- `yuy2_to_y` / `uyvy_to_y` が、Coalesce 非発動条件（例: `dst_stride_y != width`）で width 奇数のとき、`src_stride * height` ちょうどのソースバッファを `Err` にし、`src_stride * (height - 1) + width * 2 + 2` 以上で `Ok` を返すこと（例: width = 5, height = 2, src_stride = 10, dst_stride_y = 6 で 20 バイトは Err、22 バイトは Ok。`stride * height` ちょうどが `Err` になるのは `src_stride < width * 2 + 2` のとき（例: src_stride == width * 2 または width * 2 + 1））
- `yuy2_to_y` / `uyvy_to_y` が、Coalesce 発動条件（`src_stride == width * 2 && dst_stride == width && width * height <= INT_MAX`）で `width * height` が奇数のとき `+2` を要求し（必要サイズ = `src_stride * height + 2`）、`width * height` が偶数のとき従来どおり `src_stride * height` で `Ok` になること
- `i420_to_yuy2` / `i420_to_uyvy` / `i422_to_yuy2` / `i422_to_uyvy` が、width 奇数のとき `dst_stride * height` ちょうどのデスティネーションバッファを `Err` にし、`dst_stride * (height - 1) + width * 2 + 2` 以上で `Ok` を返すこと（`dst_stride * height` ちょうどが `Err` になるのは `dst_stride < width * 2 + 2` のとき）
- 偶数幅では従来どおり `stride * height` ちょうどで `Ok` になること（`+2` の要求がないこと）
- ゼロサイズ入力（width == 0 / height == 0）で panic せず、現行どおり `Err` を返すこと（`height - 1` のアンダーフロー対策）
- 境界値テストが `tests/test_convert.rs` に追加されていること（奇数幅の Err / Ok 境界と、偶数幅で従来どおりの Ok。テストコメントで width の奇数 / 偶数の区別と、Coalesce 発動 / 非発動の条件（stride 設定）を明示すること。既存の奇数幅テスト（0044 追加分）は新必要サイズを満たすため変更不要）
- YUY2 / UYVY をソースに持つ他の変換関数（`yuy2_to_argb` / `yuy2_to_i420` / `yuy2_to_i422` / `yuy2_to_nv12` / `uyvy_to_argb` / `uyvy_to_i420` / `uyvy_to_i422` / `uyvy_to_nv12`）と、デスティネーション側の `argb_to_yuy2` / `argb_to_uyvy`（`ARGBToYUY2` / `ARGBToUYVY` も `I422ToYUY2Row_Any_*` 系を使うため書き込み越えの可能性がある）について、同構造の読み書き越えがあるか（関数ごとの Coalesce 条件も含めて）調査し、結果と適用の有無を本 issue に追記して確定すること（同構造の関数には同じ検証パターンを適用する）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `src/convert/packed.rs` の `yuy2_to_y` / `uyvy_to_y` に、Coalesce を考慮したソース必要サイズのインライン検証を追加する
2. `src/convert/packed.rs` の `i420_to_yuy2` / `i420_to_uyvy` / `i422_to_yuy2` / `i422_to_uyvy` に、奇数幅時のデスティネーション必要サイズのインライン検証を追加する
3. 上記の検証に、`+2` の根拠（libyuv の行関数ラッパーの余り処理と Coalesce）をコメントで明記する
4. YUY2 / UYVY をソースに持つ他の変換関数と、デスティネーション側の `argb_to_yuy2` / `argb_to_uyvy` を調査し、同構造の関数に同じ検証を適用して結果を本 issue に追記する
5. `tests/test_convert.rs` に境界値テストを追加する
6. `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加する
