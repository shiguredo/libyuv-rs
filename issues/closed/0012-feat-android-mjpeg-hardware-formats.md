# Android/MJPEG/ハードウェアフォーマット変換追加

## 概要

Android フォーマット、MJPEG デコード、MM21/MT2T 等のハードウェアフォーマット変換関数を追加する。

## 対象関数

### Android
- Android420ToARGB
- Android420ToABGR
- Android420ToI420
- Android420ToI420Rotate

### MJPEG
- MJPGToARGB
- MJPGToI420
- MJPGToNV12
- MJPGToNV21
- MJPGSize

### MM21
- MM21ToI420
- MM21ToNV12
- MM21ToYUY2

### MT2T
- MT2TToP010

### AYUV
- AYUVToNV12
- AYUVToNV21

### Detile
- DetilePlane
- DetilePlane_16
- DetileSplitUVPlane
- DetileToYUY2

## 解決方法

src/convert.rs に Android420、MJPEG、MM21、MT2T、AYUV、Detile 関連の変換関数を追加した。src/rotate.rs に Android420ToI420Rotate と NV12ToI420Rotate を追加した。
