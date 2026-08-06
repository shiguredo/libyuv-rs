# packed 画像型のソース側 validate のエラーパステストを追加する

- Priority: Low
- Created: 2026-08-06
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/add-packed-image-validate-tests
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/lib.rs` の `define_packed_image!` マクロが生成するソース側 `validate`（stride 不足・バッファ不足の検査）が、リポジトリ内のどのテストからも一度も実行されていない。エラーパスの回帰検出ができないため、テストを追加する。

## 優先度根拠

Low。

- 検査ロジック自体は `require_c_int` / `checked_buf_size` の共通ヘルパーで構成されており、動作は正しい（バグではない）
- ただし「i420 / nv12 系は src 側の stride 不足テストが存在するのに packed 系だけ未テスト」という非対称が残る

## 現状

- `define_packed_image!` のソース側 `validate`（`src/lib.rs` のマクロ定義内）は、stride が `width * bpp` 未満のとき "stride smaller than width * bpp"、データ長が `stride * height` 未満のとき "source buffer too small" を返す
- この 2 つのエラーパスを実行するテストが存在しない（`src/lib.rs` の単体テストは yuv / nv 系のヘルパーのみ対象）
- 比較対象として、`tests/test_convert.rs` の `i420_to_argb_stride_too_small` / `nv12_to_i420_stride_too_small` は src 側の検証をテストしている

## 設計方針

既存の src 側エラーパステスト（`i420_to_argb_stride_too_small` 等）と同じパターンで、packed 系の代表型（`Yuy2Image` / `UyvyImage`）の stride 不足・バッファ不足テストを `tests/test_convert.rs` に追加する。

## 完了条件

- `Yuy2Image` / `UyvyImage` のソース側 validate のエラーパス（stride 不足・バッファ不足）テストが `tests/test_convert.rs` に追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加すること

## 解決方法

1. `tests/test_convert.rs` に `Yuy2Image` / `UyvyImage` の src 側 stride 不足・バッファ不足テストを追加する
2. `CHANGES.md` の `### misc` に `[ADD]` エントリを追加する
