# convert_to_lsb_plane_16 が depth=16 で SIMD 経路では全 0 を出力し、C 経路では符号付き乗算が UB になる

- Priority: High
- Created: 2026-08-12
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-lsb-plane-16-multiply-overflow
- Polished: 2026-08-17
- Reporter: @voluntas

## 目的

`src/planar.rs` の `convert_to_lsb_plane_16` は crate 仕様の有効範囲 8..=16 の `depth` を許可している。`depth == 16` のとき libyuv の `ConvertToLSBPlane_16` は、SIMD 経路では非ゼロ画素を全 0 にし、C 経路では符号付き乗算が未定義動作になる。有効範囲内の入力で誤出力と UB が発生する経路をなくす。

## 優先度根拠

High。

- `depth == 16` は 16bit 変換関数の有効範囲（8..=16）の上限値であり、`convert_to_lsb_plane_16` が受け付ける有効な入力である（0049 で Ok と確定した契約）。Linux / macOS の SIMD 対応 CPU（AVX2 / NEON / SVE2）では SIMD 行関数が選ばれ、非ゼロ入力が無言で全 0 になる（データ破壊）。SIMD 非対応の x86 や Windows MSVC では `DivideRow_16_C` に落ち、`src >= 32768` で符号付き乗算が UB になる。どちらの経路に落ちても、有効範囲内の一部の入力（SIMD は非ゼロ、C は `src >= 32768`）で誤出力か UB のどちらかが起きる
- 既存テスト `call_convert_plane_16` は src が全 0 で、戻り値の Ok しか見ていないため、この誤出力を検出できない

## 現状

C 側の `ConvertToLSBPlane_16`（`planar_functions.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は `int scale = 1 << depth;` を計算し、既定は `DivideRow_16_C`、CPU フラグがあれば SIMD 行関数（`DivideRow_16_AVX2` / `DivideRow_16_NEON` / `DivideRow_16_SVE2`）へディスパッチする。AVX2 / NEON の余り処理 `ANY11C` は C に落とさず同じ SIMD 関数を呼ぶ。SVE2 は Any ラッパーを使わず、余りも `umulh` で処理する（結論は同じで C に落ちない）。

C 経路 `DivideRow_16_C`（`row_common.cc`）:

```c
dst_y[x] = (src_y[x] * scale) >> 16;
```

`src_y[x]` は `uint16_t` が `int` に昇格し、`depth == 16` では `scale == 65536` になる。`src_y[x] >= 32768` のとき `src_y[x] * 65536` は `INT_MAX`（2^31 - 1）を超え、符号付きオーバーフロー（未定義動作）になる。`32767 * 65536 = 2147418112` は `INT_MAX` 未満、`32768 * 65536 = 2147483648` は超過。`depth == 15` の最大積 `65535 * 32768 = 2147450880` は `INT_MAX` 以下のため、C の符号付き溢れは有効範囲内では `depth == 16` だけである。2 の補数 wrap と算術右シフトを仮定しても `(x * 65536) >> 16` は `x >= 32768` で `x - 65536`（負値）になるが、`dst_y[x]` が `uint16_t` のため代入時の mod 2^16 変換で `x` に戻る。ただし符号付きオーバーフローは UB のためこの偶然の一致に依存できず、しかも SIMD 経路では乗数が 0 になるためこの一致すら起きない。

SIMD 経路では `scale` を 16bit レーンへ放送する（AVX2 の `vpbroadcastw`、NEON の `dup v4.8h` / `vdup.16`、SVE2 の `dup z0.h`）。`65536 = 0x00010000` の下位 16 bit は 0 なので、乗数は 0 になり **src が 0 でなくても出力は全 0** になる。32767 は C では溢れないが、SIMD では 0 になる。

0049 の対応で `convert_to_lsb_plane_16` は `depth` を 8..=16 に検証したが、この検証はシフト演算（`1 << depth`）の UB を防ぐものであり、乗算 UB は防げない（0049 の設計方針でスコープ外と明記済み。SIMD 誤出力については 0049 は言及していない）。`convert_to_lsb_plane_16` は検証後に分岐なしで `sys::ConvertToLSBPlane_16` を呼ぶ。

## 設計方針

`depth == 16` では `sys::ConvertToLSBPlane_16` を呼ばない。既存の検証（エラーの `function` は `"ConvertToLSBPlane_16"`、必要サイズは `checked_buf_size` による `stride * height`）は維持し、検証通過後に `sys::CopyPlane_16` で 16bit 行コピーする。`sys::CopyPlane_16` には現行の `ConvertToLSBPlane_16` と同じ要素単位の stride / width をそのまま渡す。呼び出し側で `* 2` しない（内部が `CopyPlane` 向けに倍にする。`HalfFloatPlane` とは違う）。ただし `sys::CopyPlane_16` は内部で `stride * 2` / `width * 2` を `int` で計算するため、`depth == 16` の分岐では `stride * 2` / `width * 2` が `c_int` 範囲内であることを `half_float_plane` と同じパターンで再検証する（要素単位の `require_c_int` だけでは `INT_MAX / 2` 超の stride / width が通過し、内部の `int` 乗算がオーバーフローする。再検証のエラーの `function` は既存検証と同じ `"ConvertToLSBPlane_16"` とする）。

採用しない:

- `depth == 16` を `Err` にする。0049 が `depth == 16` で Ok と確定した契約の機能退行になる
- docstring に注意書きを残して C を呼び続ける。SIMD 誤出力と C の UB が残る
- `copy_plane`（`&[u8]`）を使う。型と stride の単位が合わない
- 検証前に `copy_plane_16` へ委譲する。必要サイズが `(height - 1) * stride + width` に緩み、エラーの `function` が `"CopyPlane_16"` に変わる

`copy_plane` / `copy_plane_16` の本体は変更しない。有効範囲内のゼロサイズ（`width == 0` / `height == 0`）は現行どおり no-op（`Ok`）とする。ゼロサイズを `Err` に統一するのは 0064 のスコープ。`depth != 16` は現行どおり `sys::ConvertToLSBPlane_16` を呼ぶ。`convert_to_msb_plane_16` は `depth == 16` でも `scale = 1 << (16 - 16) = 1` の恒等になり安全なため対象外（`depth == 16` を有効範囲として受け付ける点に注意）。C 側が `ConvertToLSBPlane_16` を内部利用する変換（`p010_to_i010` は `depth = 10`、`p012_to_i012` は `depth = 12`）も `depth != 16` のため対象外。0053 が depth=16 の正常系を書く場合は本 issue の恒等（入力一致）に合わせ、libyuv 出力をオラクルにしない。

根拠は libyuv commit `d23308a2a7442be8e559b1b471862fd7588d6a57` の `ConvertToLSBPlane_16` / `DivideRow_16_C` / SIMD 行関数であり、libyuv 更新時に見直すべき箇所としてコメントに明記する。

## 完了条件

- `depth == 16` で `sys::ConvertToLSBPlane_16` を呼ばないこと
- `depth == 16` かつ src 値が 32767 / 32768 / 65535 のとき、出力が入力と一致すること。期待値は入力そのものであり、現行 `ConvertToLSBPlane_16` の出力（SIMD では 0）をオラクルにしない。テストは height >= 2 とする（stride 送りの誤りを検出するため）。なお恒等テストは C 経路の環境（MSVC / AVX2 非搭載の x86）では修正前のコードでも通る可能性がある（`uint16_t` 格納の mod 2^16 変換で `x` に戻るため）。修正の検証は「`sys::ConvertToLSBPlane_16` を呼ばないこと」（上記）と、CI の SIMD 機（Linux / macOS）での回帰検出が主である
- `depth != 16` の既存の挙動が変わらないこと（他 depth の変換正しさ・エラーパス網羅は 0053 のスコープ。本 issue のテストは depth=16 の恒等のみ）
- 境界値テスト（32767 / 32768 / 65535、height >= 2）が `tests/test_planar.rs` に追加されていること（0053 で分割された場合は `tests/test_planar/` 配下）。行ごとに異なる画素値を設定する（全行同一の画素値では stride 送りの誤りを検出できない）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `convert_to_lsb_plane_16` の既存検証のあと、`depth == 16` なら `sys::CopyPlane_16`（`stride * 2` / `width * 2` の `c_int` 再検証後）、それ以外は `sys::ConvertToLSBPlane_16` を呼ぶ
2. 根拠（SIMD が `scale = 65536` の下位 16 bit を 0 として放送すること、C の `DivideRow_16_C` の符号付き乗算オーバーフロー）をコメントで明記し、docstring に `depth == 16` は恒等コピー（`sys::CopyPlane_16`）に置き換わる旨を追記する
3. `tests/test_planar.rs` に画素値 32767 / 32768 / 65535 の恒等テスト（height >= 2）を追加する
4. `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加する
