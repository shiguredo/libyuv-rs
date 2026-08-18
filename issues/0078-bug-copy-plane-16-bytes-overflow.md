# copy_plane_16 が stride * 2 / width * 2 の int オーバーフロー検証を欠いている

- Priority: Medium
- Created: 2026-08-18
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-copy-plane-16-overflow
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/planar.rs` の `copy_plane_16` は、要素単位の `require_c_int` チェックのみで `sys::CopyPlane_16` を呼んでいる。`sys::CopyPlane_16`（libyuv の C 実装、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点の `planar_functions.cc`）は内部でバイト換算（`src_stride_y * 2` / `dst_stride_y * 2` / `width * 2`）を `int` で計算するため、要素単位では c_int 範囲内でも `INT_MAX / 2` 超の stride / width が通過し、C 側の int 乗算がオーバーフロー（未定義動作）になる経路がある。バイト単位の c_int 再検証を追加して経路をなくす。

## 優先度根拠

Medium。

- 有効な入力（c_int 範囲内）で C 側の int オーバーフロー（UB）になり得る。ただし発現には `src_stride > c_int::MAX / 2`（約 10 億要素以上、2 GiB 超）のバッファが必要で、実用的な入力では到達しにくい
- libyuv 同関数をラップする他の公開関数（`convert_to_lsb_plane_16` 等）では同一の検証を追加済みであり、`copy_plane_16` だけ検証が欠落している状態

## 現状

- `src/planar.rs` の `copy_plane_16` は、要素単位の `require_c_int`（width / height / src_stride / dst_stride）と `stride >= width`、`checked_mul` によるバッファサイズ検証のみを行い、`sys::CopyPlane_16` を呼ぶ
- libyuv の `CopyPlane_16`（`planar_functions.cc`）は `CopyPlane((const uint8_t*)src_y, src_stride_y * 2, (uint8_t*)dst_y, dst_stride_y * 2, width * 2, height)` を呼ぶ。`src_stride_y * 2` / `dst_stride_y * 2` / `width * 2` は `int` 演算で、引数が `INT_MAX / 2` 超だとオーバーフローする
- `convert_to_lsb_plane_16`（`src/planar.rs`）は depth == 16 の恒等コピーに `sys::CopyPlane_16` を使う際、バイト単位の c_int 再検証（`half_float_plane` と同じ配置）を追加している。`copy_plane_16` は同関数の公開ラッパーだが、この再検証を持たない

## 設計方針

- `copy_plane_16` に、`half_float_plane` / `convert_to_lsb_plane_16` と同じパターンのバイト単位 c_int 再検証を追加する
- エラーの `function` は既存検証と同じ `"CopyPlane_16"` とする
- 再検証はバッファサイズ検証より前に置く（対になる巨大なバッファが無くても検証に到達できるようにする。`half_float_plane` と同じ配置）
- ゼロサイズ入力（height == 0 時はバッファサイズ 0、width / height == 0 は no-op）の既存の挙動は変更しない

## 完了条件

- `copy_plane_16` で、要素単位では c_int 範囲内でも `stride * 2` / `width * 2` が c_int 範囲を超える場合に `Err` が返ること
- 既存の挙動（正常系・バッファ不足・stride 不足・ゼロサイズ）が変わらないこと
- テストが `tests/test_planar.rs` に追加されていること（バイト単位 c_int 超過のエラーパス。`half_float_plane` / `convert_to_lsb_plane_16` の同型テストを参照）
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `src/planar.rs` の `copy_plane_16` で、要素単位の `require_c_int` の後に、`src_stride * 2` / `dst_stride * 2` / `size.width * 2` のバイト単位 `require_c_int` を追加する（`half_float_plane` と同じ配置・エラーメッセージ形式）
2. バッファサイズ検証より前に再検証を置く（巨大バッファを要求せずにエラーパスを検証可能にする）
3. `tests/test_planar.rs` にバイト単位 c_int 超過のエラーパステストを追加する
4. `CHANGES.md` の `## develop` に `[FIX]` エントリを追加する
