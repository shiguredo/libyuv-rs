# 追加プレーン操作追加

## 概要

追加のプレーン操作関数を追加する。

## 対象関数

### コピー
- I422Copy
- I444Copy
- I010Copy
- I210Copy
- I410Copy
- NV21Copy
- CopyPlane_16

### プレーン操作
- MirrorUVPlane
- HalfMergeUVPlane
- SplitARGBPlane
- MergeARGBPlane

### 16bit プレーン
- SplitUVPlane_16
- MergeUVPlane_16
- InterpolatePlane_16
- Convert16To8Plane
- Convert8To16Plane
- Convert8To8Plane
- ConvertToLSBPlane_16
- ConvertToMSBPlane_16

### AR64/ARGB16 プレーン
- MergeAR64Plane
- MergeARGB16To8Plane
- MergeXR30Plane
- HalfFloatPlane
- ByteToFloat
- GaussPlane_F32

## 解決方法

src/planar.rs に 16bit プレーン操作、ARGB プレーン分割・結合、ビット深度変換、フロート変換、ガウスフィルタ等の関数を追加した。src/convert.rs に I422Copy, I444Copy, NV21Copy を追加した。
