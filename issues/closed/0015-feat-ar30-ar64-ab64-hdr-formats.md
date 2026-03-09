# AR30/AR64/AB64 HDR フォーマット追加

## 概要

AR30 (10bit packed RGB), AR64/AB64 (16bit per channel) HDR フォーマットの変換関数を追加する。

## 対象関数

### AR30
- AR30ToARGB
- AR30ToABGR
- AR30ToAB30
- ARGBToAR30
- ABGRToAR30

### AR64
- AR64ToARGB
- AR64ToAB64
- ARGBToAR64

### AB64
- AB64ToARGB
- ARGBToAB64

## 解決方法

src/convert.rs に AR30/AR64/AB64 の変換関数を追加した。
