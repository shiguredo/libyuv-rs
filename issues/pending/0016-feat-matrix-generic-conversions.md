# マトリックスベース色空間変換の追加

## 概要

YuvConstants を利用したマトリックスベースの色空間変換関数を追加する。

## 対象関数

### マトリックス付き変換
- I420ToARGBMatrix
- I420ToARGBMatrixFilter
- I420ToRGB24Matrix
- I420ToRGB24MatrixFilter
- I420ToRGB565Matrix
- I420ToRGBAMatrix
- I422ToARGBMatrix
- I422ToARGBMatrixFilter
- I422ToRGB24Matrix
- I422ToRGB24MatrixFilter
- I422ToRGB565Matrix
- I422ToRGBAMatrix
- I444ToARGBMatrix
- I444ToRGB24Matrix
- NV12ToARGBMatrix
- NV12ToRGB24Matrix
- NV12ToRGB565Matrix
- NV21ToARGBMatrix
- NV21ToRGB24Matrix
- I400ToARGBMatrix
- I420ToAR30Matrix
- I010ToARGBMatrix
- I010ToARGBMatrixFilter
- I010ToAR30Matrix
- I010ToAR30MatrixFilter
- I210ToARGBMatrix
- I210ToARGBMatrixFilter
- I210ToAR30Matrix
- I210ToAR30MatrixFilter
- I410ToARGBMatrix
- I410ToAR30Matrix
- I012ToAR30Matrix
- I012ToARGBMatrix
- P010ToARGBMatrix
- P010ToARGBMatrixFilter
- P010ToAR30Matrix
- P010ToAR30MatrixFilter
- P210ToARGBMatrix
- P210ToARGBMatrixFilter
- P210ToAR30Matrix
- P210ToAR30MatrixFilter
- I010AlphaToARGBMatrix
- I010AlphaToARGBMatrixFilter
- I210AlphaToARGBMatrix
- I210AlphaToARGBMatrixFilter
- I410AlphaToARGBMatrix
- I420AlphaToARGBMatrix
- I420AlphaToARGBMatrixFilter
- I422AlphaToARGBMatrix
- I422AlphaToARGBMatrixFilter
- I444AlphaToARGBMatrix
- Android420ToARGBMatrix
- YUY2ToARGBMatrix
- UYVYToARGBMatrix

### 逆方向マトリックス付き変換
- ARGBToI420Matrix
- ARGBToI422Matrix
- ARGBToI444Matrix

## pending 理由

YuvConstants の Rust 表現について設計判断が必要。

libyuv の C API では `YuvConstants` 構造体 (kUVCoeff + kRGBCoeffBias、32 bytes) へのポインタを渡す。
C 側に定義済みの定数は以下の 6 つ:

- `kYuvI601Constants` (BT.601 リミテッドレンジ)
- `kYuvJPEGConstants` (BT.601 フルレンジ)
- `kYuvH709Constants` (BT.709 リミテッドレンジ)
- `kYuvF709Constants` (BT.709 フルレンジ)
- `kYuv2020Constants` (BT.2020 リミテッドレンジ)
- `kYuvV2020Constants` (BT.2020 フルレンジ)

**提案: Rust enum でマッピングする。**

```rust
pub enum ColorSpace {
    Bt601,        // kYuvI601Constants (リミテッドレンジ)
    Bt601Full,    // kYuvJPEGConstants (フルレンジ / JPEG)
    Bt709,        // kYuvH709Constants (リミテッドレンジ)
    Bt709Full,    // kYuvF709Constants (フルレンジ)
    Bt2020,       // kYuv2020Constants (リミテッドレンジ)
    Bt2020Full,   // kYuvV2020Constants (フルレンジ)
}
```

理由:
- ユーザーが C の定数名を知る必要がなくなる
- カスタムマトリックスは libyuv 自体がサポートしていない (内部構造がプラットフォーム依存) ので不要
- 6 つの固定値しかないため enum が最適

## 分離した issue

- 汎用変換関数 (ConvertToARGB / ConvertToI420 / ConvertFromI420) → issue 0021 (実装しない判断で close 済み)
- 品質計測関数 (CalcFramePsnr / CalcFrameSsim / ComputeSumSquareError) → issue 0019
