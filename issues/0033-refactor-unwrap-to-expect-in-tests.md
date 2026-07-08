# PBT テスト内の .unwrap() を .expect("MESSAGE") に置換する

- Priority: Medium
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/refactor-unwrap-to-expect-in-tests
- Polished: 2026-07-08
- Reporter:

## 目的

shiguredo-rust 規約「`.unwrap()` ではなく `.expect("MESSAGE")` を使用すること」に違反している全 `.unwrap()` を `.expect()` に置換する。

## 優先度根拠

全 PBT テストファイルに計 34 箇所の `.unwrap()` が存在する:

## 現状

- `pbt/tests/prop_compare.rs:21,39,51` — 3 箇所
- `pbt/tests/prop_convert.rs:19,24,46,59,86,104,124` — 7 箇所
- `pbt/tests/prop_planar.rs:16,34,37,54,58,85,102,122,129,151,162,181,188` — 13 箇所
- `pbt/tests/prop_rotate.rs:31,49,90,117,124,139,142` — 7 箇所
- `pbt/tests/prop_scale.rs:29,47,67,82` — 4 箇所


## 設計方針

全 `.unwrap()` を `.expect("日本語メッセージ")` に置換する。メッセージはテストのログメッセージ規約（日本語）に従う。

例: `i420_psnr(&src, &src, size).unwrap()` → `.expect("同一 I420 画像の PSNR が成功すること")`

## 完了条件

- PBT テストファイル内に `.unwrap()` が存在しないこと
- 全 `.expect()` メッセージが日本語であること
- `cargo test --workspace` が成功すること

## 解決方法

1. 各 PBT ファイルの `.unwrap()` を `.expect("...")` に置換する
2. メッセージはテスト内容を日本語で簡潔に表現する
