# p210_to_p410 / nv12_to_nv24 / nv16_to_nv24 の手書き検証が不十分で領域外アクセスのリスクがある

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-nv24-p410-handwritten-validation
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/convert/packed.rs` の `p210_to_p410` と `src/convert/nv.rs` の `nv12_to_nv24` / `nv16_to_nv24` は、画像型の生成済み `validate` を使わずに手書き検証を再実装しており、`require_c_int` 欠落・非チェック乗算・`stride >= width` チェック欠落がある。標準検証パターン（または生成済み `validate`）に置き換えて領域外アクセスのリスクを除去する。

## 優先度根拠

High。

- リリースビルドで `stride * size.height` が wrap して検証をすり抜けると OOB アクセスになる
- `as c_int` の切り詰めで巨大な stride が負値に化けて渡る
- 各画像型は正しい検証を持つ `validate` を生成済みなのに使われていない（`P210Image` は `define_nv_image16!(1, 2)`、`Nv16Image` は `define_nv_image!(1, 2)`、`Nv24ImageMut` は `define_nv_image!(1, 1)`）

## 現状

- `p210_to_p410`（`src/convert/packed.rs`）: `src.y.len() < src.y_stride * size.height` と `src.uv.len() < src.uv_stride * size.height` の素の乗算のみ。`require_c_int` なし、`stride >= width` チェックなし
- `nv12_to_nv24`（`src/convert/nv.rs`）: `dst.y.len() < dst.y_stride * size.height` 等の素の乗算のみ。dst 側（`Nv24ImageMut` は uv 高さ = height を検証できる）に `validate` を使っていない
- `nv16_to_nv24`（`src/convert/nv.rs`）: src / dst とも手書き検証。`Nv16Image::validate` はどこからも呼ばれないデッドコードになっている
- `src/convert/nv.rs` のコメントは存在しない `validate_biplanar_src/dst` という名前を引用しており、実在しない関数名の stale コメントになっている

## 設計方針

手書き検証を標準パターンに置き換える。画像型の `validate` が正しい除数で検証できる場合はそれを使う:

- `p210_to_p410`: `src.validate(size, ...)`（P210Image は uv 高さ = height を正しく検証する）に置き換え
- `nv12_to_nv24`: src は `Nv12Image::validate`、dst は `Nv24ImageMut::validate`（uv 高さ = height）を使用
- `nv16_to_nv24`: src は `Nv16Image::validate`、dst は `Nv24ImageMut::validate` を使用
- 生成済み `validate` では検証できない特殊ケースのみ手書き検証を残し、その場合は `require_c_int` + `checked_buf_size` + `stride >= width` を完全に実装する
- stale コメント（`validate_biplanar_*` の言及）を削除または実在名に修正する

## 完了条件

- 3 関数の検証が `require_c_int` / `stride >= width` / オーバーフロー安全なサイズ計算をすべて満たすこと
- `Nv16Image::validate` / `P210Image::validate` 等のデッド `validate` が解消されること（使用されるか、使用されない場合は `#[allow(dead_code)]` 付きのまま残さないこと）
- stale コメント（`validate_biplanar_*` 言及）が削除されていること
- `tests/test_convert.rs` に 3 関数の境界値テスト（stride 不足・c_int 超過・バッファ不足）が追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. 3 関数の検証を生成済み `validate` または標準パターンに置き換える
2. stale コメントを削除する
3. `tests/test_convert.rs` にテストを追加する
4. `CHANGES.md` に `[FIX]` エントリを追加する
