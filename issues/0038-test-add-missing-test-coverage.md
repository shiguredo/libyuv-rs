# テストカバレッジの拡充: Phase 1 — 欠落ファイルとバリデーションテスト

- Priority: High
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/add-test-coverage
- Polished: 2026-07-08
- Reporter: @voluntas

## 目的

shiguredo-rust 規約に従い、欠落している単体テストファイルを作成し、lib.rs のバリデーション内部関数にテストを追加する。本 issue は Phase 1（欠落ファイル作成 + バリデーションテスト）のみを対象とし、PBT 拡充・fuzz 拡充は別 issue で対応する。

## 優先度根拠

High。以下の深刻な問題がある:

- `tests/test_convert.rs`, `tests/test_planar.rs`, `tests/test_rotate.rs`, `tests/test_scale.rs`, `tests/test_compare.rs` が全欠落（shiguredo-rust 規約違反）
- `src/lib.rs` のバリデーション内部関数 10 個（`require_c_int`, `checked_buf_size`, `validate_*_inner` 全 10 関数）に直接の単体テストが一切ない。これらは全公開 API の入力検証の最前線であり、誤りがあるとバッファオーバーランに直結する

全 5 モジュールの公開関数数: convert: 243, planar: 67, rotate: 20, scale: 18, compare: 9。合計 357。PBT カバーは 24 プロパティ（convert: 5, planar: 8, rotate: 4, scale: 4, compare: 3）。本 issue では欠落ファイルの作成とエラーパス・境界値テストに絞る。

## 現状

### 欠落テストファイル

- `tests/test_convert.rs` — 未作成
- `tests/test_planar.rs` — 未作成
- `tests/test_rotate.rs` — 未作成
- `tests/test_scale.rs` — 未作成
- `tests/test_compare.rs` — 未作成

### バリデーション内部関数 (src/lib.rs)

全 10 関数が private のため、`#[cfg(test)] mod tests` を `src/lib.rs` に追加してテストする必要がある。`tests/` 統合テストからは直接アクセスできない:

1. `require_c_int` (L154)
2. `checked_buf_size` (L162)
3. `validate_yuv_src_inner` (L173)
4. `validate_yuv_dst_inner` (L243)
5. `validate_nv_src_inner` (L313)
6. `validate_nv_dst_inner` (L370)
7. `validate_yuv16_src_inner` (L426)
8. `validate_yuv16_dst_inner` (L496)
9. `validate_nv16_src_inner` (L566)
10. `validate_nv16_dst_inner` (L623)

### 既存の PBT カバレッジ

- `prop_convert.rs`: 5 プロパティ（ARGB↔ABGR, NV12↔I420, コピー 3 種）
- `prop_planar.rs`: 8 プロパティ（コピー, split/merge UV, split/merge RGB, mirror 4 種, mirror_plane）
- `prop_rotate.rs`: 4 プロパティ（I420/ARGB 180°×2, 90°×4）
- `prop_scale.rs`: 4 プロパティ（I420/ARGB/NV12/plane 等倍スケール）
- `prop_compare.rs`: 3 プロパティ（同一画像 PSNR/SSIM/SSE）

## 設計方針

### Phase 1a: 欠落テストファイル作成

各 `tests/test_<module>.rs` に最低限のエラーパス・境界値テストを実装する。PBT でカバーできる正常系のプロパティ検証は追加せず、異常系に絞る:

- `test_convert.rs`: バッファ不足、stride 不足、c_int 範囲超過（I420→ARGB, NV12→I420 等の代表的な変換で）
- `test_planar.rs`: バッファ不足、stride 不足（copy_plane, split_uv_plane 等の代表的な操作で）
- `test_rotate.rs`: バッファ不足、サイズ不一致
- `test_scale.rs`: バッファ不足、dst サイズ不一致
- `test_compare.rs`: バッファ不足、stride 不足（PSNR, SSIM, SSE で）

### Phase 1b: バリデーション内部関数テスト

`src/lib.rs` に `#[cfg(test)] mod tests` を追加し、以下をテストする:

- `require_c_int`: c_int::MAX + 1 で Err、c_int::MAX で Ok
- `checked_buf_size`: stride * height が usize::MAX を超えるケースで Err
- 各 `validate_*_inner`: 正常系 + バッファ不足 + stride 不足 + zero-size

## 完了条件

- `tests/test_convert.rs`, `tests/test_planar.rs`, `tests/test_rotate.rs`, `tests/test_scale.rs`, `tests/test_compare.rs` が作成され、エラーパス・境界値テストが実装されていること
- `src/lib.rs` に `#[cfg(test)] mod tests` が追加され、10 個のバリデーション内部関数のテストが実装されていること
- 全テストの expect メッセージが日本語であること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること
- `CHANGES.md` の `## develop` → `### misc` セクションに `[ADD] 欠落テストファイルを作成しバリデーション内部関数の単体テストを追加する` を追記すること (@voluntas 署名付き)

## 解決方法

### Phase 1a

1. `tests/test_convert.rs` を作成し、代表的な変換関数（I420→ARGB, NV12→I420 等）のエラーパステストを実装する
2. `tests/test_planar.rs` を作成し、代表的な操作（copy_plane, split_uv_plane 等）のエラーパステストを実装する
3. `tests/test_rotate.rs`, `tests/test_scale.rs`, `tests/test_compare.rs` を同様に作成する
4. 各テストの `.expect()` メッセージは日本語で記述する

### Phase 1b

5. `src/lib.rs` の末尾付近に `#[cfg(test)] mod tests { use super::*; ... }` を追加する
6. `require_c_int` の境界値テスト: `c_int::MAX as usize` → Ok, `c_int::MAX as usize + 1` → Err
7. `checked_buf_size` のオーバーフローテスト: `checked_buf_size(usize::MAX, 2, ...)` → Err
8. 各 `validate_*_inner` の基本テスト: 正常系 Ok + バッファ不足 Err + stride 不足 Err
9. CHANGES.md にエントリを追加する

### 補足

本 issue では PBT プロパティの追加（Phase 2）と fuzz target の拡充（Phase 3）は対象外とする。これらは別 issue で対応する。convert.rs の分割（0036）が先に実施された場合は、新しいファイル構成に合わせて `tests/test_convert.rs` のテストを調整する。
