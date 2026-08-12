# i420_ssim が width または height 9〜16 サイズで Ok(NaN) を返す

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-ssim-nan-size
- Polished: 2026-08-12
- Reporter: @voluntas

## 目的

`src/compare.rs` の `i420_ssim` が、width / height が 9〜16 の入力で `Ok(NaN)` を返す。closed 0039 は `width <= 8 || height <= 8` のみを `Err` 化したが、U/V プレーンが 8 以下になるサイズ（width または height が 9〜16）の NaN が残存している。`Err` 化する最小サイズを修正する。

## 優先度根拠

High。

- `Ok(NaN)` を返すため、呼び出し側が NaN を意識せずに使用すると比較結果が不正になる
- 0039 の修正が不完全（Y プレーン基準でチェックしたが、SSIM の計算は U/V プレーン（`(width + 1) >> 1` x `(height + 1) >> 1` サイズ）で行われるため、9〜16 が漏れた）

## 現状

- `src/compare.rs` の `i420_ssim` は `if size.width <= 8 || size.height <= 8` で `Err` を返す（0039 の修正）
- libyuv 側の `I420Ssim`（`compare.cc`）は Y プレーンを元サイズ、U/V プレーンを `(width + 1) >> 1` x `(height + 1) >> 1` サイズで `CalcFrameSsim` に渡す。`CalcFrameSsim` は width または height の片方が 8 以下のときサンプル数が 0 になり `ssim_total /= samples` の除算ゼロで NaN を返す
- したがって width または height が 9〜16 のとき、Y プレーンは 9x9 以上（チェック通過）だが U/V プレーンは 8 以下になり `Ok(NaN)` が返る（例: 16x16 では U/V が 8x8、17x16 では U/V が 9x8 で NaN）
- 再現: `i420_ssim` に width = 16, height = 16 の画像（内容は問わない。NaN は `samples == 0` のみが原因で画像内容に依存しない）を渡すと `Ok(NaN)` が返る（width / height が両方 17 以上では有限値）
- なお `calc_frame_ssim` は単一プレーンを `CalcFrameSsim` に直接渡す関数で U/V 縮小がなく、width / height が両方 9 以上のとき有限値を返すため、現行の `width <= 8 || height <= 8` チェックが正しく、本 issue の変更対象外

## 設計方針

`i420_ssim` のサイズチェックを「U/V プレーンが 9x9 以上になるサイズ」（width >= 17 かつ height >= 17）に修正し、17 未満は `Err` を返す（`width <= 16 || height <= 16` で `Err`）。エラーメッセージは 17x17 の要件に合わせて更新する（例: "image must be at least 17x17 for SSIM calculation (U/V planes are downsampled)"）。`calc_frame_ssim` のチェックは変更しない。

## 完了条件

- `i420_ssim` が width または height が 16 以下（境界値: 16 / 17）で `Err` を返すこと（16x16、17x16、16x17 の非対称ケースを含む）
- `i420_ssim` が width / height が 17 以上の正常入力で `Ok` を返し、NaN でないこと（17x17）
- `calc_frame_ssim` の挙動が変更されないこと（9x9 で `Ok`、8x8 で `Err` のまま。既存の 8x8 / 4x4 の Err テスト（0039 で追加）が引き続き成功すること）
- `i420_ssim` の docstring に、width または height が 17 未満で `Err` になる要件が明記されていること
- 境界値テストが `tests/test_compare.rs` に追加されていること（i420_ssim の 16x16 / 17x16 / 16x17 で `Err`（17x17 要件のエラーメッセージが返ることも確認する。`validate` が先に Err を返さないよう U/V バッファの stride を正しく設定すること）、17x17 で Ok かつ NaN でない。テストコメントで width / height の区別を明示すること。0071 の calc_frame_ssim テスト（9x9 で Ok）とは対象関数が異なり重複しない）
- `cargo test --workspace --features source-build` が成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `src/compare.rs` の `i420_ssim` のサイズチェックを `width <= 16 || height <= 16` で `Err` を返すように修正し、エラーメッセージと関連コメント（現行の `width <= 8` の説明）を更新し、docstring に 17x17 未満で `Err` になる要件を明記する
2. `tests/test_compare.rs` に境界値テスト（16x16 / 17x16 / 16x17 で Err、17x17 で Ok かつ NaN でない）を追加する
3. `CHANGES.md` に `[FIX]` エントリ（例: 「i420_ssim が 9〜16 サイズで Ok(NaN) を返す問題を修正する」。0039 のエントリと同形式）を追加する
