# SSIM 関数が小さな画像で NaN を返す

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-compare-ssim-small-image
- Polished: 2026-06-27
- Reporter: @voluntas

## 目的

`compare.rs` の `i420_ssim` と `calc_frame_ssim` が小さな画像に対して libyuv 内部で `samples == 0` となり、平均化の除算結果として `NaN` を返すバグを修正する。

## 優先度根拠

無効な入力サイズに対して `Ok(NaN)` を返す。利用者が `Result` の中身を検証せずに下游に伝搬させる可能性があり、品質比較結果を信頼できなくなる。特にサムネイル、タイル、低解像度フレームの SSIM 計算で誤った値が使われるリスクがある。

## 現状

`src/compare.rs` の `calc_frame_ssim` は libyuv の `CalcFrameSsim` を、`i420_ssim` は libyuv の `I420Ssim` を呼び出している。

libyuv の `CalcFrameSsim`（`source/compare.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は、8x8 ウィンドウを 4x4 ずつずらしてサンプリングする。ループ条件は以下の通り:

```c
for (i = 0; i < height - 8; i += 4) {
  for (j = 0; j < width - 8; j += 4) {
    // サンプル収集
  }
}
// ...
ssim_total /= samples;
```

`width < 9` または `height < 9` の場合、ループが 1 回も実行されず `samples == 0` となる。その後 `ssim_total /= samples` で 0 による除算が行われ、IEEE 754 準拠環境では返り値が `NaN` になる。

`I420Ssim` は Y 平面に対して元画像サイズ、`U` / `V` 平面に対してそれぞれ以下のサイズで `CalcFrameSsim` を実行する:

```c
int width_uv = (width + 1) >> 1;
int height_uv = (height + 1) >> 1;
```

整数除算により、`width = 16` のとき `width_uv = 8`、`width = 17` のとき `width_uv = 9` となる。`CalcFrameSsim` が 1 回以上実行されるには `width_uv >= 9` かつ `height_uv >= 9` が必要なので、以下が成り立つ:

- Y 平面: `width >= 9` かつ `height >= 9`
- U/V 平面: `(width + 1) / 2 >= 9` つまり `width >= 17`、同様に `height >= 17`

したがって元画像が 17x17 未満の場合、少なくとも Y 平面または U/V 平面のいずれかで `samples == 0` となり `NaN` が返る。例えば元画像が 16x16 の場合、Y 平面は 16x16 で問題ないが、U/V 平面は 8x8 となり `NaN` になる。

### 再現手順

以下は現状の挙動を確認するためのコード例である。修正後はこれらの `assert` は失敗し、代わりに `result.is_err()` となる。

```rust
use shiguredo_libyuv::{calc_frame_ssim, i420_ssim, I420Image, ImageSize};

// calc_frame_ssim の場合
let a = vec![0u8; 8 * 8];
let b = vec![0u8; 8 * 8];
let size = ImageSize::new(8, 8);
let result = calc_frame_ssim(&a, 8, &b, 8, size);
assert!(result.is_ok() && result.unwrap().is_nan()); // 現状は Ok(NaN)

// i420_ssim の場合
let y = vec![0u8; 16 * 16];
let u = vec![0u8; 8 * 8];
let v = vec![0u8; 8 * 8];
let src = I420Image {
    y: &y,
    y_stride: 16,
    u: &u,
    u_stride: 8,
    v: &v,
    v_stride: 8,
};
let result = i420_ssim(&src, &src, ImageSize::new(16, 16));
assert!(result.is_ok() && result.unwrap().is_nan()); // 現状は Ok(NaN)
```

## 設計方針

SSIM 計算には 8x8 サンプリングウィンドウを 1 つ以上配置できるサイズ、すなわち幅・高さともに 9 以上が必要である。それ未満の入力に対しては定義不能な値を返すのではなく、Rust ラッパー側で入力サイズを事前検証し `Err` を返す。libyuv 側では `samples == 0` の場合を防いでいないため、本バインディング側で防ぐ。

`Error` を返す理由は、本 crate の他の公開関数（`calc_frame_psnr` 等）と同様に、回復可能な入力ミスは `Result` で表現するという一貫性による。`panic` では呼び出し側が回復できず、また libyuv upstream へのパッチは本バインディングのリリースサイクル外であるため、ラッパー側で解決する。

`calc_frame_ssim` では `require_c_int` 直後、stride / バッファサイズチェック前に最小サイズチェックを挿入する。

`i420_ssim` では `I420Image::validate` 呼び出し前に最小サイズチェックを挿入する。`I420Image::validate` 内部では stride / バッファサイズチェックが実行されるため、サイズ下限チェックを `validate` より前に置くことで、サイズ不足は先にエラーとなる。

## 後方互換への影響

9x9 未満 / 17x17 未満の入力に対して、従来は `Ok(NaN)` を返していたが、本修正後は `Err(Error)` を返す。これは公開 API の挙動変更であり、呼び出し側で `Result` を適切に処理する必要がある。`CHANGES.md` には `[CHANGE]` エントリを追加する。

## 完了条件

- `calc_frame_ssim` が `size.width < 9` または `size.height < 9` の場合に `Error` を返すこと
- `i420_ssim` が `size.width < 17` または `size.height < 17` の場合に `Error` を返すこと
- 返す `Error` は `Error::with_reason(-1, "CalcFrameSsim", "image too small")` および `Error::with_reason(-1, "I420Ssim", "image too small")` の形式とすること
- `calc_frame_ssim` / `i420_ssim` の doc comment に最小サイズ制約とエラー条件を追記すること
  - `calc_frame_ssim` では: `size.width >= 9` かつ `size.height >= 9` が必要（8x8 サンプリングウィンドウを 1 つ以上配置できるサイズ）。それ未満の場合は `Err` を返す。
  - `i420_ssim` では: `size.width >= 17` かつ `size.height >= 17` が必要（U/V 平面が 9x9 以上になるため）。それ未満の場合は `Err` を返す。
- 単体テスト `tests/test_compare.rs` を新設し、以下を検証すること
  - `calc_frame_ssim`:
    - 8x8、8x9、9x8 で `Err`（stride=width、buf_len=width*height）
    - 0x0、0xN、Nx0 も `width < 9` / `height < 9` に含まれるため `Err` となるが、これらは上記の境界値で網羅されるため省略してもよい
    - 9x9 の同一画像で `Ok` かつ `|value - 1.0| < 1e-6`（stride=9、buf_len=81、内容は任意の同一画像でよい）
    - 64x48 の同一画像で `Ok` かつ `|value - 1.0| < 1e-6`（stride=64、buf_len=64*48、内容は任意の同一画像でよい）
  - `i420_ssim`:
    - 16x16、16x17、17x16 で `Err`（Y stride=width、U/V stride=width.div_ceil(2)、U/V buf_len=U/V stride * height.div_ceil(2)）
    - 0x0、0xN、Nx0 も `width < 17` / `height < 17` に含まれるため `Err` となるが、これらは上記の境界値で網羅されるため省略してもよい
    - 17x17 の同一画像で `Ok` かつ `|value - 1.0| < 1e-6`（Y stride=17、U/V stride=9、U/V buf_len=81）
    - 64x48 の同一画像で `Ok` かつ `|value - 1.0| < 1e-6`（Y stride=64、U/V stride=32、U/V buf_len=32*24）
- PBT は既存 `pbt/tests/prop_compare.rs` の `i420_ssim_identical` を維持し、今回は単体テストで境界値を網羅する
  - `i420_ssim_identical` は `(9..=32, 9..=32)` を `w*2, h*2` で 18x18 以上を生成するため、修正後も最小サイズ制限に抵触しない
- `cargo test --workspace --features source-build` が成功すること
- `cargo fmt --all --check` と `cargo clippy -p shiguredo_libyuv --features source-build -- -D warnings` が成功すること
- `CHANGES.md` の `develop` セクションに以下のような `[CHANGE]` エントリを追加すること
  - `[CHANGE] compare モジュールの SSIM 関数が小さな画像で NaN を返していたのを Error を返すように変更する`
  - @voluntas

## 解決方法

1. `calc_frame_ssim` に `size.width < 9 || size.height < 9` の検証を追加する
   - `require_c_int` 直後、stride / バッファサイズチェック前に挿入する
   - `Error::with_reason(-1, "CalcFrameSsim", "image too small")` を返す
2. `i420_ssim` に `size.width < 17 || size.height < 17` の検証を追加する
   - `I420Image::validate` 呼び出し前に挿入する
   - `Error::with_reason(-1, "I420Ssim", "image too small")` を返す
3. `calc_frame_ssim` / `i420_ssim` の doc comment に最小サイズ制約を追記する
4. `tests/test_compare.rs` を新設し、境界値テストと正常系テストを追加する
   - `i420_ssim` の奇数サイズテストでは U/V stride を `width.div_ceil(2)` で計算する（17x17 では 9）
5. `CHANGES.md` に `[CHANGE]` エントリを追加する
