# README.md と CHANGES.md の記載をコードの実態に合わせて修正する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/doc-fix-readme-and-changes
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`README.md` と `CHANGES.md` にコードの実態と一致しない記載がある。利用者を誤導するため、実装と突き合わせて修正する。

## 優先度根拠

Medium。

- 存在しない型（P016）の記載は利用者がコンパイル不能なコードを書く原因になる
- 関数数の誤記載は機能の誤解を招く

## 現状

### README.md

- 29 行・270 行: 「YUV / RGB 間のフォーマット変換 (244 関数)」「convert | 244」— 実際は 243 関数（i420 35 / argb 39 / colorspace 33 / high_bitdepth 33 / hardware 13 / jpeg 25 / mjpeg 5 / nv 20 / packed 35 / subsampling 5）
- 103 行: 高ビット深度表の `P010 / P012 / P016 / P210 / P410` — **P016 は存在しない型**（`src/lib.rs` の 2 プレーン 16bit 型は P010 / P012 / P210 / P212 / P410 の 5 種のみ）。実在する **P212 が表から欠落**
- 同表: H010 / H210 / U010 / U210 も欠落
- 56〜105 行: 「対応フォーマット」表に H420 / H422 / H444、U420 / U422 / U444、J400、NV16、MM21、MT2T、YUV24、AYUV、Android420 が欠落（「対応フォーマット」の全称表現と不整合）

### CHANGES.md

- 109 行: 「3-プレーン: I010, I210, I410, I012, I212, I412, H010, H210, H410, U010, U210」— **H410 は存在しない型**（`src/lib.rs` の 3 プレーン 16bit 型は I010 / I012 / H010 / U010 / I210 / I212 / H210 / U210 / I410 / I412）
- 116 行: 8bit パック型リストに RGB24 / RAW が欠落
- 2026.1.0 セクション: エントリが [ADD] → [CHANGE] の順で、shiguredo-changelog 規約（CHANGE → ADD → UPDATE → FIX の順）に違反している

## 設計方針

- README / CHANGES の記載を `src/lib.rs` の型定義と `src/` の pub fn 実数（grep で再計測）に突き合わせて修正する
- 関数数は記載時点で再計測し、正しい数値に更新する
- CHANGES.md の 2026.1.0 セクションの順序は規約に合わせて並べ替える

## 完了条件

- README.md の関数数・対応フォーマット表・高ビット深度表がコードの実態と一致していること
- CHANGES.md の型一覧（H410 等）がコードの実態と一致していること
- CHANGES.md の 2026.1.0 セクションのエントリ順が規約（CHANGE → ADD → UPDATE → FIX）に沿っていること
- 記載と実装の一致を検証する手順（grep 等）がコメントやコミットメッセージで示されていること

## 解決方法

1. `src/lib.rs` の型定義と `src/` の pub fn を再計測する
2. README.md の該当箇所を修正する
3. CHANGES.md の該当箇所（H410・RGB24/RAW・順序）を修正する
