# argb_blur の cumsum バッファ検証が不足しヒープ領域外への書き込みが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-argb-blur-cumsum-validation
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/planar.rs` の `argb_blur` が cumsum バッファの必要サイズを過小検証しており、`radius` が大きい場合に libyuv がバッファ領域外へ書き込む。cumsum の必要行数（`radius * 2 + 2`）を検証に反映する。

## 優先度根拠

High。

- radius == height に近い入力でヒープ領域外への書き込み（ヒープ破壊）が発生する
- unsafe FFI 呼び出し前に検証が足りないままバッファを渡す設計上の欠陥

## 現状

libyuv の `ARGBBlur`（`planar_functions.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は cumsum を循環バッファとして使用し、ヘッダのコメントに以下を明記している:

> Caller should allocate CumulativeSum table of width * height * 16 bytes aligned to 16 byte boundary. height can be radius * 2 + 2 to save memory as the buffer is treated as circular.

実装では `max_cumsum_bot_row = &dst_cumsum[(radius * 2 + 2) * dst_stride32_cumsum];` とし、`radius` は `min(radius, height)` と `min(radius, width / 2 - 1)` に clamp される。つまり必要な cumsum 行数は最大 `radius * 2 + 2` 行。

一方 `src/planar.rs` の `argb_blur` は `cumsum.len() >= stride32_cumsum * size.height` のみ検証する。radius == height の入力（例: 800x100、radius=100）では C 側が約 202 行目まで書き込むのに対し検証は 100 行分しか要求せず、約 100 行分（数百 KB〜数 MB）のヒープ越境書き込みになる。

## 設計方針

`argb_blur` の cumsum 検証を C 側の clamp 規則（`radius = min(radius, height)`、`radius = min(radius, width / 2 - 1)`）を模倣して「実際に使用される行数」で行う:

- 有効 radius を C と同じ規則で計算する
- 必要な行数 = `min(size.height, effective_radius * 2 + 2)` として `checked_buf_size` で検証する
- `radius <= 0` または `height <= 1` の場合は C が `-1` を返すため、Rust 側でも明示的に `Err` を返す（C 実装 `ARGBBlur` の事前条件と整合させる）

## 完了条件

- radius == height に近い入力で cumsum 不足が `Err` になること
- radius / height / width の境界値（radius == height、radius > height、height <= 1、width が小さいケース）で C 側の挙動と整合すること
- `tests/test_planar.rs` に境界値テストが追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `argb_blur` に C と同じ radius clamp 規則を実装する
2. 有効 radius から cumsum の必要行数（`radius * 2 + 2`）を計算し `checked_buf_size` で検証する
3. `radius <= 0` / `height <= 1` / サイズ検証失敗時に `Err` を返す
4. docstring に cumsum の必要行数（radius * 2 + 2 行）を明記する
5. `tests/test_planar.rs` にテストを追加する
6. `CHANGES.md` に `[FIX]` エントリを追加する
