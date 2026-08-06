# 奇数幅の変換で libyuv の SIMD 余り処理が src バッファを読み越す

- Priority: Medium
- Created: 2026-08-06
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-odd-width-src-overread
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

奇数幅（width が奇数）で src_stride == width * 2 のとき、libyuv の行関数ラッパー（`ANY11` 等の余り処理）が 1 行あたり `width * 2 + 2` バイト読み出すため、最終行で src バッファの末尾を 2 バイト超えて読む。Rust 側のソース検証（`len >= src_stride * height`）を通過する入力で C 側の領域外読み出し（未定義動作）が発生しうる。対応方針を決定し、実装する。

## 優先度根拠

Medium。

- 領域外読み出しは未定義動作だが、libyuv の伝統的な仕様（行関数の余り処理がバッファ末尾の数バイトを読むことを許容）に基づく既知の挙動
- 実際のメモリレイアウトでは隣接領域を読むだけでクラッシュは稀だが、ASAN 等では検出される

## 現状

- `src/convert/packed.rs` の `yuy2_to_y` / `uyvy_to_y` は奇数幅の入力（例: width=5, src_stride=10, src.len()=20, height=2）を受理する
- libyuv の `YUY2ToYRow_Any_*` / `UYVYToYRow_Any_*`（`row_any.cc` の `ANY11` マクロ）は、SIMD の余り処理で行の先頭から `width * 2 + 2` バイトを読む。width=5 の例では最終行が `src[10..22]` となり、バッファ（20 バイト）を 2 バイト超える
- ソース側の検証（`define_packed_image!` の validate）は `len >= src_stride * height` しか要求せず、この読み越しを防げない
- 偶数幅では読み越しは発生しない（余り処理の読み量が行幅の倍数に収まる）

## 設計方針

対応方針は複数考えられるため、実装時に決定する:

- a) ソース側の検証を強化し、奇数幅でバッファに余裕（2 バイト以上）を要求する
- b) docstring に「src バッファの末尾に余裕が必要」と文書化し、現状の挙動を許容する（libyuv の仕様として）
- c) 影響範囲（yuy2 / uyvy 以外の変換関数も同様の問題を持つ可能性）を調査してから方針を決める

## 完了条件

- 対応方針が確定していること
- 方針に応じた実装とテストが完了していること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` にエントリを追加すること

## 解決方法

1. 影響範囲を調査する（yuy2 / uyvy 以外で奇数幅 + パディングなしの入力が受理される関数）
2. 対応方針を決定する（検証強化 / 文書化 / 許容）
3. 実装とテストを追加する
4. `CHANGES.md` にエントリを追加する
