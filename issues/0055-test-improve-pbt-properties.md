# PBT の検証力を向上させる（等倍限定・対合性限定からの脱却）

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/test-improve-pbt-properties
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`pbt/` の PBT が「等倍スケールのみ」「対合性（2 回 / 4 回で元に戻る）のみ」の検証に偏っており、誤った実装でも通過してしまう。PBT で検証できる性質を最大限検証するように改善する。

## 優先度根拠

Medium。

- `prop_scale.rs` の全テストは `FilterMode::None` の等倍のみ。libyuv は等倍時に `CopyPlane` へ短絡するため（`scale.cc` の `ScalePlane`）、スケールカーネル（Down2 / Bilinear 等）が一切検証されていない
- `prop_rotate.rs` / `prop_convert.rs` / `prop_planar.rs` のラウンドトリップは「対合的な変換」なら誤った実装でも通過する
- `prop_compare.rs` は同一画像のみで、非同一画像の NaN なし等の性質が未検証

## 現状

- `pbt/tests/prop_scale.rs`: 4 テストすべて `FilterMode::None` の等倍（実質コピー）のみ。`Linear` / `Bilinear` / `Box` の正しさは未検証
- `pbt/tests/prop_rotate.rs`: `Rotate180`（2 回で戻る）と `Rotate90`（4 回で戻る）のみ。`Rotate270` は 90 の逆回転で独立した検証にならない
- `pbt/tests/prop_compare.rs`: 同一画像の `psnr >= 100.0` / `ssim == 1.0` のみ。任意 2 画像で「有限値かつ NaN でない」性質が未検証。`psnr >= 100.0` は同一画像なら `kMaxPsnr`（128.0）が確定するため `== 128.0` に厳格化できる
- `pbt/src/lib.rs`: `even_size` / `arb_i420` は偶数サイズ前提だが doc に明記されていない
- 全 PBT で stride == width の等倍のみ。パディング付き stride（stride > width）のラウンドトリップが存在しない

## 設計方針

- scale: 等倍以外（ダウンスケール / アップスケール）と `FilterMode` 全モードで「同一画像をスケールして戻す」または「面積 / 平均の性質」を検証する（例: ダウンスケール後の画像サイズが期待どおり、すべて同一値の画像はスケール後も同一値）
- rotate: `Rotate270` を独立して検証する
- compare: 任意 2 画像の PSNR / SSIM が NaN でない・有限値である性質を追加する
- パディング付き stride（stride = width + padding）でラウンドトリップする戦略を追加する
- `pbt/src/lib.rs` のヘルパーに偶数前提を doc で明記する（または内部で `assert` する）

## 完了条件

- `FilterMode` の `Linear` / `Bilinear` / `Box` を含むスケールの性質テストが存在すること
- 等倍以外のサイズ変化（ダウンスケール / アップスケール）のテストが存在すること
- `Rotate270` の独立した検証が存在すること
- 任意 2 画像で PSNR / SSIM が NaN にならない性質テストが存在すること
- パディング付き stride のラウンドトリップテストが存在すること
- `pbt/src/lib.rs` のヘルパーに偶数前提の doc が明記されていること
- 全テストが `cargo test --workspace --features source-build` で成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. `prop_scale.rs` に等倍以外の戦略（ダウンスケール / アップスケール / 全 FilterMode）を追加する
2. `prop_rotate.rs` に `Rotate270` を追加する
3. `prop_compare.rs` に非同一画像の性質テストを追加し、同一画像の PSNR を `== 128.0` に厳格化する
4. `pbt/src/lib.rs` にパディング付き stride の戦略と doc を追加する
5. `CHANGES.md` の `### misc` にエントリを追加する
