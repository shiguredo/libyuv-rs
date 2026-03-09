# 追加 ARGB 操作追加

## 概要

ARGB 画像の追加操作関数を追加する。

## 対象関数

### エッジ検出
- ARGBSobel
- ARGBSobelToPlane
- ARGBSobelXY

### 色操作
- ARGBColorMatrix
- ARGBColorTable
- ARGBLumaColorTable
- ARGBQuantize
- RGBColorMatrix
- RGBColorTable

### 合成
- ARGBAdd
- ARGBSubtract
- ARGBMultiply

### フィルタ
- ARGBBlur
- ARGBComputeCumulativeSum

### シャッフル
- ARGBShuffle
- AR64Shuffle

### 検出
- ARGBDetect

### ポリノミアル
- ARGBPolynomial

### グレースケール変換
- ARGBGrayTo
- ARGBCopyAlpha
- ARGBCopyYToAlpha
- ARGBExtractAlpha

## 解決方法

src/planar.rs に Sobel フィルタ、カラーマトリックス、算術演算、ブラー、量子化、色テーブル、チャンネル操作、フォーマット検出等の ARGB 操作関数を追加した。
