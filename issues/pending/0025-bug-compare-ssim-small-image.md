# SSIM 関数が小さな画像で 0 除算 / NaN を返す

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-compare-ssim-small-image
- Polished: {YYYY-MM-DD}
- Reporter:

## 目的

`compare.rs` の SSIM 関数が小さな画像に対して libyuv 内部で 0 除算を引き起こし、結果として NaN または SIGFPE を返すバグを修正する。

## 優先度根拠

無効な入力サイズに対して無効な結果 ( NaN ) またはプロセスクラッシュ ( SIGFPE ) を引き起こす。利用者が結果を信頼できなくなる。

## 現状

`src/compare.rs` の `i420_ssim` と `calc_frame_ssim` は、libyuv の `CalcFrameSsim` を呼び出している。`CalcFrameSsim` は `width < 9` または `height < 9` の場合、サンプル収集ループが 1 回も実行されず `samples == 0` となり、その後 `ssim_total /= samples` で 0 除算が発生する。

`i420_ssim` は Y/U/V 平面それぞれで `CalcFrameSsim` を呼び出す。元画像が 16x16 の場合、U/V 平面は 8x8 となり 0 除算する。

## 設計方針

Rust ラッパー側で入力サイズを事前検証し、`CalcFrameSsim` に到達する前にエラーを返す。

## 完了条件

- `calc_frame_ssim` と `i420_ssim` が 9x9 未満の画像に対して `Error` を返すこと
- 該当する境界値テストが追加されていること
- `cargo test --all` が成功すること

## 解決方法

1. `calc_frame_ssim` に `size.width < 9 || size.height < 9` の検証を追加する
2. `i420_ssim` では元画像サイズが 9x9 未満の場合、または U/V 平面サイズが 9x9 未満になる場合をエラーにする
3. `tests/test_compare.rs` を新設し、小サイズ画像での SSIM 呼び出しが `Error` になることをテストする
