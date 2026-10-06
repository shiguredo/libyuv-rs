# アルファプレーン検証で size.height == 0 または size.width == 0 時に panic または不適切な挙動をする

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-convert-alpha-validate-height-zero
- Polished: 2026-06-27
- Reporter: @voluntas

## 目的

`convert.rs` の `validate_alpha_src` / `validate_alpha_dst` が `size.height == 0` のとき `usize` の減算で panic するのを防ぎ、代わりに `Error` を返すようにする。

## 優先度根拠

debug ビルドで即座に panic する。公開 API (`i420_alpha_to_argb` 等) を通じて容易に引き起こせる。

## 現状

`src/convert.rs` の `validate_alpha_src` / `validate_alpha_dst` は、バッファサイズを以下のように計算している。

```rust
let required = src_stride_a * (size.height - 1) + size.width;
```

`size.height == 0` のとき `size.height - 1` が `usize` のアンダーフローとなり、debug ビルドでは panic する。

この検証は以下の 7 つの公開 API で使用されている。

- `i420_alpha_to_argb` → libyuv `I420AlphaToARGB`
- `i420_alpha_to_abgr` → libyuv `I420AlphaToABGR`
- `i422_alpha_to_argb` → libyuv `I422AlphaToARGB`
- `i422_alpha_to_abgr` → libyuv `I422AlphaToABGR`
- `i444_alpha_to_argb` → libyuv `I444AlphaToARGB`
- `i444_alpha_to_abgr` → libyuv `I444AlphaToABGR`
- `argb_to_i420_alpha` → libyuv `ARGBToI420Alpha`

libyuv 側の実装（`source/convert_argb.cc` の `I420AlphaToARGBMatrix` 系、`source/convert.cc` の `ARGBToI420Alpha`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は `height == 0` または `width <= 0` の場合に `-1` を返す。例えば `I420AlphaToARGBMatrix` では以下のように先行判定している:

```c
if (!src_y || !src_u || !src_v || !src_a || !dst_argb || width <= 0 ||
    height == 0 || height == INT_MIN) {
  return -1;
}
```

`ARGBToI420Alpha` でも同様に `width <= 0 || height == 0 || height == INT_MIN` で `-1` を返す。したがって本バインディング側でも同様に `Error` を返すのが自然である。

### 再現手順

以下は現状の挙動を確認するためのコード例である。現状では `validate_alpha_src` 内の `size.height - 1` で panic するため、最後の `assert` には到達しない。修正後は `result.is_err()` となる。

```rust
use shiguredo_libyuv::{i420_alpha_to_argb, I420Image, ArgbImageMut, ImageSize};

let y = vec![0u8; 0];
let u = vec![0u8; 0];
let v = vec![0u8; 0];
let src_a = vec![0u8; 0];
let mut dst = vec![0u8; 0];
let src = I420Image {
    y: &y,
    y_stride: 0,
    u: &u,
    u_stride: 0,
    v: &v,
    v_stride: 0,
};
let mut dst_argb = ArgbImageMut {
    data: &mut dst,
    stride: 0,
};
let size = ImageSize::new(0, 0);
let result = i420_alpha_to_argb(&src, &src_a, 0, &mut dst_argb, size, false);
// 現状はここで panic する。修正後は result.is_err() となる。
```

## 設計方針

`validate_alpha_src` / `validate_alpha_dst` において、バッファサイズ計算 `(size.height - 1) * stride + size.width` より先に、`size.width == 0` または `size.height == 0` の場合は `Error::with_reason(-1, function, "invalid image size")` を返す。これにより `usize` のアンダーフローを防ぎ、libyuv の事前条件と整合する。

`size.height > 0` かつ `size.width > 0` の場合は、従来通りバッファサイズを検証するが、`checked_mul` / `checked_add` を用いてオーバーフロー安全に計算する。例:

```rust
let required = (size.height - 1)
    .checked_mul(src_stride_a)
    .and_then(|v| v.checked_add(size.width))
    .ok_or_else(|| Error::with_reason(-1, function, "alpha buffer size overflow"))?;
```

本 issue の目的は `size.height == 0` 時の panic 防止に絞る。`src_stride_a` / `dst_stride_a` の `c_int` 範囲チェックや `stride < width` チェックの追加は別目的のため、別 issue で対応する。

## 完了条件

- `size.height == 0` の入力で上記 7 関数が panic せず `Error` を返すこと
- `size.width == 0` の入力でも `Error` を返すこと（既存の `src.validate()` / `dst.validate()` で先にエラーにならない場合もあるため、本検証でも必ずチェックする）
- `size.height > 0` かつ `size.width > 0` の場合は従来通りバッファサイズを検証すること
- 該当する境界値テストが `tests/test_convert.rs` に追加されていること
  - `size.height == 0` のケース（例: `size = ImageSize::new(1, 0)`、`src_a = &[]`、`src_stride_a = 1`。src/dst 画像側の stride は width 以上に設定し、画像側の `validate` が先に落ちないようにする）
  - `size.width == 0` のケース（例: `size = ImageSize::new(0, 1)`、`src_a = &[0]`、`src_stride_a = 1`）
  - バッファサイズが不足するケース（例: `size = ImageSize::new(8, 8)`、`src_a.len() < 8*8`）
  - 正常系ケース（例: `size = ImageSize::new(8, 8)`、十分なバッファで `Ok(())`）
  - テストコメントで、`width == 0` と `height == 0` のケースを区別して明示すること
- `cargo test --workspace --features source-build` が成功すること
- `cargo fmt --all --check` と `cargo clippy -p shiguredo_libyuv --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `develop` セクションに以下のような `[FIX]` エントリを追加すること
  - `[FIX] convert モジュールのアルファプレーン検証で size.width == 0 または size.height == 0 時に panic または不適切な挙動をする問題を修正する`
  - @voluntas

## 解決方法

1. `validate_alpha_src` / `validate_alpha_dst` に `size.width == 0 || size.height == 0` の検証を追加する
   - `Error::with_reason(-1, function, "invalid image size")` を返す
2. `size.height > 0` かつ `size.width > 0` の場合は、バッファサイズを `(size.height - 1).checked_mul(stride).and_then(|v| v.checked_add(size.width))` で計算し、`None` の場合は `Error::with_reason(-1, function, "alpha buffer size overflow")` を返す
3. `tests/test_convert.rs` を新設し、上記 7 関数で境界値テストと正常系テストを追加する
4. `CHANGES.md` に `[FIX]` エントリを追加する
