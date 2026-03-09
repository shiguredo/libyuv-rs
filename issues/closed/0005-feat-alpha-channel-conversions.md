# アルファチャンネル付き変換追加

## 概要

アルファプレーン付き YUV から ARGB/ABGR への変換関数を追加する。

## 対象関数

### I420Alpha
- I420AlphaToARGB
- I420AlphaToABGR
- ARGBToI420Alpha

### I422Alpha
- I422AlphaToARGB
- I422AlphaToABGR

### I444Alpha
- I444AlphaToARGB
- I444AlphaToABGR

### ARGB アルファ操作
- ARGBCopyAlpha
- ARGBCopyYToAlpha
- ARGBExtractAlpha

## 解決方法

convert.rs に以下の 7 関数を追加した:

- i420_alpha_to_argb, i420_alpha_to_abgr
- i422_alpha_to_argb, i422_alpha_to_abgr
- i444_alpha_to_argb, i444_alpha_to_abgr
- argb_to_i420_alpha

ARGBCopyAlpha, ARGBCopyYToAlpha, ARGBExtractAlpha は issue 0010 で planar.rs に実装済み。
Matrix 系バリアント (I420AlphaToARGBMatrix 等) は issue 0016 に移譲した。
