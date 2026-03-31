# 追加 NV12/NV21 変換追加

## 概要

NV12/NV21 の追加変換関数を追加する。

## 対象関数

### NV12
- NV12ToRAW
- NV12ToRGB565
- NV12ToI420Rotate
- NV12ToNV24
- NV21Copy

### NV21
- NV21ToRAW
- NV21ToYUV24

### NV16/NV24
- NV16ToNV24
- NV24Scale

### ABGR -> NV12/NV21
- ABGRToNV12
- ABGRToNV21

## 解決方法

convert.rs に以下の関数を追加した:

- nv12_to_raw, nv12_to_rgb565, nv12_to_nv24
- nv16_to_nv24
- nv21_to_raw, nv21_to_yuv24
- abgr_to_nv12, abgr_to_nv21

NV12ToI420Rotate は issue 0012 で既に追加済み。
NV24Scale は issue 0007 で追加済み。
NV21Copy は issue 0009 で追加済み。
Matrix 系バリアントは issue 0016 に移譲した。
