# YUY2/UYVY パック YUV 変換追加

## 概要

YUY2 (YUYV) / UYVY パック形式の YUV フォーマット変換関数を追加する。

## 対象関数

### YUY2
- YUY2ToARGB
- YUY2ToI420
- YUY2ToI422
- YUY2ToNV12
- YUY2ToY
- ARGBToYUY2
- I420ToYUY2
- I422ToYUY2

### UYVY
- UYVYToARGB
- UYVYToI420
- UYVYToI422
- UYVYToNV12
- UYVYToY
- ARGBToUYVY
- I420ToUYVY
- I422ToUYVY

### Matrix 付き
- YUY2ToARGBMatrix
- UYVYToARGBMatrix

## 解決方法

convert.rs に以下の 16 関数を追加した:

YUY2:
- yuy2_to_argb, yuy2_to_i420, yuy2_to_i422, yuy2_to_nv12, yuy2_to_y
- argb_to_yuy2, i420_to_yuy2, i422_to_yuy2

UYVY:
- uyvy_to_argb, uyvy_to_i420, uyvy_to_i422, uyvy_to_nv12, uyvy_to_y
- argb_to_uyvy, i420_to_uyvy, i422_to_uyvy

YUY2ToARGBMatrix, UYVYToARGBMatrix は YuvConstants が必要なため issue 0016 に移譲した。
