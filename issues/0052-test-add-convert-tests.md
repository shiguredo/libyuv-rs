# convert モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-convert-tests
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/convert/` の公開関数 243 個（実測）のうち、変換結果の正しさ（期待値）を検証するテストが存在するのは 8 関数のみ（PBT の copy 系 3 関数: `i420_copy` / `argb_copy` / `nv12_copy` は完全一致検証、`mjpeg_size` 1 関数は既知 JPEG の寸法照合、ラウンドトリップ 4 関数: `argb_to_abgr` / `abgr_to_argb` / `nv12_to_i420` / `i420_to_nv12` は恒等変換でも通過するため実質的な正しさは未検証（0055 の改善対象））。なお `mjpeg_to_i420` / `mjpeg_to_nv12` / `mjpeg_to_nv21` / `mjpeg_to_argb` は「出力が非ゼロ」のみの弱い検証（ground truth ではない）であり、正しさの検証としては未達。残りの約 230 関数は変換結果の正しさを検証するテストがなく、FFI バインディングとしての品質保証が不足している。テストカバレッジを拡充する。

## 優先度根拠

Medium。

- 既知の検証バグ（issue 0044 / 0045 / 0046 / 0047 / 0048 / 0050 の対象関数）に正しさを検証するテストが存在しないため、修正の回帰検出ができない
- 16bit 系（high_bitdepth）・colorspace 系・jpeg 系・packed 系・hardware 系は fuzz によるパニック安全性すら担保されていない（fuzz は fuzz_convert.rs の 42 関数 + fuzz_mjpeg.rs の 5 関数のみ到達）

## 現状

変換結果の正しさを検証するテストが存在しない関数群（fuzz によるパニック安全性のみ担保されている関数も含む。完了条件の代表関数選定はサブモジュール全体から行う）:

- `colorspace.rs`: H010 / H210 / H420 / H422 / H444 / U010 / U210 / U420 / U422 / U444 系全 33 関数
- `high_bitdepth.rs`: I010 / I012 / I210 / I212 / I410 / I412 / P012 系全 33 関数
- `jpeg.rs`: 全 25 関数
- `packed.rs`: yuy2 / uyvy / rgb565 / argb1555 / argb4444 / p010 / p210 系全 35 関数
- `hardware.rs`: android420 / mm21 / mt2t / ayuv / detile 系全 13 関数
- `argb.rs`: ar30 / ar64 / rgba / bgra / raw 系、`abgr_to_nv12` / `abgr_to_nv21`
- `i420.rs`: i400 系・alpha 系（alpha 系は 0050 の対象）・`i420_to_i010` / `i420_to_ar30` / `i420_to_ab30`
- `nv.rs`: nv12_to_raw / nv12_to_rgb565 / nv12_to_nv24 / nv16_to_nv24 / nv21_to_yuv24 / nv21_copy 等
- `subsampling.rs`: `i422_copy` / `i444_copy`（他の 3 関数は fuzz でカバー済み）
- `mjpeg.rs`: `mjpeg_to_i420` / `mjpeg_to_nv12` / `mjpeg_to_nv21` / `mjpeg_to_argb` は「出力が非ゼロ」のみの弱い検証（ground truth ではない）のため、正しさの検証としては未達。`mjpeg_size` は寸法照合で検証済み

なお fuzz は `fuzz/fuzz_targets/fuzz_convert.rs` の計 42 関数（i420 / argb / nv / subsampling 系）+ `fuzz_mjpeg.rs` の 5 関数に到達しており、パニック安全性は担保されているが正しさは検証されていない。

## 設計方針

- 正常系は「既知入力に対する期待値」を検証できる ground truth 方式の単体テストを中心にする（単純なラウンドトリップは誤った実装でも通るため、既知ピクセル列の変換結果と期待バイト列の比較を行う）。期待バイト列の由来は、libyuv 本体の unit test の期待値の流用・既知カラーパッチの手計算・独立ビルドでの生成から選ぶ（許容誤差は変換の丸めを考慮して関数ごとに定める）。ground truth を単体テストで行うのは、色変換係数の期待値がプロパティ（ラウンドトリップ等）で表現しにくいロッシー変換が対象。ロスレス変換（copy 系・チャンネル入替等）は PBT のラウンドトリップでカバーする（0055 のスコープ。0055 の完了条件に convert の新規 PBT 追加が明記されていないため、0055 側で対象を明記する必要がある）
- エラーパスはバッファ不足・stride 不足・c_int 超過の境界値を関数群ごとに代表でカバーする（サイズ 0 のテストは 0064 のスコープ。挙動が関数ごとに 3 パターン混在しているため 0052 では扱わない）
- テストファイルが肥大化した場合は `tests/test_convert/` 配下にサブモジュール化して分割する（shiguredo-rust スキルのファイル内 `mod` 分割を検討した上で、規模に応じてディレクトリ分割を選択）
- PBT の改善は issue 0055、fuzz ターゲットの拡充は issue 0056 で対応する（本 issue では単体テストに絞る。0056 の完了条件に convert の fuzz 拡充が含まれていないため、0056 側で convert の対象関数を明記する必要がある）
- 0044〜0050 の対象関数（yuy2_to_y / uyvy_to_y、android420 系、mm21 / mt2t 系、detile 系、p210_to_p410 / nv12_to_nv24 / nv16_to_nv24、alpha 系）の境界値テストは各 bug issue のスコープで、0052 の選定対象外とする。正常系についても同様に各 bug issue のスコープ（0044 / 0045 / 0048 は正常系を自スコープに含む。0046 / 0047 は完了条件に正常系を含まないため、実装時に 0046 / 0047 側の完了条件へ正常系テストを追加する必要がある）。0052 では 0050 が委譲した alpha 系の正常系・バッファ不足テストと、上記以外の関数を対象とする

## 完了条件

- 上記の関数群のうち、サブモジュールごとに代表関数（各サブモジュールの代表的な変換関数 2〜3 個。0044〜0050 の対象関数は設計方針のとおり選定対象外）を選定し、正常系（ground truth）・エラーパス（バッファ不足・stride 不足・c_int 超過）のテストが `tests/test_convert.rs`（分割後は `tests/test_convert/` 配下）に存在すること。ロスレス関数（copy 系等）を選定する場合は正常系は PBT（0055 のスコープ）に委譲し、ground truth はロッシー変換に適用する。対象関数の選定一覧を実装時に issue に追記して確定すること
- 0050 が委譲した alpha 系 7 関数の正常系・バッファ不足テストが追加されていること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加すること

## テスト対象の代表関数（選定一覧）

各サブモジュールの代表関数とテスト方針を定める。ロスレス関数（copy 系・チャンネル入替系）は選定しない（正常系は 0055 の PBT に委譲する方針のため。エラーパスの代表カバーもロッシー変換側で兼ねる）。ground truth の計算は `tests/helpers/convert.rs` の参照実装（libyuv の `row_common.cc` と同一の整数演算を再現）で行う。

- `colorspace.rs`: `h420_to_argb`（BT.709 limited）・`u420_to_argb`（BT.2020 limited）
- `high_bitdepth.rs`: `i010_to_argb`（10bit→ARGB）・`i012_to_i420`（12bit→8bit）
- `jpeg.rs`: `j420_to_argb`（BT.601 full range）・`argb_to_j420`（RGB→BT.601 full range）
- `packed.rs`: `rgb565_to_argb` / `argb_to_rgb565`（ビットパック変換）・`p010_to_nv12`（10bit→8bit）
- `hardware.rs`: `ayuv_to_nv12` / `ayuv_to_nv21`（AYUV→NV 系。android420 / mm21 / mt2t / detile 系は 0045 / 0046 / 0047 のスコープのため選定しない）
- `argb.rs`: `ar30_to_argb` / `argb_to_ar30`（10bit パック変換）・`i420_to_rgba`（色変換 + チャンネル入替）
- `i420.rs`: `i400_to_argb`（グレー→ARGB）・`i420_to_i400`（Y 抽出）・`i420_to_i010`（8bit→10bit）
- `nv.rs`: `nv12_to_raw`（Y 抽出）・`nv21_to_yuv24`（NV21→YUV24）・`i444_to_nv12`（UV パック）
- `subsampling.rs`: `i444_to_rgb24`・`i422_to_rgb24`（色変換。`i422_to_i444` のアップサンプリングは libyuv のスケーリング補間が SIMD 実装に依存し、ground truth をプラットフォーム非依存で固定できないため選定しない）
- `mjpeg.rs`: `mjpeg_to_i420`（既知 JPEG のデコード結果を ground truth として固定）

alpha 系 7 関数（`i420_alpha_to_argb` / `i420_alpha_to_abgr` / `i422_alpha_to_argb` / `i422_alpha_to_abgr` / `i444_alpha_to_argb` / `i444_alpha_to_abgr` / `argb_to_i420_alpha`）は、0050 の委譲どおり正常系・バッファ不足テストを追加する（stride 境界は 0050 で実装済み）。

## 解決方法

1. サブモジュールごとにテスト対象関数の優先順位を決める（各サブモジュールの代表関数 2〜3 個を最優先。0044〜0050 の対象関数の境界値テストは各 bug issue のスコープのため対象外）
2. ground truth 方式の正常系テストと境界値のエラーパステスト（バッファ不足・stride 不足・c_int 超過）を追加する
3. テストファイルの分割（必要に応じて `tests/test_convert/` サブモジュール化）
4. `CHANGES.md` の `### misc` に `[ADD]` エントリを追加する
