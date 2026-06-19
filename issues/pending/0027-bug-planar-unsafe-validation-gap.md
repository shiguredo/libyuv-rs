# planar.rs の unsafe 呼び出し前の入力検証が不十分

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-planar-unsafe-validation-gap
- Polished: {YYYY-MM-DD}
- Reporter:

## 目的

`planar.rs` の `interpolate_plane_16` と `mirror_plane` において、libyuv への unsafe 呼び出し前の入力検証が不十分な箇所を修正する。

## 優先度根拠

オーバーフロー、`c_int` への縮小変換、範囲外メモリアクセス等のリスクがあり、メモリ安全性を損なう可能性がある。

## 現状

### `interpolate_plane_16`

`src/planar.rs:1562-1610` において、以下の検証が欠落している。

- `size.width` / `size.height` / 各 stride の `c_int` 範囲チェック
- `stride >= width` の最小幅チェック
- `stride * size.height` の `checked_mul` によるオーバーフロー防止
- libyuv 戻り値の `Error::check`

### `mirror_plane`

`src/planar.rs:862-879` において、以下の問題がある。

- `Result` を返さない
- `c_int` 範囲チェックなし
- `stride >= width` チェックなし
- バッファサイズチェックなし

同ファイルの `i400_mirror` 等は `Result` を返し検証を行っているため、API 間で不整合がある。

## 設計方針

他の plane 関数と同じく `require_c_int`、`checked_buf_size`、バッファサイズチェックを適用する。`mirror_plane` は `Result<(), Error>` を返すように変更する。

## 完了条件

- `interpolate_plane_16` と `mirror_plane` が不正入力に対して `Error` を返すこと
- 両関数の修正に対する単体テストが追加されていること
- 既存の `cargo test --all` / `cargo clippy` が成功すること

## 解決方法

1. `interpolate_plane_16` に 8bit 版 `interpolate_plane` と同じ検証パターンを適用する
2. `interpolate_plane_16` の libyuv 戻り値を `Error::check` する
3. `mirror_plane` を `Result<(), Error>` を返すように変更し、検証を追加する
4. `tests/test_planar.rs` を新設し、両関数の境界値テストを追加する
