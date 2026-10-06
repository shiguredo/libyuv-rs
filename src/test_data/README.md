# MJPEG テストデータ

`tests/mjpeg.rs` で参照する固定 JPEG バイト列。MCU 境界 (8x8 / 8x16 / 16x16) を網羅するため、
4:2:0 / 4:2:2 / 4:4:4 の 3 つのサブサンプリングのファイルを用意している。

## 含まれるファイル

| ファイル | 幅×高さ | サブサンプリング |
|---|---|---|
| `mjpeg_8x8_yuv420.jpg` | 8x8 | 4:2:0 |
| `mjpeg_16x16_yuv422.jpg` | 16x16 | 4:2:2 |
| `mjpeg_64x48_yuv444.jpg` | 64x48 | 4:4:4 |

## 再生成手順

ImageMagick と jpegtran が必要。

```bash
cd src/test_data/
magick -size 8x8   gradient:red-blue -sampling-factor 2x2 -quality 80 mjpeg_8x8_yuv420.jpg
magick -size 16x16 gradient:red-blue -sampling-factor 2x1 -quality 80 mjpeg_16x16_yuv422.jpg
magick -size 64x48 gradient:red-blue -sampling-factor 1x1 -quality 80 mjpeg_64x48_yuv444.jpg

# メタデータを削除 (バイト列を最小化)
for f in mjpeg_8x8_yuv420.jpg mjpeg_16x16_yuv422.jpg mjpeg_64x48_yuv444.jpg; do
  jpegtran -copy none "$f" > "$f.tmp" && /bin/mv -f "$f.tmp" "$f"
done
```

ImageMagick の `-sampling-factor` は「水平×垂直のサブサンプリング比」を指定する形式:

- `2x2` → YUV 4:2:0
- `2x1` → YUV 4:2:2
- `1x1` → YUV 4:4:4

CI からは再生成しない。差し替えが必要になったときに上記手順を手動で実行する。
