# planar モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-planar-tests
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/planar.rs` の公開関数 67 個のうち、正しさを検証するテストが存在するのは 9 関数のみ（`copy_plane` / `split_uv_plane` / `merge_uv_plane` / `split_rgb_plane` / `merge_rgb_plane` / `i420_mirror` / `argb_mirror` / `nv12_mirror` / `rgb24_mirror` 等）。残りの 44 関数は単体テスト・PBT・fuzz のいずれにも載っておらず、既知の検証バグ（issue 0042 / 0043 / 0049 の対象関数）も含む。テストカバレッジを拡充する。

## 優先度根拠

Medium。

- 既知の検証バグ（`half_float_plane` / `argb_blur` / `depth` 未検証）にテストが存在しないため、修正の回帰検出ができない
- 16bit 系・ARGB 加工系はパニック安全性（fuzz）すら担保されていない

## 現状

完全に未テストの関数群（抜粋）:

- `i400_copy` / `i420_interpolate` / `argb_interpolate` / `interpolate_plane_16`
- `argb_sobel` / `argb_sobel_to_plane` / `argb_sobel_xy`
- `argb_color_matrix` / `rgb_color_matrix` / `argb_polynomial`
- `argb_add` / `argb_subtract` / `argb_multiply`
- `argb_blur` / `argb_compute_cumulative_sum` / `argb_quantize`
- `argb_*_color_table` / `argb_shuffle` / `argb_extract_alpha`
- `argb_copy_alpha` / `argb_copy_y_to_alpha` / `argb_detect`
- `copy_plane_16` / `merge_uv_plane_16` / `split_uv_plane_16`
- `merge_argb_plane` / `split_argb_plane` / `merge_ar64_plane`
- `merge_xr30_plane` / `merge_argb16_to_8_plane` / `mirror_uv_plane`
- `convert_16_to_8_plane` / `convert_8_to_16_plane` / `convert_to_lsb_plane_16` / `convert_to_msb_plane_16`
- `half_float_plane` / `half_merge_uv_plane` / `byte_to_float` / `gauss_plane_f32`
- `set_plane` / `i420_rect` / `argb_rect`（エラーパス未検証）

## 設計方針

- 正常系は ground truth 方式（既知入力に対する期待値）を中心にする
- エラーパスはバッファ不足・stride 不足・c_int 超過・サイズ 0 の境界値を関数群ごとに代表でカバーする
- `src/planar.rs` が 4,437 行と巨大なため、テストファイルは `tests/test_planar/` 配下に機能別サブモジュール（copy / uv / argb / 16bit / float 等）で分割する
- 対応する fuzz ターゲットの拡充は issue 0056 で対応する

## 完了条件

- 上記の未テスト関数のうち、主要な関数（検証ロジックを持つ関数 + 既知のバグ対象関数）に正常系・エラーパスのテストが存在すること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. 関数の優先順位を決める（検証ロジックを持つ関数と既知のバグ対象関数を最優先）
2. ground truth 方式の正常系テストと境界値のエラーパステストを追加する
3. 必要に応じて `tests/test_planar/` サブモジュール化で分割する
4. `CHANGES.md` の `### misc` にエントリを追加する
