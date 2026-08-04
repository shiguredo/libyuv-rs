# rotate / scale / compare モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-rotate-scale-compare-tests
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/rotate.rs`（20 関数中 17 関数）・`src/scale.rs`（18 関数中 14 関数）・`src/compare.rs`（9 関数中 2 関数）が正しさの検証テストを持たない。fuzz にも未到達の関数が多い。各モジュールのテストカバレッジを拡充する。

## 優先度根拠

Medium。

- エラーパス（`dst_size` 不一致・stride 不足・バッファ不足）が未検証の関数が多い
- `compute_sum_square_error` / `compute_hamming_distance` は値検証ゼロ
- 回転の 90 / 270 度は libyuv が特殊な実装（一時バッファ使用）を持つため、テストで守られていない

## 現状

- **rotate**: 正しさ検証は `i420_rotate` / `argb_rotate` / `rotate_plane` のみ。`rotate_plane_90/180/270` / `transpose_plane` / `split_rotate_uv` 系 / `i010_rotate` / `i210_rotate` / `i410_rotate` / `i422_rotate` / `i444_rotate` / `nv12_to_i420_rotate` / `android420_to_i420_rotate` / `rotate_plane_16` が未テスト。rotate 固有の `dst_size` 不一致エラーパス（`RotationMode::output_size` との照合）も未検証
- **scale**: 正しさ検証は `i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane` のみ（しかも PBT は等倍のみ）。`i422_scale` / `i444_scale` / `i420_scale_12` / `i420_scale_16` / `scale_plane_12` / `scale_plane_16` / `nv24_scale` / `uv_scale` / `uv_scale_16` / `argb_scale_clip` が未テスト
- **compare**: `compute_sum_square_error`（非 plane 版）と `compute_hamming_distance` が値検証ゼロ。`sum_square_error_to_psnr` の正規式（sse > 0 のケース）と `hash_djb2` の参照値比較が未検証。`calc_frame_ssim` の成功パス（9x9 で Ok）が未検証

## 設計方針

- 正常系は ground truth 方式（既知ピクセル列を回転・スケールした期待値との比較）を中心にする
- rotate は 90 / 270 度のラウンドトリップ（4 回で元に戻る）に加え、180 度のラウンドトリップと、`dst_size` 不一致のエラーパスを検証する
- scale は等倍以外（ダウンスケール / アップスケール）の正常系と、`FilterMode` 全モードを検証する
- compare は `compute_sum_square_error` / `compute_hamming_distance` / `hash_djb2` の参照値比較（C 実装の式と照合できる既知値）を追加する

## 完了条件

- rotate の 17 関数、scale の 14 関数、compare の未検証 2 関数に正常系・エラーパスのテストが存在すること
- rotate の `dst_size` 不一致エラーパスがテストされていること
- scale の等倍以外のスケールカーネルがテストされていること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. rotate / scale / compare の順に、ground truth 方式の正常系テストとエラーパステストを追加する
2. `FilterMode` 全モード・`RotationMode` 全モードをカバーする
3. `CHANGES.md` の `### misc` にエントリを追加する
