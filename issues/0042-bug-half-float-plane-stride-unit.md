# half_float_plane の stride 単位が libyuv の仕様（バイト）と不一致で出力が壊れる

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-half-float-plane-stride-unit
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/planar.rs` の `half_float_plane` が stride を要素数（u16 単位）として検証・渡している一方、libyuv の `HalfFloatPlane` は stride をバイト単位と規定している。この不一致により、典型的な呼び出し（stride == width）で出力が無言で破壊される。stride の扱いを仕様に合わせて修正する。

## 優先度根拠

High。

- エラーを返さずに出力がゴミになる（サイレントなデータ破壊）
- `src_stride >= width` や `checked_buf_size` の検証はすべて通るため、エラーも出ない

## 現状

libyuv の `planar_functions.h` の `HalfFloatPlane` には次の注記がある（commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）:

> Note: Unlike other libyuv functions that operate on uint16_t buffers, the src_stride_y and dst_stride_y parameters of HalfFloatPlane() are in bytes, not in units of uint16_t.

実装（`planar_functions.cc` の `HalfFloatPlane`）は冒頭で `src_stride_y >>= 1; dst_stride_y >>= 1;` を実行する。

一方 `src/planar.rs` の `half_float_plane` は:

- `src_stride < size.width` / `dst_stride < size.width` を要素数単位で検証
- `checked_buf_size(src_stride, size.height)` を要素数単位で検証
- その値をそのまま `sys::HalfFloatPlane` に渡している

そのため stride == width の典型的な入力では、C 側の行送りが width / 2 要素となり行が重なり合い、検証はすべて通過するのに出力がゴミになる。逆にバイト単位で渡すと呼び出し側の検証（要素数計算）に引っかかり呼び出せない。

## 設計方針

Rust の公開 API は他関数と一貫して「stride は要素数」のまま維持し、`sys::HalfFloatPlane` に渡す直前に `* 2` してバイト単位に変換する（C 側が `>>= 1` するため、結果的に要素数 stride で行送りされる）。

- 検証（要素数単位の `require_c_int` / `checked_buf_size` / `stride >= width`）は現状のままで正しくなる
- `* 2` のオーバーフローは `checked_mul` で検証し、`require_c_int` の範囲を超える場合はエラーにする
- docstring に「stride は u16 要素数」であることを明記する

## 完了条件

- `half_float_plane` が stride == width の入力で正しい出力を返すこと（変換前後の値が単調・一致する等の検証可能なテスト）
- stride / width / height の `checked_mul` オーバーフロー時および `c_int` 範囲超過時に `Err` を返すこと
- `tests/test_planar.rs` に正常系・エラーパスのテストが追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `half_float_plane` で `sys::HalfFloatPlane` に渡す前に `src_stride` / `dst_stride` を `checked_mul(2)` でバイト単位に変換する
2. 変換値の `require_c_int` チェックを追加する
3. docstring に stride の単位（u16 要素数）を明記する
4. `tests/test_planar.rs` にテストを追加する
5. `CHANGES.md` に `[FIX]` エントリを追加する
