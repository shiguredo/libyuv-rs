# fuzz_compare.rs が存在しない PlanarImage 型を参照している

- Priority: High
- Created: 2026-07-29
- Completed: {YYYY-MM-DD}
- Model: qwen3.8-max-preview
- Branch: feature/fix-fuzz-compare-planar-image
- Polished: 2026-07-29
- Reporter: @voluntas

## 目的

`fuzz/fuzz_targets/fuzz_compare.rs` が存在しない `PlanarImage` 型を参照しており、fuzz クレートがコンパイル不能になっている問題を修正する。

## 優先度根拠

High。fuzz ターゲットがコンパイル不能なため、compare 関数のパニック安全性検証が完全に停止している。

## 現状

`fuzz/fuzz_targets/fuzz_compare.rs:53,58` で `PlanarImage` 構造体を使用しているが、`src/` 内に `PlanarImage` の定義は存在しない。正しい型は `I420Image`（`define_yuv_image!` マクロで定義。フィールド `y`, `y_stride`, `u`, `u_stride`, `v`, `v_stride` は同一）。

## 設計方針

`PlanarImage` を `I420Image` に置き換える。フィールド名・構造は同一のため、型名の置換だけで済む。

## 完了条件

- `fuzz/fuzz_targets/fuzz_compare.rs` の `PlanarImage` が `I420Image` に置き換わっていること
- `cargo fuzz build` または `cargo check -p shiguredo-libyuv-fuzz` が成功すること（fuzz クレートのコンパイル確認）
- `CHANGES.md` の `## develop` セクションの `### misc` に `[FIX]` エントリを追記すること（@voluntas 署名付き）
