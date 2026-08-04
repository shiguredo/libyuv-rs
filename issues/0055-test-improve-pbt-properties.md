# PBT の検証力を向上させる（等倍限定・対合性限定からの脱却）

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-improve-pbt-properties
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`pbt/` の PBT が「等倍スケールのみ」「対合性（2 回 / 4 回で元に戻る）のみ」の検証に偏っており、誤った実装でも通過してしまう。PBT で検証できる性質を最大限検証するように改善する。

## 優先度根拠

Medium。

- `prop_scale.rs` の全テストは `FilterMode::None` の等倍のみ。libyuv は等倍時に `CopyPlane` へ短絡するため（`scale.cc` の `ScalePlane`）、スケールカーネル（Down2 / Bilinear 等）が一切検証されていない
- ラウンドトリップ（往復一致のみ）は、両方向の変換が互いに逆関数（または両方向とも恒等）であれば誤った実装でも通過する

## 現状

- `pbt/tests/prop_scale.rs`: 4 テストすべて `FilterMode::None` の等倍（実質コピー）のみ。`Linear` / `Bilinear` / `Box` の正しさは未検証（加えて libyuv の `ScaleFilterReduce` は縮小比 1/2 以上で Box を Bilinear に、縦等倍・1/3 縮小で Bilinear を Linear に還元するため、各カーネルを実際に通す縮小比の選定が必要）
- `pbt/tests/prop_rotate.rs`: `Rotate180`（2 回で戻る）と `Rotate90`（4 回で戻る）のみ。`Rotate270` は 90 の逆回転で独立した検証にならない
- `pbt/tests/prop_compare.rs`: 同一画像の `psnr >= 100.0` / `ssim == 1.0` のみ。任意 2 画像で「有限値かつ NaN でない」性質が未検証。`psnr >= 100.0` は同一画像なら `kMaxPsnr`（128.0）が確定するため `== 128.0` に厳格化できる
- `pbt/src/lib.rs`: `even_size` は偶数前提を doc に明記済みだが、`arb_i420` / `arb_biplanar` は偶数前提が doc に明記されていない
- 全 PBT で stride == width のみ。パディング付き stride（stride > width）のラウンドトリップが存在しない

なお `i420_ssim` は width / height が 16 以下（U/V プレーンが 8 以下）でも `Ok(NaN)` を返す（0039 は width <= 8 || height <= 8 のみ Err 化したため、9〜16 の修正漏れが残存）。この修正漏れは別 issue（0072）で対応し、本 issue の性質テストは SSIM を 18x18 以上に限定して回避する。

## 設計方針

- scale: 等倍以外（ダウンスケール / アップスケール）と `FilterMode` 全モードで「すべて同一値の画像はスケール後も同一値」を検証する（ダウンスケール → アップスケールの往復一致はロッシー変換のため成立しない性質であり検証しない。「すべて同一値」の性質は全フィルタで出力が同一になるため、各カーネルを実際に通すことは縮小比の選定で構成的に保証する。`ScaleFilterReduce` の還元を考慮し、Box カーネルは両軸とも 1/2 未満の縮小でのみ実行される（1/2 以上またはアップスケールは Bilinear に還元）、Bilinear は 2 倍拡大で固有のカーネルを通る（1/2 縮小は Box と同一カーネル `ScaleRowDown2Box_C` のため除外）、Linear は縦 1/3 縮小 × 横 1/2 縮小等の組合せで検証する）
- rotate: `Rotate270` を独立して検証する（4 回ラウンドトリップは `Rotate90` と同一実装でも通過する。「Rotate270 を 2 回適用が Rotate180 と一致」も 90 度回転が同式を満たすため識別できない。270 と 90 を識別できるのは「`Rotate90` 適用後の `Rotate270` で恒等」の関係式であり、これで検証する。なお関係式ベースでは Rotate90 / Rotate270 が共に同一の別誤回転（例: 両方 Rotate180）で実装された場合を検出できないため、既知の非対称小パターンの回転結果が既知の画素配置と一致する固定入力の具体検証も 1 つ加える）
- compare: 任意 2 画像の PSNR / SSIM が NaN でない・有限値である性質を追加する（SSIM は U/V プレーンが 9x9 以上になる 18x18 以上のサイズに限定。PSNR はサイズ制約なし）
- パディング付き stride（stride = width + padding）でラウンドトリップする戦略を追加する（パディング領域は変換が書き込まないため、比較は width 分のピクセルデータに限定する。適用対象はロスレス変換（copy 系・mirror 系・convert の往復等）で、rotate 90 / 270 のように出力寸法が転置される関数には適用しない）
- 0052 / 0053 からの委譲を受けて、convert（`prop_convert.rs`）と planar（`prop_planar.rs`）のロスレス変換（copy 系・チャンネル入替・split/merge 系・mirror 系等）のラウンドトリップ PBT を追加する
- `pbt/src/lib.rs` の `arb_i420` / `arb_biplanar` に偶数前提を doc で明記する

## 完了条件

- `FilterMode` の `Linear` / `Bilinear` / `Box` を含むスケールの性質テストが存在すること（`ScaleFilterReduce` の還元を考慮した縮小比（Box は両軸 1/2 未満の縮小、Bilinear は 2 倍拡大（1/2 縮小は Box と同一カーネルのため除外）、Linear は縦 1/3 縮小 × 横 1/2 縮小等）で、各カーネルを実際に通すこと。適用関数は `i420_scale` / `argb_scale` / `nv12_scale` / `scale_plane` のいずれかから選定し、選定一覧を実装時に issue に追記して確定すること）
- 等倍以外のサイズ変化（ダウンスケール / アップスケール）のテストが存在すること
- `Rotate270` の独立した検証が存在すること（「`Rotate90` 適用後の `Rotate270` で恒等」の関係式ベース。4 回ラウンドトリップや 2 回 = Rotate180 ではない）
- 任意 2 画像で PSNR / SSIM が NaN にならない性質テストが存在すること（SSIM は 18x18 以上のサイズに限定。`even_size()` は偶数しか生成しないため、0072 の修正（下限 17）後も実効下限は 18 のまま）
- パディング付き stride のラウンドトリップテストが存在すること（比較は width 分のみ。ロスレス変換（copy 系・mirror 系・convert の往復等）に適用）
- convert（`prop_convert.rs`）と planar（`prop_planar.rs`）のロスレス変換のラウンドトリップ PBT が追加されていること（0052 / 0053 からの委譲分。0052 / 0053 の選定一覧に含まれるロスレス関数（copy 系・split/merge 系・mirror 系等）を対象とし、対象関数の選定一覧を実装時に issue に追記して確定すること）
- `pbt/src/lib.rs` の `arb_i420` / `arb_biplanar` に偶数前提の doc が明記されていること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `cargo fmt --all --check` と `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加すること

## 解決方法

1. `prop_scale.rs` に等倍以外の戦略（ダウンスケール / アップスケール / 全 FilterMode。`ScaleFilterReduce` の還元を考慮した縮小比）を追加する
2. `prop_rotate.rs` に `Rotate270` の関係式ベースの検証を追加する
3. `prop_compare.rs` に非同一画像の性質テストを追加し、同一画像の PSNR を `== 128.0` に厳格化する（SSIM は 18x18 以上に限定）
4. `prop_convert.rs` / `prop_planar.rs` にロスレス変換のラウンドトリップ PBT を追加する（0052 / 0053 からの委譲分）
5. `pbt/src/lib.rs` にパディング付き stride の戦略と `arb_i420` / `arb_biplanar` の偶数前提 doc を追加する
6. `CHANGES.md` の `### misc` に `[ADD]` エントリを追加する
