# scale モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-scale-tests
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/scale.rs` の公開関数 18 個のうち、変換結果の正しさ（期待値）を検証するテストが存在するのは 4 関数のみ（`i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane`。しかも PBT は `FilterMode::None` 等倍のみで、恒等変換でも通過するため実質的な正しさは未検証）。残りの 14 関数は正しさを検証するテストがなく、エラーパス（stride 不足・バッファ不足）も未検証の関数が多い。テストカバレッジを拡充する。

## 優先度根拠

Medium。

- 正しさ検証（ground truth）が存在しない関数が 14 個あり、スケーリングの品質保証が不足している
- `FilterMode`（None / Linear / Bilinear / Box）のうち Linear / Bilinear / Box のロッシー変換は期待値の検証がない

## 現状

正しさ検証は `i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane` のみ（しかも PBT は `FilterMode::None` 等倍のみ）。未テストは以下 14 関数:

- `i422_scale` / `i444_scale` / `i420_scale_12` / `i420_scale_16` / `i422_scale_12` / `i422_scale_16` / `i444_scale_12` / `i444_scale_16`
- `scale_plane_12` / `scale_plane_16` / `nv24_scale` / `uv_scale` / `uv_scale_16` / `argb_scale_clip`

なお fuzz は `fuzz/fuzz_targets/fuzz_scale.rs` に到達しており、パニック安全性は担保されているが正しさは検証されていない。0056 の完了条件に「scale の全 18 関数が fuzz ターゲットから呼ばれること」が含まれているため、fuzz の拡充は 0056 のスコープ。

## 設計方針

- 正常系は「既知入力に対する期待値」を検証できる ground truth 方式の単体テストを中心にする（単純なラウンドトリップは誤った実装でも通るため、既知ピクセル列の変換結果と期待バイト列の比較を行う）。期待バイト列の由来は、libyuv 本体の unit test の期待値の流用・既知カラーパッチの手計算・独立ビルドでの生成から選ぶ（許容誤差は変換の丸めを考慮して関数ごとに定める。Linear / Bilinear / Box はロッシー変換のため許容誤差の設定が必須）
- 正常系の検証対象は、未テスト 14 関数のほか、既存の等倍のみで正しさ未検証の `i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane` を含む（0055 は PBT の性質テストのみで ground truth は含まないため、値検証は本 issue のスコープ）
- 等倍以外（ダウンスケール / アップスケール）の正常系を ground truth で検証する。`FilterMode` 全モード・等倍以外の性質テストは 0055（PBT）のスコープと調整する
- PBT の改善は issue 0055、fuzz ターゲットの拡充は issue 0056 で対応する（本 issue では単体テストに絞る）
- サイズ 0 のテストは 0064 のスコープ（挙動が関数ごとに 3 パターン混在しているため本 issue では扱わない）

## 完了条件

- 14 関数に正常系・エラーパスのテストが存在すること（テスト配置先は `tests/test_scale.rs`。対象関数の選定一覧を実装時に issue に追記して確定すること。エラーパスはバッファ不足・stride 不足を対象とし、`require_c_int` のみで実用上テスト不能の c_int 超過は含めない。テストファイルはヘッダの「正常系のプロパティ検証は PBT でカバーする」を ground truth テスト追加に合わせて更新すること）
- 既存の等倍のみで正しさ未検証の 4 関数（`i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane`）に等倍以外の ground truth テストが存在すること（`FilterMode` は None / Linear / Bilinear / Box から代表モードを選定し、選定一覧に追記して確定する）
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加すること

## 解決方法

1. テスト対象関数の優先順位を決める（代表的な変換関数を最優先）
2. ground truth 方式の正常系テスト（等倍以外・代表 `FilterMode`）とエラーパステスト（バッファ不足・stride 不足）を追加する
3. `CHANGES.md` の `### misc` に `[ADD]` エントリを追加する
