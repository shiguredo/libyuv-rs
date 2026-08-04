# convert モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-convert-tests
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/convert/` の公開関数 243 個のうち、正しさを検証するテストが存在するのは 7 関数のみ（`argb_to_abgr` / `abgr_to_argb` / `nv12_to_i420` / `i420_to_nv12` / `i420_copy` / `argb_copy` / `nv12_copy`）。残りの約 200 関数は単体テスト・PBT・fuzz のいずれにも載っておらず、FFI バインディングとしての品質保証が不足している。テストカバレッジを拡充する。

## 優先度根拠

Medium。

- 既知の検証バグ（issue 0044 / 0045 / 0046 / 0047 / 0048 / 0050 の対象関数）にテストが一切存在しないため、修正の回帰検出ができない
- 16bit 系・colorspace 系・jpeg 系・packed 系・hardware 系はパニック安全性（fuzz）すら担保されていない

## 現状

完全に未テストの関数群:

- `colorspace.rs`: H010 / H210 / H420 / H422 / H444 / U010 / U210 / U420 / U422 / U444 系全 33 関数
- `high_bitdepth.rs`: I010 / I012 / I210 / I212 / I410 / I412 / P012 系全 33 関数
- `jpeg.rs`: 全 25 関数
- `packed.rs`: yuy2 / uyvy / rgb565 / argb1555 / argb4444 / p010 / p210 系全 35 関数
- `hardware.rs`: android420 / mm21 / mt2t / ayuv / detile 系全 13 関数
- `argb.rs`: ar30 / ar64 / rgba / bgra / raw 系
- `i420.rs`: i400 系・alpha 系（alpha 系は 0044 / 0050 の対象）
- `nv.rs`: nv12_to_raw / nv12_to_rgb565 / nv12_to_nv24 / nv16_to_nv24 / nv21_to_yuv24 / nv21_copy 等
- `subsampling.rs`: 全関数

## 設計方針

- 正常系は「既知入力に対する期待値」を検証できる ground truth 方式の単体テストを中心にする（単純なラウンドトリップは誤った実装でも通るため、既知ピクセル列の変換結果と期待バイト列の比較を行う）
- エラーパスはバッファ不足・stride 不足・c_int 超過・サイズ 0 の境界値を関数群ごとに代表でカバーする
- テストファイルが肥大化した場合は `tests/test_convert/` 配下にサブモジュール化して分割する
- 対応する fuzz ターゲットの拡充は issue 0056 で対応する（本 issue では単体テスト・PBT に絞る）

## 完了条件

- 上記の未テスト関数群のうち、主要な変換関数（各サブモジュールの代表関数 + 検証ロジックを持つ関数）に正常系・エラーパスのテストが存在すること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. サブモジュールごとにテスト対象関数の優先順位を決める（検証ロジックを持つ関数と既知のバグ対象関数を最優先）
2. ground truth 方式の正常系テストと境界値のエラーパステストを追加する
3. テストファイルの分割（必要に応じて `tests/test_convert/` サブモジュール化）
4. `CHANGES.md` の `### misc` にエントリを追加する
