# i420_ssim / calc_frame_ssim が 9〜16 サイズで Ok(NaN) を返す

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-ssim-nan-size
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/compare.rs` の `i420_ssim` / `calc_frame_ssim` が、width / height が 9〜16 の入力で `Ok(NaN)` を返す。closed 0039 は `width <= 8 || height <= 8` のみを `Err` 化したが、U/V プレーンが 8 以下になるサイズ（width / height が 16 以下）の NaN が残存している。`Err` 化する最小サイズを修正する。

## 優先度根拠

High。

- `Ok(NaN)` を返すため、呼び出し側が NaN を意識せずに使用すると比較結果が不正になる
- 0039 の修正が不完全（Y プレーン基準でチェックしたが、SSIM の計算は U/V プレーン（`(width + 1) >> 1` サイズ）で行われるため、9〜16 が漏れた）

## 現状

- `src/compare.rs` の `i420_ssim` は `if size.width <= 8 || size.height <= 8` で `Err` を返す（0039 の修正）
- libyuv 側の `I420Ssim` は U/V プレーンを `(width + 1) >> 1` サイズで `CalcFrameSsim` に渡し、`CalcFrameSsim` は width / height <= 8 のときサンプル数が 0 になり `ssim_total /= samples` の除算ゼロで NaN を返す
- したがって width / height が 9〜16 のとき、Y プレーンは 9x9 以上（チェック通過）だが U/V プレーンは 5〜8 になり `Ok(NaN)` が返る
- 再現: `i420_ssim` に width = 16, height = 16 の同一画像を渡すと `Ok(NaN)` が返る（17x17 以上では有限値）

## 設計方針

`i420_ssim` / `calc_frame_ssim` のサイズチェックを「U/V プレーンが 9x9 以上になるサイズ」（width / height >= 17）に修正し、17 未満は `Err` を返す。

## 完了条件

- width / height が 16 以下（境界値: 16 / 17）で `Err` を返すこと
- width / height が 17 以上の正常入力で `Ok` を返し、NaN でないこと
- 境界値テストが `tests/test_compare.rs` に追加されていること（16x16 で Err、17x17 で Ok かつ NaN でない。テストコメントで width == 16 と 17 の区別を明示すること）
- `cargo test --workspace --features source-build` が成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `src/compare.rs` の `i420_ssim` / `calc_frame_ssim` のサイズチェックを 17 未満で `Err` を返すように修正する
2. `tests/test_compare.rs` に境界値テスト（16x16 で Err、17x17 で Ok かつ NaN でない）を追加する
3. `CHANGES.md` に `[FIX]` エントリを追加する
