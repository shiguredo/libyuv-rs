# 追加スケーリング関数追加

## 概要

I422, I444 等の追加スケーリング関数と ScaleClip を追加する。

## 対象関数

- I422Scale
- I444Scale
- ARGBScaleClip
- ScalePlane_12
- ScalePlane_16
- I420Scale_12
- I420Scale_16
- I422Scale_12
- I422Scale_16
- I444Scale_12
- I444Scale_16
- YUVToARGBScaleClip

## 解決方法

scale.rs に以下の関数を追加した:

- i422_scale, i444_scale
- i420_scale_12, i420_scale_16, i422_scale_12, i422_scale_16, i444_scale_12, i444_scale_16
- scale_plane_12, scale_plane_16
- nv24_scale, uv_scale, uv_scale_16
- argb_scale_clip

YUVToARGBScaleClip は YuvConstants が必要なため issue 0016 に移譲した。
RGBScale は libyuv.h に scale_rgb.h が含まれておらずバインディングが生成されないため除外した。
validate_planar16_src/dst, validate_biplanar16_src/dst を lib.rs で公開し、16bit スケーリング関数で使用した。
