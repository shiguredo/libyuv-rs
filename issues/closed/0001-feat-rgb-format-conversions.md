# RGB フォーマット変換追加

## 概要

RGBA, BGRA, RAW, RGB565, ARGB1555, ARGB4444 フォーマットの変換関数を追加する。

## 対象関数

### RGBA
- RGBAToARGB
- RGBAToI420
- I420ToRGBA
- I422ToRGBA

### BGRA
- BGRAToARGB
- BGRAToI420
- I420ToBGRA
- I422ToBGRA
- ARGBToBGRA

### RAW (RGB packed, BGR order)
- RAWToARGB
- RAWToI420
- RAWToRGB24
- RAWToRGBA
- ARGBToRAW
- I420ToRAW
- I422ToRAW
- I444ToRAW
- NV12ToRAW
- NV21ToRAW
- RAWToI444

### RGB565
- RGB565ToARGB
- RGB565ToI420
- ARGBToRGB565
- ARGBToRGB565Dither
- I420ToRGB565
- I420ToRGB565Dither

### ARGB1555
- ARGB1555ToARGB
- ARGB1555ToI420
- ARGBToARGB1555
- I420ToARGB1555

### ARGB4444
- ARGB4444ToARGB
- ARGB4444ToI420
- ARGBToARGB4444
- I420ToARGB4444

## 解決方法

convert.rs に以下の 32 関数を追加した:

RGBA: rgba_to_argb, rgba_to_i420, i420_to_rgba, i422_to_rgba
BGRA: bgra_to_argb, bgra_to_i420, i420_to_bgra, i422_to_bgra, argb_to_bgra
RAW: raw_to_argb, raw_to_i420, raw_to_rgb24, raw_to_rgba, argb_to_raw, i420_to_raw, i422_to_raw, i444_to_raw, raw_to_i444
RGB565: rgb565_to_argb, rgb565_to_i420, argb_to_rgb565, argb_to_rgb565_dither, i420_to_rgb565, i420_to_rgb565_dither
ARGB1555: argb1555_to_argb, argb1555_to_i420, argb_to_argb1555, i420_to_argb1555
ARGB4444: argb4444_to_argb, argb4444_to_i420, argb_to_argb4444, i420_to_argb4444

NV12ToRAW, NV21ToRAW は issue 0006 で実装済み。
