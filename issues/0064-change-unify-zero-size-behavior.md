# ゼロサイズ入力（width == 0 / height == 0）の挙動を統一する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/change-unify-zero-size-behavior
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

画像サイズ 0（`width == 0` / `height == 0`）の入力に対する挙動が関数ごとに異なり、統一ポリシーがない。ゼロサイズ入力の扱いをモジュール全体で統一する。

## 優先度根拠

Medium。

- 同じ 0 サイズ入力に対して「エラー」になる関数と「無言の成功」（`Ok(())`）を返す関数が混在し、利用者を混乱させる
- 一部の関数では 0 サイズが原因の panic（issue 0026）や意味のない値（`Ok(0)`）が返る

## 現状

ゼロサイズ入力の挙動は 3 パターンに分かれている:

1. **明示的に Err**: `src/compare.rs` の `i420_psnr` / `calc_frame_psnr`（「意味のない値（kMaxPsnr）が正常値として返るためエラーにする」）
2. **C 側の -1 が Error になる**: scale / rotate の int 系（C が `width <= 0 || height == 0` で -1 を返し、`Error::check` がエラー化する）。ただし reason なし
3. **無言の成功（Ok(())）**: C が void の関数群（`rotate_plane_90/180/270` / `transpose_plane` / `split_rotate_uv_90/180/270` / `split_transpose_uv`）と、`src/convert/hardware.rs` の `detile_plane` 系（「C 実装の早期 return と同一セマンティクスで no-op」と明示）

さらに `src/planar.rs` の `argb_detect` は height=0 で `Ok(0)`（「アルファなし」と区別不能）を返す。

## 設計方針

モジュール全体で「ゼロサイズ入力は `Err`」に統一する（エラーを返せない C の void 関数は、Rust 側の検証で弾く）。ただし:

- `detile_plane` 系の「0 サイズは no-op」は明示的な設計判断として残す場合は、docstring に明記して例外であることを示す
- どの関数で例外を許容するかは、既存のコメント（「C 実装の早期 return と同一セマンティクス」）と照合して実装時に決定する
- エラーメッセージは統一形式（`"width and height must be greater than 0"`）にする

## 完了条件

- `width == 0` / `height == 0` の入力に対する挙動がモジュール全体で統一されていること（例外があれば docstring に明記）
- 挙動変更した関数の `CHANGES.md` エントリ（`[CHANGE]` または `[FIX]`）が追加されていること
- 境界値テスト（0 サイズ）が追加されていること
- `cargo test --workspace --features source-build` が成功すること

## 解決方法

1. ゼロサイズ入力を返す全公開関数の挙動を調査し、統一ポリシー（デフォルト: Err）を決める
2. 各関数の検証にゼロサイズチェックを追加する（void 系は Rust 側で弾く）
3. docstring に挙動を明記する
4. テストを追加し、`CHANGES.md` にエントリを追加する
