# planar モジュールのテストカバレッジを拡充する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-add-planar-tests
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/planar.rs` の公開関数 67 個（実測）のうち、変換結果の正しさ（期待値）を検証するテストが存在するのは PBT の 10 関数のみ（`copy_plane` / `split_uv_plane` / `merge_uv_plane` / `split_rgb_plane` / `merge_rgb_plane` / `i420_mirror` / `argb_mirror` / `nv12_mirror` / `rgb24_mirror` / `mirror_plane`）。うち 9 関数はラウンドトリップのみで、恒等変換でも通過するため実質的な正しさは未検証（0055 の改善対象）。単体テストは `copy_plane` / `split_uv_plane` のエラーパスのみ。fuzz は `fuzz_planar.rs` の 23 関数に到達しているが、16bit 系・ARGB 加工系の大半は fuzz 未到達。残りの約 50 関数は変換結果の正しさを検証するテストがなく、既知の検証バグ（issue 0042 / 0049 の対象関数）も含む。テストカバレッジを拡充する。

## 優先度根拠

Medium。

- 既知の検証バグ（`half_float_plane` / `depth` 未検証）に正しさを検証するテストが存在しないため、修正の回帰検出ができない
- 16bit 系・ARGB 加工系の大半（argb_blur / argb_quantize / sobel 系 / color_table 系等）は fuzz によるパニック安全性すら担保されていない

## 現状

変換結果の正しさを検証するテストが存在しない関数群（fuzz によるパニック安全性のみ担保されている関数も含む。完了条件の代表関数選定はサブモジュール全体から行う）:

- `i400_copy` / `i420_interpolate` / `argb_interpolate` / `interpolate_plane_16`
- `argb_sobel` / `argb_sobel_to_plane` / `argb_sobel_xy`
- `argb_color_matrix` / `rgb_color_matrix` / `argb_polynomial`
- `argb_add` / `argb_subtract` / `argb_multiply`
- `argb_blur` / `argb_compute_cumulative_sum` / `argb_quantize`
- `argb_*_color_table` 系（`argb_luma_color_table` / `argb_color_table` / `rgb_color_table`）/ `argb_shuffle` / `argb_extract_alpha`
- `argb_copy_alpha` / `argb_copy_y_to_alpha` / `argb_detect`
- `copy_plane_16` / `merge_uv_plane_16` / `split_uv_plane_16`
- `merge_argb_plane` / `split_argb_plane` / `merge_ar64_plane`
- `merge_xr30_plane` / `merge_argb16_to_8_plane` / `mirror_uv_plane`
- `convert_16_to_8_plane` / `convert_8_to_16_plane` / `convert_to_lsb_plane_16` / `convert_to_msb_plane_16`
- `half_float_plane` / `half_merge_uv_plane` / `byte_to_float` / `gauss_plane_f32`
- `set_plane` / `i420_rect` / `argb_rect`（fuzz はカバー済み。エラーパス未検証）
- `swap_uv_plane` / `i420_blend` / `argb_blend` / `blend_plane` / `interpolate_plane` / `argb_attenuate` / `argb_unattenuate` / `argb_shade` / `argb_gray` / `argb_sepia`（fuzz はカバー済み。正しさ未検証）
- `i400_mirror` / `argb_gray_to` / `convert_8_to_8_plane`（単体テスト・PBT・fuzz のいずれにも未搭載）

なお fuzz は `fuzz/fuzz_targets/fuzz_planar.rs` の 23 関数に到達しており、パニック安全性は担保されているが正しさは検証されていない。

## 設計方針

- 正常系は「既知入力に対する期待値」を検証できる ground truth 方式の単体テストを中心にする（単純なラウンドトリップは誤った実装でも通るため、既知ピクセル列の変換結果と期待バイト列の比較を行う）。期待バイト列の由来は、libyuv 本体の unit test の期待値の流用・既知カラーパッチの手計算・独立ビルドでの生成から選ぶ（許容誤差は変換の丸めを考慮して関数ごとに定める）。ground truth を単体テストで行うのは、色変換係数の期待値がプロパティ（ラウンドトリップ等）で表現しにくいロッシー変換が対象。ロスレス変換（copy 系・チャンネル入替等）は PBT のラウンドトリップでカバーする（0055 のスコープ。0055 の完了条件に planar の新規 PBT 追加が明記されていないため、0055 側で対象を明記する必要がある）
- エラーパスはバッファ不足・stride 不足・c_int 超過の境界値を関数群ごとに代表でカバーする（代表でカバーする対象の例示であり、関数固有のエラーパス（例: `i420_rect` / `argb_rect` の矩形境界外、`argb_blur` の cumsum 不足）も代表選定時の対象に含める。サイズ 0 のテストは 0064 のスコープ。挙動が関数ごとに 3 パターン混在しているため 0053 では扱わない）
- `src/planar.rs` が 4,437 行と巨大なため、テストファイルは機能別サブモジュール（copy / uv / argb / mirror / blend / interpolate / 16bit / float 等。mirror・blend・interpolate・rect 系を含む全機能グループをカバーするように定義し、対象関数の振り分けは実装時に issue に追記して確定する）で分割する（shiguredo-rust スキルのファイル内 `mod` 分割を検討した上で、規模に応じて `tests/test_planar/` ディレクトリ分割を選択。0042 は「0053 で分割された場合は `tests/test_planar/` 配下に」と条件付きで参照している。0049 は分割への言及がないため、分割する場合は 0049 側へ言及追加が必要）
- PBT の改善は issue 0055、fuzz ターゲットの拡充は issue 0056 で対応する（本 issue では単体テストに絞る。0056 の完了条件に planar の fuzz 拡充が含まれていないため、0056 側で planar の対象関数を明記する必要がある）
- 0042 / 0049 の対象関数（half_float_plane、depth 系 7 関数）の境界値テストは各 bug issue のスコープで、0053 の選定対象外とする。0042 はエラーパス・境界値と PBT を自スコープに含む。0049 は境界値を自スコープに含み、正常系・その他のエラーパスの網羅は 0053 のスコープと委譲している。0053 では 0049 が委譲した depth 系の正常系・エラーパス網羅と、上記以外の関数を対象とする

## 完了条件

- 上記の関数群のうち、機能別サブモジュールごとに代表関数（各サブモジュールの代表的な変換関数 2〜3 個。0042 / 0049 の対象関数は設計方針のとおり代表関数の選定対象外。ただしテスト追加自体は対象。0053 では 0049 が委譲した depth 系の正常系・エラーパス網羅を追加する）を選定し、正常系（ground truth）・エラーパス（バッファ不足・stride 不足・c_int 超過・関数固有のエラーパス）のテストが `tests/test_planar.rs`（分割後は `tests/test_planar/` 配下）に存在すること。ロスレス関数（copy 系等）を選定する場合は正常系は PBT（0055 のスコープ）に委譲し、ground truth はロッシー変換に適用する。対象関数の選定一覧を実装時に issue に追記して確定すること
- 0049 が委譲した depth 系 7 関数の正常系・エラーパス網羅テストが追加されていること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加すること

## 解決方法

1. 機能別サブモジュールごとにテスト対象関数の優先順位を決める（各サブモジュールの代表関数 2〜3 個を最優先。0042 / 0049 の対象関数の境界値テストは各 bug issue のスコープのため対象外）
2. ground truth 方式の正常系テストと境界値のエラーパステスト（バッファ不足・stride 不足・c_int 超過）を追加する
3. 必要に応じて `tests/test_planar/` サブモジュール化で分割する
4. `CHANGES.md` の `### misc` に `[ADD]` エントリを追加する
