# tests/test_convert/main.rs の既存テストをサブモジュールに分割する

- Priority: Medium
- Created: 2026-08-18
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-split-test-convert-main
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`tests/test_convert/main.rs` が 3585 行と巨大になっている。convert テストは既に `tests/test_convert/` 配下にサブモジュール分割されているが、既存テスト (86 件) は `main.rs` に残ったままである。既存テストも機能ごとにサブモジュールへ分割し、`main.rs` をモジュール宣言と共有ヘルパーのみに減らすことで、テストファイルの可読性と保守性を向上させる。

## 現状

`tests/test_convert/main.rs` は 3585 行。新規テスト (0052 で追加) はサブモジュール (colorspace / high_bitdepth / jpeg / packed / hardware / argb / i420 / nv / subsampling / alpha / mjpeg) に分かれている一方、既存 86 テストは `main.rs` に残る。既存テストは機能ごとに明確なセクションがあり、以下のグループに分割できる:

- `i420_to_argb` / `nv12_to_i420` のエラーパス (バッファ不足・stride 不足の 4 件)
- `yuy2_to_y` / `uyvy_to_y` の正常系・エラーパス (12 件。Y 成分抽出・パディング・奇数幅・stride 境界・バッファ不足・ゼロサイズ)
- `yuy2_to_argb` / `uyvy_to_argb` / `yuy2_to_i420` / `uyvy_to_i420` / `yuy2_to_i422` / `uyvy_to_i422` / `yuy2_to_nv12` / `uyvy_to_nv12` / `i420_to_yuy2` / `i420_to_uyvy` / `i422_to_yuy2` / `i422_to_uyvy` / `argb_to_yuy2` / `argb_to_uyvy` の奇数幅読み書き越え (16 件 + ヘルパー 2 個)
- `android420_to_i420` / `android420_to_argb` / `android420_to_abgr` の pixel_stride_uv 検証・インターリーブ境界 (14 件)
- `mm21_to_i420` / `mt2t_to_p010` のタイル配置必要サイズ検証 (8 件)
- `detile_plane` / `detile_plane_16` / `detile_to_yuy2` のタイル配置必要サイズ・stride・tile_height 検証 (10 件)
- `p210_to_p410` / `nv12_to_nv24` / `nv16_to_nv24` の stride・バッファ・c_int 検証 (18 件)
- alpha 系 7 関数の stride 境界テスト (7 件 + ヘルパー 1 個)

なお `alpha.rs` には 0052 で追加した alpha 系の正常系・バッファ不足テストが既にあり、`main.rs` の alpha 系 stride 境界テストはこれと統合できる。他の新規サブモジュール名 (colorspace / high_bitdepth / jpeg / packed / hardware / argb / i420 / nv / subsampling / mjpeg) と衝突しない名前で分割する必要がある。

## 設計方針

- 既存テストを機能ごとに新規サブモジュールへ移動する。分割先は以下のとおり:
  - `basic.rs`: `i420_to_argb` / `nv12_to_i420` のエラーパス
  - `yuy2_uyvy.rs`: `yuy2_to_y` / `uyvy_to_y` と YUY2 / UYVY 系の奇数幅読み書き越え（`check_odd_width_boundaries` / `check_even_width_ok` ヘルパー含む）
  - `android.rs`: `android420_*` 系
  - `mm21_mt2t.rs`: `mm21_to_i420` / `mt2t_to_p010` 系
  - `detile.rs`: `detile_plane` / `detile_plane_16` / `detile_to_yuy2` 系
  - `p210_nv24.rs`: `p210_to_p410` / `nv12_to_nv24` / `nv16_to_nv24` 系
  - `alpha.rs` (既存): `main.rs` の alpha 系 stride 境界テスト (`check_alpha_stride_boundaries` ヘルパー含む) を既存の `alpha.rs` に統合する
- 移動は `git mv` ではなく、テストコードをサブモジュールファイルへ移し、`main.rs` から削除する（テストの分割なのでファイル移動ではなくコード移動）
- `main.rs` にはモジュール宣言 (`mod`) と共有の `use` だけを残す。テストの実行はこれをエントリポイントとして `cargo test` で行う
- 各サブモジュールは `use super::*;` で `main.rs` の `use` を参照し、必要な型・関数を引き継ぐ
- 既存テストのロジック・期待値は変更しない（純粋な移動・分割のみ）
- 分割後も `cargo test --workspace --features source-build` で全テストが成功すること

## 完了条件

- `tests/test_convert/main.rs` がモジュール宣言と共有 `use` のみになり、既存テストがすべてサブモジュールへ移動していること
- 上記の分割先サブモジュール (basic / yuy2_uyvy / android / mm21_mt2t / detile / p210_nv24) が新設され、既存 `alpha.rs` に stride 境界テストが統合されていること
- 既存テストの内容（ロジック・期待値）が一切変更されていないこと
- `cargo test --workspace --features source-build` が成功すること（テスト数が現状と同じ 194 件であること）
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にリファクタリングのエントリを追加すること（種別は `[UPDATE]`）

## 解決方法

1. 既存テストを機能ごとにサブモジュールファイルへ移動する（上記設計方針の分割先に従う）
2. `main.rs` をモジュール宣言と共有 `use` のみにする
3. `CHANGES.md` の `### misc` に `[UPDATE]` エントリを追加する
