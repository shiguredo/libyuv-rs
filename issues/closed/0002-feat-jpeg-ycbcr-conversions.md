# JPEG YCbCr (J系) フォーマット変換追加

## 概要

JPEG カラースペース (フルレンジ YCbCr) の変換関数を追加する。

## 対象関数

### J400 (グレースケール JPEG)
- J400ToARGB

### J420
- J420ToARGB
- J420ToABGR
- J420ToRAW
- J420ToRGB24
- J420ToRGB565
- J420ToI420

### J422
- J422ToARGB
- J422ToABGR

### J444
- J444ToARGB
- J444ToABGR

### ARGB/ABGR -> J系
- ARGBToJ400
- ARGBToJ420
- ARGBToJ422
- ARGBToJ444
- ABGRToJ400
- ABGRToJ420
- ABGRToJ422

### RAW/RGB24 -> J系
- RAWToJ400
- RAWToJ420
- RAWToJ444
- RAWToJNV21
- RGB24ToJ400
- RGB24ToJ420
- RGBAToJ400

## 解決方法

convert.rs に以下の 26 関数を追加した:

J -> 他:
- j400_to_argb
- j420_to_argb, j420_to_abgr, j420_to_raw, j420_to_rgb24, j420_to_rgb565, j420_to_i420
- j422_to_argb, j422_to_abgr
- j444_to_argb, j444_to_abgr

他 -> J:
- argb_to_j400, argb_to_j420, argb_to_j422, argb_to_j444
- abgr_to_j400, abgr_to_j420, abgr_to_j422
- raw_to_j400, raw_to_j420, raw_to_j444, raw_to_jnv21
- rgb24_to_j400, rgb24_to_j420
- rgba_to_j400
