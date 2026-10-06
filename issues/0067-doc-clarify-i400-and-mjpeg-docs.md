# i420_to_i400 と mjpeg_size の docstring を実装の実態に合わせる

- Priority: Low
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/doc-clarify-i400-and-mjpeg-docs
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/convert/i420.rs` の `i420_to_i400` と `src/convert/mjpeg.rs` の `mjpeg_size` の docstring が実装の実態と食い違っている。正確な記載に修正する。

## 優先度根拠

Low。

- ドキュメントの正確性の問題（動作は正しい）
- `i420_to_i400` は安全側の過剰検証であり、ドキュメントで要件を明示すれば利用者は誤解しない

## 現状

### i420_to_i400（src/convert/i420.rs）

libyuv の `I400ToI420` と対になる `I420ToI400` は U / V プレーンを一切読まない（C 実装は `(void)src_u;` 等で明示的に無視する）。一方 Rust 側は `src.validate(size, ...)` で I420 の U / V プレーン全体の有効性を要求するため、Y プレーンだけ有効な入力でもエラーになる。docstring に「U / V プレーンは検証対象（C 実装では使用されないが、バッファ要件を検証する）」ことを明記する。

### mjpeg_size（src/convert/mjpeg.rs）

docstring に「JPEG SOF マーカーをパースし」とあるが、libyuv の `MJPGSize` は `LoadFrame`（`ValidateJpeg` + `jpeg_read_header` 全体）を実行する。SOF だけをパースしているわけではないため、記載を修正する。

## 設計方針

- 2 関数の docstring を実装（C 実装）の実態に合わせて修正する
- 修正はドキュメントのみとし、動作は変更しない
- 動作変更（検証の緩和等）が必要と判断した場合は、別 issue で対応する（本 issue では判断材料として現状を記載する）

## 完了条件

- 2 関数の docstring が実装の実態と一致していること
- `cargo test --workspace --features source-build` が成功すること（動作変更なしの確認）

## 解決方法

1. `i420_to_i400` の docstring に U / V プレーンの検証要件を明記する
2. `mjpeg_size` の docstring を実装の実態に合わせて修正する
