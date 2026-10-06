# test_mjpeg.rs の expect メッセージを日本語化する

- Priority: Medium
- Created: 2026-07-08
- Completed: 2026-07-30
- Model: DeepSeek V4 Pro
- Branch: feature/fix-test-mjpeg-japanese-messages
- Polished: 2026-07-29
- Reporter:

## 目的

`tests/test_mjpeg.rs` 内の `.expect()` メッセージが英語で書かれており、AGENTS.md の「テストのログメッセージは全て日本語にすること」に違反しているため修正する。

## 優先度根拠

Medium。規約違反であり機械的置換で対応可能。動作への影響はない。

## 現状

以下の 5 箇所で英語の expect メッセージが使用されている:

- `tests/test_mjpeg.rs:63` — `.expect("mjpeg_size should succeed for known good JPEG")`
- `tests/test_mjpeg.rs:92` — `.expect("mjpeg_to_i420 succeeds for known good JPEG")`
- `tests/test_mjpeg.rs:112` — `.expect("mjpeg_to_nv12 succeeds for known good JPEG")`
- `tests/test_mjpeg.rs:130` — `.expect("mjpeg_to_nv21 succeeds for known good JPEG")`
- `tests/test_mjpeg.rs:146` — `.expect("mjpeg_to_argb succeeds for known good JPEG")`

## 設計方針

各メッセージを日本語に翻訳する。内容を正確に反映しつつ、簡潔な表現とする。

## 完了条件

- `tests/test_mjpeg.rs` 内の全 `.expect()` メッセージが日本語であること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること

## 解決方法

5 箇所の英語 expect メッセージを日本語に置換した。cargo fmt により行折り返しが適用された。
