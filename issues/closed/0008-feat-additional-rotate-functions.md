# 追加回転関数追加

## 概要

I422, I444 等の追加回転関数を追加する。

## 対象関数

- I422Rotate
- I444Rotate
- RotatePlane90
- RotatePlane180
- RotatePlane270
- RotatePlane_16
- SplitRotateUV
- SplitRotateUV90
- SplitRotateUV180
- SplitRotateUV270
- SplitTransposeUV
- TransposePlane

## 解決方法

src/rotate.rs に I422/I444 回転、16bit プレーン回転、固定角度回転、UV 分割回転、転置関数を追加した。
