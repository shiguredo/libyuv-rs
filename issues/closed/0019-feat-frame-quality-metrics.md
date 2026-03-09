# フレーム品質計測関数の追加

## 概要

単一プレーン/フレーム向けの品質計測関数を compare モジュールに追加する。
既存の `i420_psnr` / `i420_ssim` は 3 プレーン YUV 専用であり、ARGB 等のパックドフォーマットには対応していない。

## 対象関数

### CalcFramePsnr

単一プレーンの PSNR (Peak Signal-to-Noise Ratio) を計算する。

```c
LIBYUV_API
double CalcFramePsnr(const uint8_t* src_a,
                     int stride_a,
                     const uint8_t* src_b,
                     int stride_b,
                     int width,
                     int height);
```

### CalcFrameSsim

単一プレーンの SSIM (Structural Similarity Index) を計算する。

```c
LIBYUV_API
double CalcFrameSsim(const uint8_t* src_a,
                     int stride_a,
                     const uint8_t* src_b,
                     int stride_b,
                     int width,
                     int height);
```

### ComputeSumSquareError

stride なしバッファの二乗誤差合計を計算する。

```c
LIBYUV_API
uint64_t ComputeSumSquareError(const uint8_t* src_a,
                               const uint8_t* src_b,
                               int count);
```

## 元 issue

issue 0016 から分離。設計判断は不要で単純な追加のため独立させた。

## 解決方法

compare.rs に以下の 3 関数を追加した:

- `calc_frame_psnr`: 単一プレーンの PSNR 計算
- `calc_frame_ssim`: 単一プレーンの SSIM 計算
- `compute_sum_square_error`: stride なしバッファの二乗誤差合計計算
