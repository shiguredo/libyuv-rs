# compare モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-compare-tests
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/compare.rs` の公開関数 9 個のうち、`compute_sum_square_error`（非 plane 版）と `compute_hamming_distance` は値検証がゼロ。`sum_square_error_to_psnr` の正規式（sse > 0 のケース）と `hash_djb2` の参照値比較が未検証。`calc_frame_ssim` の成功パス（9x9 で Ok）が未検証（closed 0039 は Err パスのみ追加）。テストカバレッジを拡充する。

## 優先度根拠

Medium。

- `compute_sum_square_error` / `compute_hamming_distance` は値検証ゼロ（closed 0039 の calc_frame_ssim NaN バグは値検証テストがなかったために見逃された）
- `sum_square_error_to_psnr` / `hash_djb2` / `calc_frame_ssim` の正常系の値検証がない

## 現状

- `compute_sum_square_error`（非 plane 版。C 実装では plane 版が行ごとに呼ぶコア実装）と `compute_hamming_distance` が値検証ゼロ
- `sum_square_error_to_psnr` の正規式（sse > 0 のケース）が未検証（test_compare.rs は count == 0 と sse == 0 のみ）
- `hash_djb2` の参照値比較が未検証（空スライスと非空の `Ok` のみ）
- `calc_frame_ssim` の成功パス（9x9 で Ok）が未検証（8x8 / 4x4 の Err のみ）

## 設計方針

- `compute_sum_square_error` / `compute_hamming_distance` / `hash_djb2` の参照値比較（C 実装の式と照合できる既知値）を追加する
- `sum_square_error_to_psnr` の正規式（sse > 0）と `calc_frame_ssim` の成功パス（9x9 で Ok、SSIM 値の検証）を追加する
- サイズ 0 のテストは 0064 のスコープ（挙動が関数ごとに 3 パターン混在しているため本 issue では扱わない）

## 完了条件

- `compute_sum_square_error` / `compute_hamming_distance` の参照値比較テストが `tests/test_compare.rs` に存在すること（`compute_hamming_distance` のエラーパスは `require_c_int` のみで実用上テスト不能のため、正常系の参照値比較のみを要求する）
- `hash_djb2` の参照値比較・`sum_square_error_to_psnr` 正規式（sse > 0）・`calc_frame_ssim` 成功パス（9x9 で Ok）がテストされていること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加すること

## 解決方法

1. `compute_sum_square_error` / `compute_hamming_distance` / `hash_djb2` の参照値比較テストを追加する
2. `sum_square_error_to_psnr` の正規式（sse > 0）と `calc_frame_ssim` の成功パス（9x9 で Ok）のテストを追加する
3. `CHANGES.md` の `### misc` に `[ADD]` エントリを追加する
