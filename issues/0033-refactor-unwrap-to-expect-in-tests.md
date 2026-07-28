# PBT テスト内の .unwrap() を .expect("MESSAGE") に置換する

- Priority: Medium
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/refactor-unwrap-to-expect-in-tests
- Polished: 2026-07-29
- Reporter:

## 目的

shiguredo-rust 規約「`.unwrap()` ではなく `.expect("MESSAGE")` を使用すること」に違反している全 `.unwrap()` を `.expect()` に置換する。

## 優先度根拠

Medium。規約違反であり動作への影響はない。しかし `.unwrap()` ではパニック時の情報が少なく、テスト失敗時の原因特定に時間がかかる。34 箇所と件数が多く、一括で対応する価値がある。

## 現状

全 PBT テストファイルに計 34 箇所の `.unwrap()` が存在する（`tests/` 配下には 0 件）:

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
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること

## 解決方法

1. 各 PBT ファイルの `.unwrap()` を `.expect("...")` に置換する
2. メッセージはテスト内容を日本語で簡潔に表現する
