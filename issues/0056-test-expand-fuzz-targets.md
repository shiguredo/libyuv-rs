# fuzz ターゲットのカバレッジを拡充し CI でビルド検証する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-expand-fuzz-targets
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`fuzz/` のターゲットが公開関数の一部しか呼んでおらず（rotate は 20 関数中 3 関数、scale は 18 関数中 4 関数、convert は 251 関数中 42 関数）、大半の関数のパニック安全性が担保されていない。さらに fuzz ターゲットが workspace 外のため CI でコンパイル検証すらされず、API 変更への追随漏れが検出できない。ターゲット拡充と CI 組み込みを行う。

## 優先度根拠

Medium。

- fuzz に未到達の関数（`rotate_plane_16` / `scale_plane_16` / `i010_rotate` / `uv_scale` / `argb_scale_clip` 等）のパニック安全性が未検証
- `fuzz/` は `Cargo.toml` の workspace `exclude` にあるため、CI（fmt / clippy / test）でコンパイルされず、過去に「旧汎用画像型を参照してコンパイル不能」になった履歴がある（CHANGES.md に FIX 記載）

## 現状

- `fuzz/fuzz_targets/fuzz_rotate.rs`: `i420_rotate` / `argb_rotate` / `rotate_plane` の 3 関数のみ到達。`rotate_plane_90/180/270` / `transpose_plane` / `split_rotate_uv` 系 / `i010_rotate` / `i210_rotate` / `i410_rotate` / `nv12_to_i420_rotate` / `android420_to_i420_rotate` / `i422_rotate` / `i444_rotate` / `rotate_plane_16` が未到達
- `fuzz/fuzz_targets/fuzz_scale.rs`: `i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane` の 4 関数のみ。`scale_plane_12/16` / `i420_scale_12/16` / `i422_scale(_12/_16)` / `i444_scale(_12/_16)` / `nv24_scale` / `uv_scale(_16)` / `argb_scale_clip` が未到達
- `fuzz/fuzz_targets/fuzz_convert.rs`: 42 関数のみ。`pool.clone()` による入力の使い回し（`rgb` / `u422` / `v422` 等が同一データになる）で入力多様性が失われている
- CI（`.github/workflows/ci.yml`）に fuzz のビルド・実行ステップが存在しない

## 設計方針

- 未到達の公開関数を fuzz ターゲットに追加する（パニック安全性の検証）
- `fuzz/fuzz_targets/fuzz_convert.rs` の `pool.clone()` を `&mut pool` の消費に変更し、入力多様性を回復する
- CI に `cargo check --manifest-path fuzz/Cargo.toml`（または `cargo fuzz` が使える環境なら `cargo +nightly fuzz build`）を追加し、コンパイルエラーを CI で検出する
- fuzz ターゲットの実行は時間がかかるため、CI ではビルド検証までとし、実行は Makefile の `make fuzzing` に委ねる（現行の運用を維持）

## 完了条件

- rotate の全 20 関数・scale の全 18 関数が fuzz ターゲットから呼ばれること
- `fuzz_convert.rs` の `pool.clone()` が解消され、入力が消費されること
- CI に fuzz ターゲットのビルド検証ステップが追加されていること
- `cargo check --manifest-path fuzz/Cargo.toml` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. `fuzz_rotate.rs` / `fuzz_scale.rs` に未到達関数の呼び出しを追加する
2. `fuzz_convert.rs` の `pool.clone()` を修正する
3. `.github/workflows/ci.yml` に fuzz のビルド検証ステップを追加する
4. `CHANGES.md` の `### misc` にエントリを追加する
