# convert_to_lsb_plane_16 の有効範囲内（depth=16）で C 側の乗算が int オーバーフロー UB になる

- Priority: Medium
- Created: 2026-08-12
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-lsb-plane-16-multiply-overflow
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/planar.rs` の `convert_to_lsb_plane_16` は crate 仕様の有効範囲 8..=16 の `depth` を許可しているが、`depth == 16` のとき C 側の `DivideRow_16_C` が符号付き乗算オーバーフロー（C 標準上は未定義動作）を起こしうる。有効範囲内の入力で UB が発生する経路をなくす。

## 優先度根拠

Medium。

- 実害は限定的（2 の補数表現では乗算が wrap し、`(x * 65536) >> 16` は x に戻るため出力は正しい。ただし C 標準上は UB であり、UBSan 等で検出される）
- 有効範囲内（depth == 16）かつ src 値が 32768 以上（最上位ビットが立っている）のときのみ発生する

## 現状

C 側の `ConvertToLSBPlane_16`（`planar_functions.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は `int scale = 1 << depth;` を計算し、行関数 `DivideRow_16_C`（`row_common.cc`）が以下を実行する:

```c
dst_y[x] = (src_y[x] * scale) >> 16;
```

`src_y[x]` は `uint16_t` が `int` に昇格し、`depth == 16` では `scale == 65536` になる。`src_y[x] >= 32768` のとき `src_y[x] * 65536` は `INT_MAX`（2^31 - 1）を超え、符号付きオーバーフロー（未定義動作）になる。

0049 の対応で `convert_to_lsb_plane_16` は `depth` を 8..=16 に検証したが、この検証はシフト演算（`1 << depth`）の UB を防ぐものであり、`DivideRow_16_C` 内の乗算 UB は防げない（0049 の設計方針でスコープ外と明記済み）。

## 設計方針

対応方法は次の選択肢が考えられる（本 issue で確定する）:

- `depth == 16` のときは `(x * 65536) >> 16 == x` が恒等式であるため、Rust 側で行コピー（`copy_plane` 相当）に置き換える（乗算を回避する）
- `depth == 16` を `Err` にする（16bit フルレンジの LSB 変換が使えなくなる機能退行）
- C 側の修正を上流に依頼し、libyuv 更新まで docstring に UB の注意書きを残す（UB は残る）

`depth == 16` は 16bit データの標準的な使い方であり、機能退行を伴わない選択肢（1 番目）が望ましい。実装時に C 側の `DivideRow_16_C` と `ConvertToLSBPlane_16` の動作を確認して確定する（libyuv 更新時に見直すべき箇所としてコメントに明記する）。

## 完了条件

- `depth == 16` の入力で UB が発生しないこと（src 値に 32768 以上の値を含むテストを追加し、出力が入力と一致すること）
- `depth != 16` の既存の挙動が変わらないこと
- 境界値テスト（32767 / 32768 / 65535 の src 値）が `tests/test_planar.rs` に追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `convert_to_lsb_plane_16` の `depth == 16` の分岐を実装する（乗算を回避する行コピー、または設計方針で確定した方法）
2. 根拠（`DivideRow_16_C` の乗算オーバーフローと恒等式）をコメントで明記する
3. `tests/test_planar.rs` に境界値テスト（32767 / 32768 / 65535）を追加する
4. `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加する
