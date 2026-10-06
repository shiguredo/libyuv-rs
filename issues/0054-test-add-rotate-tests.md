# rotate モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-rotate-tests
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/rotate.rs` の公開関数 20 個のうち、変換結果の正しさ（期待値）を検証するテストが存在するのは 3 関数のみ（`i420_rotate` / `argb_rotate` / `rotate_plane`。しかもラウンドトリップのみで、恒等変換でも通過するため実質的な正しさは未検証（0055 の改善対象））。残りの 17 関数は正しさを検証するテストがなく、エラーパス（`dst_size` 不一致・stride 不足・バッファ不足）も未検証の関数が多い。テストカバレッジを拡充する。

## 優先度根拠

Medium。

- エラーパス（`dst_size` 不一致・stride 不足・バッファ不足）が未検証の関数が多い
- 回転は 90 / 270 度が転置ベース、180 度が一時行バッファ使用という特殊な実装を持つため、テストで守られていない

## 現状

正しさ検証は `i420_rotate` / `argb_rotate` / `rotate_plane` のみ（ラウンドトリップ。0055 の改善対象）。未テストは以下 17 関数:

- `rotate_plane_90/180/270` / `transpose_plane` / `split_rotate_uv` 系 / `split_transpose_uv`
- `i010_rotate` / `i210_rotate` / `i410_rotate` / `i422_rotate` / `i444_rotate`
- `nv12_to_i420_rotate` / `android420_to_i420_rotate` / `rotate_plane_16`

rotate 固有の `dst_size` 不一致エラーパス（`RotationMode::output_size` との照合）も未検証（なお `dst_size` パラメータは mode 付き関数（`rotate_plane` 等 12 箇所）にのみ存在し、固定角度系（`rotate_plane_90` 等）にはない）。

## 設計方針

- 正常系は「既知入力に対する期待値」を検証できる ground truth 方式の単体テストを中心にする（単純なラウンドトリップは誤った実装でも通るため、既知ピクセル列の変換結果と期待バイト列の比較を行う）。期待バイト列の由来は、libyuv 本体の unit test の期待値の流用・既知カラーパッチの手計算・独立ビルドでの生成から選ぶ
- rotate は ground truth 方式（既知ピクセル列を回転した期待値との比較）で検証し、`dst_size` 不一致のエラーパスを検証する。ラウンドトリップ（90 度 4 回・180 度 2 回）は既存 PBT にあり、270 度の独立した PBT 検証は 0055 の改善対象のため、本 issue ではラウンドトリップを再実装しない（本 issue の 270 度 ground truth とは別。mode 付き関数の全モード ground truth は本 issue で行う）
- 正常系の検証対象は、未テスト 17 関数のほか、既存のラウンドトリップのみで正しさ未検証の `i420_rotate` / `argb_rotate` / `rotate_plane` を含む（0055 は PBT の性質テストのみで ground truth は含まないため、値検証は本 issue のスコープ）
- PBT の改善は issue 0055、fuzz ターゲットの拡充は issue 0056 で対応する（本 issue では単体テストに絞る。0056 の完了条件に rotate 全 20 関数の fuzz 追加が含まれているため、rotate の fuzz は 0056 のスコープ）
- サイズ 0 のテストは 0064 のスコープ（挙動が関数ごとに 3 パターン混在しているため本 issue では扱わない）

## 完了条件

- 17 関数に正常系・エラーパスのテストが存在すること（テスト配置先は `tests/test_rotate.rs`。対象関数の選定一覧を実装時に issue に追記して確定すること。エラーパスはバッファ不足・stride 不足・`dst_size` 不一致を対象とし、`require_c_int` のみで実用上テスト不能の c_int 超過は含めない。テストファイルはヘッダの「正常系のプロパティ検証は PBT でカバーする」を ground truth テスト追加に合わせて更新すること）
- mode 付き関数（`i010_rotate` / `i210_rotate` / `i410_rotate` / `android420_to_i420_rotate` / `nv12_to_i420_rotate` / `i422_rotate` / `i444_rotate` / `rotate_plane_16` / `split_rotate_uv`）に `RotationMode` 全モード（None / 90 / 180 / 270）の ground truth テストが存在すること（既存テストのある `i420_rotate` / `argb_rotate` / `rotate_plane` の全モード ground truth も含む）
- rotate の `dst_size` 不一致エラーパスがテストされていること（mode 付き関数）
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加すること

## 解決方法

1. ground truth 方式の正常系テストとエラーパステスト（バッファ不足・stride 不足・`dst_size` 不一致）を追加する
2. mode 付き rotate 関数の `RotationMode` 全モードを ground truth でカバーする（ラウンドトリップではなく）
3. `CHANGES.md` の `### misc` に `[ADD]` エントリを追加する
