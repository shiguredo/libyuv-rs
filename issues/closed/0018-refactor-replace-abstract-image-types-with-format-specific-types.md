# 抽象的な画像バッファ型を具体的なフォーマット名の型に置き換える

## 概要

現状の画像バッファ型 (`PlanarImage`, `BiplanarImage`, `PackedImage` 等) は抽象的な名前で、
異なるフォーマットを同じ型で共有している。
libyuv の C API は関数名もパラメータ名も具体的なフォーマット名を使っており、
抽象化するメリットがない。

## 現状

| 抽象型 | 共有しているフォーマット |
|---|---|
| `PlanarImage` / `PlanarImageMut` | I420, I422, I444, J420, J422, J444, I400 |
| `BiplanarImage` / `BiplanarImageMut` | NV12, NV21, NV24 |
| `PackedImage` / `PackedImageMut` | ARGB, ABGR, RGBA, BGRA, RGB24, RAW, RGB565, YUY2, UYVY 等 |
| `PlanarImage16` / `PlanarImageMut16` | I010, I210, I410, I012, I212, I412 |
| `BiplanarImage16` / `BiplanarImageMut16` | P010, P210, P410 等 |
| `PackedImage16` / `PackedImageMut16` | AR64, AB64 |

## 問題

1. **型レベルでフォーマットを区別できない** — NV12 と NV21 を取り違えてもコンパイルエラーにならない
2. **フィールド名が曖昧** — `chroma` より `uv` の方が libyuv の `src_uv` と対応が明確
3. **型名からフォーマットを推測できない** — `BiplanarImage` は何のフォーマットか分からない
4. **libyuv の C API と乖離している** — C API は `NV12ToI420(src_y, src_stride_y, src_uv, src_stride_uv, ...)` のように具体的

## 方針

抽象的な型を廃止し、フォーマットごとに具体的な型を定義する。

### 8bit 3 プレーン YUV

```rust
// I420, I422, I444 はメモリレイアウトが同じ (Y + U + V) なので共通型でよいが、
// クロマサブサンプリングが異なるため分ける価値がある
pub struct I420Image<'a> {
    pub y: &'a [u8],
    pub y_stride: usize,
    pub u: &'a [u8],
    pub u_stride: usize,
    pub v: &'a [u8],
    pub v_stride: usize,
}
// I420ImageMut, I422Image, I422ImageMut, I444Image, I444ImageMut, ...
```

### 8bit 2 プレーン YUV

```rust
pub struct Nv12Image<'a> {
    pub y: &'a [u8],
    pub y_stride: usize,
    pub uv: &'a [u8],       // chroma ではなく uv (libyuv の src_uv に合わせる)
    pub uv_stride: usize,   // chroma_stride ではなく uv_stride
}
// Nv12ImageMut, Nv21Image, Nv21ImageMut, Nv24Image, ...
```

### 8bit パック

```rust
pub struct ArgbImage<'a> {
    pub data: &'a [u8],
    pub stride: usize,
}
// ArgbImageMut, Rgb24Image, Rgb24ImageMut, ...
```

### 16bit も同様

```rust
pub struct I010Image<'a> { ... }
pub struct P010Image<'a> { ... }
pub struct Ar64Image<'a> { ... }
```

## 検討事項

- 型の数が大幅に増える（現状 12 型 → 数十型）
- 破壊的変更になるため、メジャーバージョンアップが必要
- マクロで定義を生成するかどうか
- フォーマットごとに型を分けると同じフィールドの型定義が繰り返されるが、抽象化のデメリットの方が大きい

## 解決方法

マクロベースの型生成で全フォーマットに対応する具体的な型を定義した。

### 変更内容

1. **lib.rs**: 抽象型 (`PlanarImage`, `BiplanarImage`, `PackedImage` 等 12 型) を全て削除し、7 つのマクロ (`define_yuv_image!`, `define_y_image!`, `define_nv_image!`, `define_packed_image!`, `define_yuv_image16!`, `define_nv_image16!`, `define_packed_image16!`) で 108 のフォーマット固有型を生成するように変更
2. **convert.rs**: 全 240+ 関数の引数型を具体的な型に変更 (例: `PlanarImage` → `I420Image`, `BiplanarImage` → `Nv12Image`)
3. **planar.rs**: 全関数の引数型を具体的な型に変更
4. **rotate.rs**: 全関数の引数型を具体的な型に変更
5. **scale.rs**: 全関数の引数型を具体的な型に変更
6. **compare.rs**: 全関数の引数型を具体的な型に変更
7. **PBT テスト**: 全ファイルの型参照を新しい型名に更新
8. **README.md**: コード例と画像バッファ型の表を更新

### フィールド名の変更

- BiplanarImage の `chroma` → `uv`, `chroma_stride` → `uv_stride` (libyuv の C API に合わせた)
- I400/J400 の `data` → `y`, `stride` → `y_stride`

### バリデーション

- `validate_planar_src()` 等のフリー関数を廃止し、各型の `validate()` メソッドに統合
