# fuzz ターゲットが旧汎用画像型を参照してコンパイル不能になっている

- Priority: High
- Created: 2026-07-29
- Completed: 2026-07-29
- Model: qwen3.8-max-preview
- Branch: feature/fix-fuzz-compare-planar-image
- Polished: 2026-07-29
- Reporter: @voluntas

## 目的

全 fuzz ターゲットが旧汎用画像型（`PlanarImage`, `PackedImage`, `BiplanarImage` 等）を参照しており、コンパイル不能になっている問題を修正する。

## 優先度根拠

High。fuzz ターゲットがコンパイル不能なため、全モジュールのパニック安全性検証が完全に停止している。

## 現状

issue 0018 で汎用画像型がフォーマット固有型に置換された際、fuzz ターゲットが追従しなかった。全 5 ファイル（fuzz_compare, fuzz_convert, fuzz_rotate, fuzz_scale, fuzz_planar）で合計 80 件以上のコンパイルエラーが発生している。

## 設計方針

各 fuzz ターゲットで呼ばれている関数のシグネチャに基づき、旧汎用型を正しいフォーマット固有型に置換する。`BiplanarImage` → `Nv12Image` / `Nv21Image` の置換ではフィールド名も `chroma` → `uv`、`chroma_stride` → `uv_stride` に変更する。

## 完了条件

- 全 fuzz ターゲットがフォーマット固有型を使用していること
- `cargo check`（fuzz クレート）が成功すること
- `CHANGES.md` の `## develop` セクションの `### misc` に `[FIX]` エントリを追記すること（@voluntas 署名付き）

## 解決方法

全 5 fuzz ターゲットの旧汎用画像型をフォーマット固有型に置換した:

- `PlanarImage` / `PlanarImageMut` → `I420Image` / `I420ImageMut` / `I422ImageMut` / `I444ImageMut`（関数のシグネチャに応じて使い分け）
- `PackedImage` / `PackedImageMut` → `ArgbImage` / `ArgbImageMut` / `AbgrImageMut` / `Rgb24Image` / `Rgb24ImageMut`
- `BiplanarImage` / `BiplanarImageMut` → `Nv12Image` / `Nv12ImageMut` / `Nv21Image` / `Nv21ImageMut`（フィールド名 `chroma` → `uv`、`chroma_stride` → `uv_stride` も変更）

`CHANGES.md` の `### misc` に `[FIX]` エントリを追加した。
