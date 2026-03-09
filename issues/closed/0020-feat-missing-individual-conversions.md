# 不足している個別変換関数の追加

## 概要

libyuv C API に存在するがラップされていない個別変換関数を追加する。
Matrix 系関数 (issue 0016) や品質計測関数 (issue 0019) は対象外。

## 対象関数

### 変換

| 関数 | 説明 |
|---|---|
| `I422ToI210` | I422 (8bit 4:2:2) → I210 (10bit 4:2:2)。10bit パイプラインへの入口 |
| `I420ToAB30` | I420 → AB30 (10bit パックド)。`i420_to_ar30` の逆バイト順版 |
| `I422ToRGB565` | I422 → RGB565。組み込みディスプレイ向け |
| `ARGBToRGBA` | ARGB → RGBA。逆方向の `rgba_to_argb` は実装済み |

### コピー/ミラー

| 関数 | 説明 |
|---|---|
| `I400Copy` | I400 (グレースケール) コピー。`copy_plane` で代替可能だが一貫性のため |
| `I400Mirror` | I400 ミラー。`mirror_plane` で代替可能だが同上 |

### 対象外

- `RGBScale`: `scale_rgb.h` は `libyuv.h` にインクルードされておらず、公式 API ではないため対象外

## 解決方法

以下の 6 関数を追加した:

- `i420_to_ab30`: convert.rs に追加 (I420→AB30 変換)
- `i422_to_i210`: convert.rs に追加 (I422→I210 8bit→10bit 変換)
- `i422_to_rgb565`: convert.rs に追加 (I422→RGB565 変換)
- `argb_to_rgba`: convert.rs に追加 (ARGB→RGBA 変換)
- `i400_copy`: planar.rs に追加 (I400 コピー)
- `i400_mirror`: planar.rs に追加 (I400 ミラー)
