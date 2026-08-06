# half_float_plane の stride 単位が libyuv の仕様（バイト）と不一致で出力が壊れる

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-half-float-plane-stride-unit
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/planar.rs` の `half_float_plane` が stride を要素数（u16 単位）として検証・渡している一方、libyuv の `HalfFloatPlane` は stride をバイト単位と規定している。この不一致により、典型的な呼び出し（stride == width）で出力が無言で破壊される。stride の扱いを仕様に合わせて修正する。

## 優先度根拠

High。

- エラーで検出できないままサイレントにデータが破壊されるため、呼び出し側で気づく手段がない

## 現状

libyuv の `planar_functions.h` の `HalfFloatPlane` には次の注記がある（commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）:

> Note: Unlike other libyuv functions that operate on uint16_t buffers, the src_stride_y and dst_stride_y parameters of HalfFloatPlane() are in bytes, not in units of uint16_t.

実装（`planar_functions.cc` の `HalfFloatPlane`）は冒頭で `src_stride_y >>= 1; dst_stride_y >>= 1;` を実行する。

一方 `src/planar.rs` の `half_float_plane` は:

- `src_stride < size.width` / `dst_stride < size.width` を要素数単位で検証
- `checked_buf_size(src_stride, size.height)` を要素数単位で検証
- その値をそのまま `sys::HalfFloatPlane` に渡している

そのため stride == width の典型的な入力では、C 側の行送りが width / 2 要素となり行が重なり合い、検証はすべて通過するのに出力がゴミになる。逆にバイト単位の値（2 × width）を要素数として渡すと、典型的な contiguous バッファでは `checked_buf_size` の計算（2 × width × height 要素）に引っかかりエラーになる。

再現例: width = 8、height = 4、stride = 8（要素数）、src / dst に 32 要素のバッファを渡すと、C 側の行送りが 4 要素になり、行 1 は要素 4 から、行 2 は要素 8 ではなく要素 8 行目の値が行 1 の後半に重なって出力される。エラーは返らず結果のみ壊れる。

## 設計方針

Rust の公開 API は他関数と一貫して「stride は要素数」のまま維持し、`sys::HalfFloatPlane` に渡す直前に `* 2` してバイト単位に変換する（C 側が `>>= 1` するため、結果的に要素数 stride で行送りされる）。

- 検証（要素数単位の `require_c_int` / `checked_buf_size` / `stride >= width`）は現状のままで正しくなる
- 変換値の `require_c_int` チェックは既存の `require_c_int` 検証の直後（バッファサイズ検証より前）に置く。そうしないと 2 GiB 超のバッファ確保がテストに要求される
- 変換した値が `c_int` の範囲を超える場合はエラーにする（`require_c_int` を通過した stride の 2 倍は `usize` のオーバーフローを起こさないため、オーバーフロー検査は不要）
- docstring に「stride は u16 要素数」であることを明記する

## 完了条件

- `half_float_plane` が stride == width（height >= 2）の入力で正しい出力を返すこと。ただし stride == width では C 側の行合体（coalesce）により行ごとの stride 送りが検証されないため、stride > width（パディング付き）の入力でも行配置の検証（各行の入力値が出力の正しい行位置に配置されること）が行えること
- 正常系の検証は参照実装を使わない既知解で行うこと。scale = 1.0 では 2 の冪の入力が変換後も厳密に一致する（libyuv の `HalfFloatRow` は全実装（C / NEON / AVX2 / F16C）が `bits >> 13` による truncation であり、2 の冪は下位 13 bit が全て 0 のため）ため、出力を直接期待値として検証できること
- stride の 2 倍変換が `c_int` の範囲を超える場合に `Err` を返すこと（`checked_buf_size` のオーバーフローは 64-bit では到達不能なためテスト対象外）
- docstring に stride の単位（u16 要素数）が明記されていること
- 上記のテストが `tests/test_planar.rs`（0053 で分割された場合は `tests/test_planar/` 配下）に追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `half_float_plane` で `sys::HalfFloatPlane` に渡す前に `src_stride` / `dst_stride` を 2 倍してバイト単位に変換する
2. 変換値の `require_c_int` チェックを既存の `require_c_int` 検証の直後（バッファサイズ検証より前）に追加する
3. docstring に stride の単位（u16 要素数）を明記する
4. 正常系のテスト（stride == width / stride > width の既知解による行配置検証）とエラーパス・境界値のテスト（stride × 2 の `c_int` 範囲超過）を `tests/test_planar.rs`（0053 で分割された場合は `tests/test_planar/` 配下）に追加する
5. `CHANGES.md` に `[FIX]` エントリを追加する
