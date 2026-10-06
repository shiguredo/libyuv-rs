# compare / rotate / scale の docstring の不正確さと情報不足を修正する

- Priority: Low
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/doc-fix-docstrings
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/compare.rs` / `src/rotate.rs` / `src/scale.rs` の docstring に、実装や libyuv の仕様と食い違う記載・重要な制約の未記載がある。利用者が誤った前提で API を使わないよう修正する。

## 優先度根拠

Low。

- 動作には影響しない（ドキュメントの問題）
- ただし「0.0 から 1.0 の範囲」のような誤記載は、負の SSIM を受け取った利用者を混乱させる

## 現状

### src/compare.rs

- 52 行・191 行: `i420_ssim` / `calc_frame_ssim` の「返り値は 0.0 から 1.0 の範囲」— libyuv の `Ssim8x8` は強い逆相関のデータで負の SSIM を返しうる（分子の第二因子が負になる）。1.0 が完全一致である点のみ正しい
- `compute_sum_square_error`: `count == 0` の挙動（エラーにならず 0 を返す）が docstring にない（`sum_square_error_to_psnr` は count == 0 をエラーにするのに同列の関数で説明がない）
- `compute_hamming_distance`: 長さが異なる場合に短い方に切り詰める仕様が docstring にない
- `hash_djb2`: `seed` 引数の意味（初期ハッシュ値、libyuv ヘッダは 5381 を推奨）が docstring にない

### src/scale.rs

- `uv_scale`: `width` が「UV ペア数」であること（stride はバイト数）が docstring にない。width をバイト数と誤解すると検証をすり抜ける
- `uv_scale_16`: libyuv の `UVScale_16` は「This function is currently incomplete, it can't handle all cases.」と明記され、コピー（同一サイズ / 整数倍縦縮小）と 2 倍アップスケール以外は -1 を返す。この制約が docstring にない
- `argb_scale_clip`: `clip_width == 0` / `clip_height == 0` の挙動が経路依存で不定（無書きの Ok または Err）なのに未記載

### src/rotate.rs

- 全 rotate 関数に「`dst_size` が回転後サイズと一致しない場合は Err」のパスがあるが、90 / 270 度では dst_size を (height, width) に置き換える必要があることが docstring に明記されていない
- `rotate_plane_180` 等の docstring が 1 行のみで説明不足

## 設計方針

- 各 docstring を実装・libyuv の C 実装と照合して正確な記述に修正する
- 制約（切り詰め・不完全実装・ゼロクリップの挙動）を明記する
- 修正はドキュメントのみとし、動作は変更しない

## 完了条件

- 上記の docstring がすべて実装の実態と一致する記載になっていること
- `cargo doc --no-deps`（`DOCS_RS=1` 以外の通常パス）が成功すること
- 動作変更がないこと（`cargo test --workspace --features source-build` が成功すること）

## 解決方法

1. 各 docstring を修正する
2. `cargo doc --no-deps` とテストで確認する
