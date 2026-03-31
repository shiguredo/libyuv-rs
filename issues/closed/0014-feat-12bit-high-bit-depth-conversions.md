# 12bit 高ビット深度変換追加

## 概要

12bit 高ビット深度フォーマットの変換関数を追加する。

## 対象関数

### I012 (12bit I420)
- I012ToI420
- I012ToP012
- I420ToI012

### I212 (12bit I422)
- I212ToI420
- I212ToI422
- I212ToP212

### I412 (12bit I444)
- I412ToI420
- I412ToI444

### P012 (packed 12bit)
- P012ToI012

## 解決方法

src/convert.rs に 12bit 高ビット深度変換関数を追加した。
