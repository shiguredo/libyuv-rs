# validate_yuv/nv 系の src / dst 検証関数 8 ペアの重複を統合する

- Priority: Low
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-unify-validation-helpers
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/lib.rs` のバッファ検証ヘルパー 8 個（`validate_yuv_src_inner` / `validate_yuv_dst_inner` / `validate_nv_src_inner` / `validate_nv_dst_inner` / `validate_yuv16_src_inner` / `validate_yuv16_dst_inner` / `validate_nv16_src_inner` / `validate_nv16_dst_inner`）が、エラーメッセージの "source" / "destination" 以外は 100% 同一の実装を 4 組重複させている。src / dst をパラメータで切り替える 1 関数に統合する。

## 優先度根拠

Low。

- 動作は正しい（重複は冗長性の問題）
- ただし「チェックの書き忘れ」（issue 0048 の手書き検証のような）の温床になる構造は改善する価値がある

## 現状

- `validate_yuv_src_inner`（`src/lib.rs`）と `validate_yuv_dst_inner` は「source Y buffer too small」/「destination Y buffer too small」等のメッセージ語句だけが異なる
- 同様のペアが NV / YUV16 / NV16 にも存在する
- `define_y_image!` の validate だけはヘルパーを経由せずインライン実装されており、他のマクロとの一貫性がない

## 設計方針

- src / dst を `is_src: bool` や「source / destination」のプレフィックス文字列で切り替える 1 関数に統合する（例: `validate_yuv_inner(..., is_src: bool, function, ...)`）
- エラーメッセージは現在の語句（"source Y buffer too small" / "destination Y buffer too small"）を維持する
- `define_y_image!` のインライン検証もヘルパー（例: `validate_y_src_inner` / `validate_y_dst_inner`）に委譲して統一する
- 統合後のテストは既存の `src/lib.rs` の `#[cfg(test)] mod tests` と issue 0052〜0054 のテストでカバーする

## 完了条件

- src / dst のペアが 1 関数に統合されていること
- `define_y_image!` の validate がヘルパー委譲になっていること
- `packed.rs` の `yuy2_to_y` / `uyvy_to_y` のインライン検証ブロック（`require_c_int` / `stride >= width` / `checked_buf_size` の手書き実装）も統合対象に含まれていること（これらの関数は本 issue のスコープ決定後に追加されたため、追記でスコープを拡大する）
- `hardware.rs` の `android420_to_argb` / `android420_to_abgr` / `android420_to_i420` と `rotate.rs` の `android420_to_i420_rotate` のインターリーブ検証ブロック（`pixel_stride_uv` の値検証と `pixel_stride_uv == 2` の U/V ストライド・バッファ検証）も統合対象に含まれていること（4 関数で約 50 行 × 4 が重複しており、本 issue のスコープ決定後に追加された）
- エラーメッセージが変更前と同一であること
- `cargo test --workspace --features source-build` が成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. 8 個のヘルパーを 4 個（または 2 個）に統合する
2. `define_y_image!` のインライン検証をヘルパーに委譲する
3. 呼び出し側（マクロの `validate` メソッド）を更新する
4. テストで回帰がないことを確認し、`CHANGES.md` の `### misc` にエントリを追加する
