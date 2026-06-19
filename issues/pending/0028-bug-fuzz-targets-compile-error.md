# fuzz ターゲットが存在しない型を参照してコンパイルできない

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-fuzz-targets-compile-error
- Polished: {YYYY-MM-DD}
- Reporter:

## 目的

`fuzz/fuzz_targets/` 以下のターゲットが公開 API に存在しない型を参照しておりコンパイルできない問題を修正する。

## 優先度根拠

fuzz ターゲットが現在ビルド不能であり、継続的な fuzzing が実行できない。`cargo check --manifest-path fuzz/Cargo.toml` で 83 箇所の `E0422` エラーが発生する。

## 現状

`fuzz/fuzz_targets/fuzz_compare.rs`、`fuzz_convert.rs`、`fuzz_planar.rs`、`fuzz_rotate.rs`、`fuzz_scale.rs` で `PlanarImage` / `PlanarImageMut` / `BiplanarImage` / `BiplanarImageMut` / `PackedImage` / `PackedImageMut` を使用している。これらの型は `src/lib.rs` のマクロで定義・公開されている `I420Image` / `Nv12Image` / `ArgbImage` 等の形式固有型に置き換わったため、存在しない。

## 設計方針

各関数のシグネチャに合わせて、実在する形式固有型に置き換える。`fuzz_convert.rs` では `Nv12ImageMut` と `Nv21ImageMut` を同じ変数に束縛できないため、変数を分離する。

## 完了条件

- `cargo check --manifest-path fuzz/Cargo.toml` が成功すること
- `cargo fuzz` 実行環境で各ターゲットがビルドできること
- `take` / `even_dim` 等の重複ヘルパーは別途検討する (本 issue の完了条件には含めない)

## 解決方法

1. `fuzz_compare.rs` の `PlanarImage` を `I420Image` に置き換える
2. `fuzz_convert.rs` の汎用型を `I420Image` / `I422Image` / `I444Image` / `Nv12Image` / `Nv21Image` / `ArgbImage` / `AbgrImage` / `Rgb24Image` および対応する `*Mut` に置き換える
3. `fuzz_convert.rs` の `Nv12ImageMut` と `Nv21ImageMut` の変数を分離する
4. `fuzz_convert.rs` の ABGR 系変換には専用の `AbgrImage` バッファを用意する
5. `fuzz_planar.rs`、`fuzz_rotate.rs`、`fuzz_scale.rs` も同様に形式固有型に置き換える
