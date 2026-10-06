# fuzz ターゲットが存在しない型を参照してコンパイルできない

- Priority: High
- Created: 2026-06-19
- Completed:
- Model: Kimi K2.7 Code
- Branch: bugfix/fuzz-targets-compile-error
- Polished: 2026-06-27
- Reporter:

## 目的

`fuzz/fuzz_targets/` 以下のターゲットが、現在の公開 API に存在しない汎用画像型（`PlanarImage` / `PlanarImageMut` / `BiplanarImage` / `BiplanarImageMut` / `PackedImage` / `PackedImageMut`）を参照しており、コンパイルできない問題を修正する。

## 優先度根拠

fuzz ターゲットが現在ビルド不能であり、継続的な fuzzing が実行できない。`cargo check --manifest-path fuzz/Cargo.toml` では 83 箇所の `E0422` エラーが発生する。

## 現状

- `fuzz/fuzz_targets/fuzz_compare.rs`、`fuzz_convert.rs`、`fuzz_planar.rs`、`fuzz_rotate.rs`、`fuzz_scale.rs` で上記の存在しない型を使用している。
- これらの型は、`src/lib.rs` のマクロで定義・公開されている形式固有型（`I420Image` / `Nv12Image` / `ArgbImage` 等）に置き換わった。

## 影響範囲

`fuzz/` ディレクトリ内の fuzz ターゲットのみ。ライブラリ本体（`src/`）および公開 API には変更を加えない。

## 設計方針

各関数のシグネチャに合わせて、実在する形式固有型に置き換える。具体的な型名や変数分割は実装時に各ターゲットの呼び出し先を確認して決定する。

## 完了条件

- `cargo check --manifest-path fuzz/Cargo.toml` が成功すること
- `cargo fuzz build --fuzz-dir fuzz` が成功すること
- `take` / `even_dim` 等の重複ヘルパーは本 issue の完了条件に含めない（別途検討する）

## 実装前の確認事項

- 各 fuzz ターゲットが呼び出している API のシグネチャを確認し、対応する形式固有型（および `*Mut` 型）を特定すること
- `fuzz_convert.rs` では `Nv12ImageMut` と `Nv21ImageMut` を同じ変数に束縛できない箇所があるため、変数を分離する必要がある可能性がある
- ABGR 系変換には `AbgrImage` / `AbgrImageMut` を使用する必要がある可能性がある
- 変更後は `cargo check` と `cargo fuzz build` の両方を実行し、エラーが解消されていることを確認すること
