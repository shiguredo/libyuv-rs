# fuzz ターゲットのカバレッジを拡充し CI でビルド検証する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-expand-fuzz-targets
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`fuzz/` のターゲットが公開関数の一部しか呼んでおらず（rotate は 20 関数中 3 関数、scale は 18 関数中 4 関数、convert は 243 関数中 42 関数 + fuzz_mjpeg の 5 関数、planar は 67 関数中 23 関数）、大半の関数のパニック安全性が担保されていない。さらに fuzz ターゲットが workspace 外のため CI でコンパイル検証すらされず、API 変更への追随漏れが検出できない。ターゲット拡充と CI 組み込みを行う。

## 優先度根拠

Medium。

- fuzz に未到達の関数のパニック安全性が未検証（16bit 系・colorspace 系・jpeg 系・packed 系・hardware 系は未到達。planar の 16bit 系・ARGB 加工系も未到達）
- `fuzz/` は `Cargo.toml` の workspace `exclude` にあるため CI（fmt / clippy / test）でコンパイルされず、過去に「旧汎用画像型を参照してコンパイル不能」になった履歴がある（CHANGES.md に FIX 記載）

## 現状

- `fuzz/fuzz_targets/fuzz_rotate.rs`: `i420_rotate` / `argb_rotate` / `rotate_plane` の 3 関数のみ到達。`rotate_plane_90/180/270` / `transpose_plane` / `split_rotate_uv` 系 / `split_transpose_uv` / `i010_rotate` / `i210_rotate` / `i410_rotate` / `nv12_to_i420_rotate` / `android420_to_i420_rotate` / `i422_rotate` / `i444_rotate` / `rotate_plane_16` が未到達
- `fuzz/fuzz_targets/fuzz_scale.rs`: `i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane` の 4 関数のみ。`scale_plane_12/16` / `i420_scale_12/16` / `i422_scale(_12/_16)` / `i444_scale(_12/_16)` / `nv24_scale` / `uv_scale(_16)` / `argb_scale_clip` が未到達
- `fuzz/fuzz_targets/fuzz_convert.rs`: 42 関数のみ。`pool.clone()` による入力の使い回し（`rgb` / `u422` / `v422` 等が同一データになる）で入力多様性が失われている（なお `-max_len=4096` のためプールは前半の take で枯渇し、後半は 0 埋めが主になる。多様性の回復は入力が残っている小サイズ画像に限られる）
- `fuzz/fuzz_targets/fuzz_planar.rs`: 23 関数のみ。16bit 系（`copy_plane_16` / `merge_uv_plane_16` 等）・ARGB 加工系（`argb_blur` / `argb_quantize` / sobel 系 / color_table 系等）が未到達（0053 の現状列挙を参照）
- `fuzz/fuzz_targets/fuzz_compare.rs` / `fuzz_mjpeg.rs`: compare 系 6 関数 / mjpeg 系 5 関数に到達
- CI（`.github/workflows/ci.yml`）に fuzz のビルド・実行ステップが存在しない（fuzz/ は workspace 外のため `cargo fmt --all --check` / `cargo clippy --workspace` も fuzz を検証しない。なお fuzz の既存コードは rustfmt 未整形のままである（6 ターゲット中 5 ターゲット（fuzz_mjpeg を除く）に差分あり））

## 設計方針

- 未到達の公開関数を fuzz ターゲットに追加する（パニック安全性の検証）: rotate は全 20 関数、scale は全 18 関数、convert は 0052 の現状列挙のうち fuzz 未到達の関数群（argb / i420 / nv / subsampling 系の未到達関数を含む）の代表関数、planar は 0053 の現状列挙のうち fuzz 未到達の関数群（16bit 系・ARGB 加工系の代表関数。convert / planar とも対象関数の選定一覧を実装時に issue に追記して確定する）。compare は 6 関数到達済みでパニック安全性を担保しており、未到達の 3 関数（calc_frame_psnr / calc_frame_ssim / compute_sum_square_error）は本 issue の対象外とする
- `fuzz/fuzz_targets/fuzz_convert.rs` の `pool.clone()` を `&mut pool` の消費に変更し、入力多様性を回復する（プール枯渇後は 0 埋めになるため、効果は入力が残っている小サイズ画像に限られる）
- CI に `cargo check --manifest-path fuzz/Cargo.toml --features source-build` を追加し、コンパイルエラーを CI で検出する（fuzz/Cargo.toml に `[features] source-build = ["shiguredo_libyuv/source-build"]` を追加し、feature を転送する。素の `cargo check` は prebuilt 依存になり version bump 直後に失敗するため、source-build を明示する。`cargo +nightly fuzz build` は CI に nightly ツールチェーンと cargo-fuzz を導入しないため使わない。配置は 9 OS マトリクス全件ではなく単一 OS（ubuntu-24.04）の専用ジョブに追加し、既存 test ジョブと同様に NASM のインストールステップを含める（x86_64 では libjpeg-turbo のソースビルドに NASM が必須））。fmt も `cargo fmt --manifest-path fuzz/Cargo.toml --check` を検証に含める（現状の fuzz コードは rustfmt 未整形のため、既存ターゲット全体の整形を本 issue のスコープに含める）。clippy は fuzz に適用しない（check でコンパイル検証は担保される）
- fuzz ターゲットの実行は時間がかかるため、CI ではビルド検証までとし、実行は Makefile の `make fuzzing` に委ねる（現行の運用を維持）

## 完了条件

- rotate の全 20 関数・scale の全 18 関数が fuzz ターゲットから呼ばれること
- convert / planar の fuzz 未到達関数群のうち、選定した代表関数（設計方針のとおり。選定一覧を issue に追記して確定）が fuzz ターゲットから呼ばれること
- `fuzz_convert.rs` の `pool.clone()` が解消され、入力が消費されること
- CI に fuzz ターゲットのビルド検証ステップが追加されていること（`cargo check --manifest-path fuzz/Cargo.toml --features source-build` と `cargo fmt --manifest-path fuzz/Cargo.toml --check`）
- `cargo check --manifest-path fuzz/Cargo.toml --features source-build` と `cargo fmt --manifest-path fuzz/Cargo.toml --check` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` / `[UPDATE]` エントリ（fuzz ターゲット追加と CI ステップ追加）を追加すること

## 解決方法

1. `fuzz_rotate.rs` / `fuzz_scale.rs` に未到達関数の呼び出しを追加する
2. `fuzz_convert.rs` に convert の代表関数の呼び出しを追加し、`pool.clone()` を修正する
3. `fuzz_planar.rs` に planar の代表関数（16bit 系・ARGB 加工系）の呼び出しを追加する
4. 既存 fuzz ターゲット全体を rustfmt で整形する
5. `fuzz/Cargo.toml` に `[features] source-build` を追加し、feature を転送する
6. `.github/workflows/ci.yml` に fuzz のビルド検証ステップ（単一 OS）を追加する
7. `CHANGES.md` の `### misc` にエントリを追加する
