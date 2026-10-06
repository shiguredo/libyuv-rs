# p210_to_p410 / nv12_to_nv24 / nv16_to_nv24 の手書き検証が不十分で領域外アクセスのリスクがある

- Priority: High
- Created: 2026-08-04
- Completed: 2026-08-06
- Model: DeepSeek V4 Flash
- Branch: feature/fix-nv24-p410-handwritten-validation
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/convert/packed.rs` の `p210_to_p410` と `src/convert/nv.rs` の `nv12_to_nv24` / `nv16_to_nv24` は、画像型の生成済み `validate` を使わずに手書き検証を再実装しており、`require_c_int` 欠落・非チェック乗算・最小ストライドチェック欠落がある（`nv12_to_nv24` は src 側のみ生成済み `validate` を使用済みで、手書きなのは dst 側）。生成済み `validate` に置き換えて領域外アクセスのリスクを除去する。

## 優先度根拠

High。

- 最小ストライドチェック欠落により、通常サイズの入力でも領域外アクセスが発生する（例: p210_to_p410 で width=100、y_stride=50、height=10、y.len()=500 のとき検証は `500 < 50 * 10` で通過するが、C 側は 1 行 100 要素 × 10 行 = 1000 要素を読み出す。デバッグビルド・リリースビルドを問わず発生）
- リリースビルドで `stride * size.height` が wrap して検証をすり抜けると OOB アクセスになる
- `as c_int` の切り詰めで巨大な stride が負値に化けて渡る
- 各画像型は正しい検証を持つ `validate` を生成済みなのに、対象 3 関数で使われていない（`P210Image` は `define_nv_image16!(1, 2)`、`Nv16Image` は `define_nv_image!(1, 2)`、`Nv24ImageMut` は `define_nv_image!(1, 1)`。`Nv24ImageMut::validate` 自体は `nv24_scale` で使用済みだが、nv12_to_nv24 / nv16_to_nv24 の dst 検証には使われていない）

## 現状

- `p210_to_p410`（`src/convert/packed.rs`）: `src.y.len() < src.y_stride * size.height` と `src.uv.len() < src.uv_stride * size.height` の素の乗算のみ。`require_c_int` なし、最小ストライドチェックなし
- `nv12_to_nv24`（`src/convert/nv.rs`）: src は `Nv12Image::validate` を使用済み。dst 側（`Nv24ImageMut` は uv 高さ = height を検証できる）に `validate` を使っていない
- `nv16_to_nv24`（`src/convert/nv.rs`）: src / dst とも手書き検証。`Nv16Image::validate` はどこからも呼ばれないデッドコードになっている（`Nv16ImageMut::validate` も同様にデッド）
- `src/convert/nv.rs` のコメントは存在しない `validate_biplanar_src/dst` という名前を引用しており、実在しない関数名の stale コメントになっている（関数本体の「個別に検証する」コメントも置換後は stale になる）

## 設計方針

手書き検証を生成済み `validate` に置き換える（対象 3 関数の入力はすべて生成済み `validate` で完全に検証可能であり、手書き検証を残す特殊ケースは存在しない）:

- `p210_to_p410`: `src.validate(size, ...)`（P210Image は uv 高さ = height を正しく検証する）に置き換え
- `nv12_to_nv24`: dst を `Nv24ImageMut::validate`（uv 高さ = height）に置き換え（src は現行のまま）
- `nv16_to_nv24`: src は `Nv16Image::validate`、dst は `Nv24ImageMut::validate` に置き換え
- stale コメント（`validate_biplanar_*` の言及）と関数本体の stale コメントを削除する（対応する実在の関数はないため「実在名に修正」は選択しない）
- ゼロサイズ入力（width == 0 / height == 0）は置換前後とも C 実装が `-1` を返し `Err` になる挙動が不変である（0064 のゼロサイズ統一方針と整合）
- `Nv24ImageMut::validate` は `nv24_scale`（`src/scale.rs`）で既に使用済み。本 issue で新たに使用されるようになるのは `Nv16Image::validate` / `P210Image::validate` の 2 つ。修正後もデッドのまま残る validate（`Nv16ImageMut` / `P410Image` / `P212Image` 等）の `#[allow(dead_code)]` 整理は 0061 / 0057 のスコープとする

## 完了条件

- 3 関数の検証が `require_c_int` / 最小ストライドチェック / オーバーフロー安全なサイズ計算をすべて満たすこと
- `Nv16Image::validate` / `P210Image::validate` が本 issue の関数から実際に呼び出されること（修正後もデッドのまま残る validate の `#[allow(dead_code)]` 整理は 0061 / 0057 のスコープ）
- stale コメント（`validate_biplanar_*` 言及）と関数本体の stale コメントが削除されていること
- ゼロサイズ入力（width == 0 / height == 0）で `Err` を返すこと（置換前後で挙動不変）
- `tests/test_convert.rs` に 3 関数の境界値テスト（stride 不足・stride の c_int 超過・バッファ不足・正常系）が追加されていること（0052 で計画される convert 系テスト拡充との重複分は本 issue でカバーする）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. 3 関数の検証を生成済み `validate` に置き換える
2. stale コメント（doc コメントと関数本体コメント）を削除する
3. `tests/test_convert.rs` にテスト（stride 不足・stride の c_int 超過・バッファ不足・正常系）を追加する
4. `CHANGES.md` に `[FIX]` エントリを追加する

## 実装内容

- `p210_to_p410`（`src/convert/packed.rs`）: `src.validate` / `dst.validate` に置き換え
- `nv12_to_nv24`（`src/convert/nv.rs`）: dst を `Nv24ImageMut::validate` に置き換え（src は従来どおり `Nv12Image::validate`）
- `nv16_to_nv24`（`src/convert/nv.rs`）: `src.validate`（`Nv16Image`）/ `dst.validate`（`Nv24ImageMut`）に置き換え
- `validate_biplanar_src/dst` の stale コメントと関数本体の stale コメントを削除
- `tests/test_convert.rs` に 17 本のテスト（1 シナリオ 1 テスト）を追加。stride 不足（Y / UV）・stride の c_int 超過・バッファ不足（Y / UV、src / dst）・正常系を 3 関数で網羅
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加

## 検証

- `cargo fmt --all --check`: 成功
- `cargo clippy --workspace --features source-build -- -D warnings`: 成功
- `cargo test --workspace --features source-build`: 全 63 passed
- PR #24 を squash merge（CI 全 5 ジョブ pass）
