# libyuv-rs

[![crates.io](https://img.shields.io/crates/v/shiguredo_libyuv.svg)](https://crates.io/crates/shiguredo_libyuv)
[![docs.rs](https://docs.rs/shiguredo_libyuv/badge.svg)](https://docs.rs/shiguredo_libyuv)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![GitHub Actions](https://github.com/shiguredo/libyuv-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shiguredo/libyuv-rs/actions/workflows/ci.yml)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/shiguredo)

## About Shiguredo's open source software

We will not respond to PRs or issues that have not been discussed on Discord. Also, Discord is only available in Japanese.

Please read <https://github.com/shiguredo/oss> before use.

## 時雨堂のオープンソースソフトウェアについて

利用前に <https://github.com/shiguredo/oss> をお読みください。

## 概要

Google の [libyuv](https://chromium.googlesource.com/libyuv/libyuv/) を利用した画像変換・処理ライブラリの Rust バインディングです。

ビルド時に libyuv のソースコードを git clone し、CMake でビルドした上で bindgen でバインディングを自動生成します。

## 特徴

- prebuilt バイナリによる高速ビルド (デフォルト)
- ソースからのビルドも可能 (`--features source-build`)
- YUV / RGB 間のフォーマット変換 (244 関数)
- 画像のスケーリング (リサイズ)
- 画像の回転 (0 / 90 / 180 / 270 度)
- 画像の左右反転 (ミラー)
- 画像の品質比較 (PSNR / SSIM)
- アルファブレンド・補間
- プレーン操作 (コピー / 塗りつぶし / 分割 / 結合)
- ARGB 加工 (グレースケール / セピア / シェーディング / アルファ事前乗算)
- 矩形描画
- 静的ライブラリ (libyuv / libjpeg-turbo) のシンボルを `shiguredo_yuv_` / `shiguredo_jpeg_` プレフィックス付きに書き換えて他ライブラリ (例: 別クレートが組み込む libjpeg) との衝突を回避
- 高ビット深度 (10bit / 12bit / 16bit) 対応
- HDR フォーマット (AR30 / AR64 / AB64) 対応
- ハードウェアフォーマット (Android420 / MM21 / MT2T) 対応

## 動作要件

- Ubuntu 26.04 x86_64
- Ubuntu 26.04 arm64
- Ubuntu 24.04 x86_64
- Ubuntu 24.04 arm64
- Ubuntu 22.04 x86_64
- Ubuntu 22.04 arm64
- macOS 26 arm64
- macOS 15 arm64
- Windows Server 2025 x86_64
- Windows 11 x86_64
- iOS 13.0 以降 (実機 arm64)
- iOS 14.0 以降 (シミュレーター arm64)
- Android API level 21 以降 (arm64-v8a / x86_64)

## 対応フォーマット

### YUV (3 プレーン)

| フォーマット | クロマサブサンプリング | 備考 |
|---|---|---|
| I420 | 4:2:0 | 最も一般的 |
| I422 | 4:2:2 | |
| I444 | 4:4:4 | |
| J420 / J422 / J444 | JPEG フルレンジ | |
| I400 | グレースケール (Y のみ) | |

### YUV (2 プレーン)

| フォーマット | クロマ順序 | 備考 |
|---|---|---|
| NV12 | UV | H.264 / H.265 デコーダー出力で一般的 |
| NV21 | VU | Android カメラ出力で一般的 |
| NV24 | UV (4:4:4) | |

### RGB

| フォーマット | ピクセルサイズ |
|---|---|
| ARGB | 4 bytes |
| ABGR | 4 bytes |
| RGBA | 4 bytes |
| BGRA | 4 bytes |
| RGB24 | 3 bytes |
| RAW | 3 bytes |
| RGB565 | 2 bytes |
| ARGB1555 | 2 bytes |
| ARGB4444 | 2 bytes |

### パック YUV

| フォーマット | ピクセルサイズ | 備考 |
|---|---|---|
| YUY2 (YUYV) | 2 bytes | 4:2:2 パック |
| UYVY | 2 bytes | 4:2:2 パック |

### 高ビット深度

| フォーマット | ビット深度 |
|---|---|
| I010 / I210 / I410 | 10bit |
| I012 / I212 / I412 | 12bit |
| P010 / P012 / P016 / P210 / P410 | 10-16bit |
| AR30 / AB30 | 10bit packed |
| AR64 / AB64 | 16bit |

## ビルド

デフォルトでは GitHub Releases から prebuilt バイナリをダウンロードしてビルドします。

```bash
cargo build
```

### iOS / Android 向け prebuilt

Cargo のターゲットに応じて、以下のアーカイブを自動選択します。
各アーカイブにはシンボル書き換え済みの `lib/libshiguredo_yuv.a`、`lib/libshiguredo_jpeg.a`、`bindings.rs`、`THIRD_PARTY_LICENSES` を収録し、SHA256 チェックサムを添付します。

| 対象 | Rust ターゲット | アーカイブ名 |
| --- | --- | --- |
| iOS 実機 arm64 | `aarch64-apple-ios` | `libyuv-ios_arm64.tar.gz` |
| iOS シミュレーター arm64 | `aarch64-apple-ios-sim` | `libyuv-ios-sim_arm64.tar.gz` |
| Android arm64-v8a | `aarch64-linux-android` | `libyuv-android_arm64.tar.gz` |
| Android x86_64 | `x86_64-linux-android` | `libyuv-android_x86_64.tar.gz` |

prebuilt の対象は iOS 実機が 13.0 以降、iOS シミュレーターが 14.0 以降、Android が API level 21 以降です。
モバイル向けの成果物は、対応を追加したバージョンの GitHub Release から提供します。

```bash
rustup target add aarch64-apple-ios
IPHONEOS_DEPLOYMENT_TARGET=13.0 cargo build --target aarch64-apple-ios
```

アプリケーションのリンクには、iOS では Xcode と各ターゲットの下限以上の `IPHONEOS_DEPLOYMENT_TARGET` 設定、Android では Android NDK と対象 ABI のリンカー設定が必要です。
[iOS の Rust ターゲット](https://doc.rust-lang.org/rustc/platform-support/apple-ios.html) と [Android NDK の CMake ガイド](https://developer.android.com/ndk/guides/cmake) も参照してください。

### ソースからビルド

libyuv をソースからビルドする場合は `source-build` feature を有効にしてください。

```bash
cargo build --features source-build
```

ソースビルドには以下が必要です:

- Git
- C / C++ コンパイラ
- rustup の `llvm-tools` コンポーネント (シンボル書き換えに使用)
- NASM (Android x86_64 の libjpeg-turbo ビルドに必要)

```bash
rustup component add llvm-tools
```

iOS では、Xcode の SDK を使って実機とシミュレーターをビルドします。
最小バージョンは `IPHONEOS_DEPLOYMENT_TARGET` で指定できます。
未指定の場合は実機が `13.0`、arm64 シミュレーターが `14.0` です。

```bash
IPHONEOS_DEPLOYMENT_TARGET=13.0 cargo build --target aarch64-apple-ios --features source-build
IPHONEOS_DEPLOYMENT_TARGET=14.0 cargo build --target aarch64-apple-ios-sim --features source-build
```

Android では、`ANDROID_NDK_HOME` に NDK のディレクトリを指定します。
`ANDROID_PLATFORM` で最小 API level を数値または `android-<数値>` の形式で指定でき、未指定の場合は `21` です。
指定できる API level は `21` 以降です。
以下は Linux ホストでの arm64-v8a 向けの例です。

```bash
export ANDROID_NDK_HOME=/path/to/android-ndk
export ANDROID_PLATFORM=21
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
rustup target add aarch64-linux-android
cargo build --target aarch64-linux-android --features source-build
```

リリース用の Android prebuilt は NDK `28.2.13676358` でビルドします。
CI では、すべてのモバイルターゲットでソースビルド、シンボル書き換え、Rust のリンク、アーカイブ生成を検証します。

### docs.rs 向けビルド

libyuv がない環境では、docs.rs 向けのドキュメント生成のみ可能です。

```bash
DOCS_RS=1 cargo doc --no-deps
```

### 環境変数

| 環境変数 | 説明 |
|---|---|
| `LIBYUV_TARGET` | prebuilt バイナリのプラットフォーム名を明示的に指定する |
| `IPHONEOS_DEPLOYMENT_TARGET` | iOS ソースビルドの最小 OS バージョン (未指定時は実機 `13.0`、arm64 シミュレーター `14.0`) |
| `ANDROID_NDK_HOME` | Android ソースビルドで使用する NDK のディレクトリ |
| `ANDROID_PLATFORM` | Android ソースビルドの最小 API level (未指定時は `21`) |

## 使い方

### フォーマット変換

```rust
use shiguredo_libyuv::{i420_to_argb, ArgbImageMut, I420Image, ImageSize};

let size = ImageSize::new(640, 480);

let src = I420Image {
    y: &y_plane,
    y_stride: 640,
    u: &u_plane,
    u_stride: 320,
    v: &v_plane,
    v_stride: 320,
};

let mut argb_buf = vec![0u8; 640 * 480 * 4];
let mut dst = ArgbImageMut {
    data: &mut argb_buf,
    stride: 640 * 4,
};

i420_to_argb(&src, &mut dst, size)?;
```

### スケーリング

```rust
use shiguredo_libyuv::{i420_scale, FilterMode, ImageSize};

let src_size = ImageSize::new(1920, 1080);
let dst_size = ImageSize::new(640, 360);

i420_scale(&src, src_size, &mut dst, dst_size, FilterMode::Bilinear)?;
```

### 回転

```rust
use shiguredo_libyuv::{i420_rotate, ImageSize, RotationMode};

let src_size = ImageSize::new(640, 480);
let dst_size = ImageSize::new(480, 640); // 90 度回転で幅と高さが入れ替わる

i420_rotate(&src, src_size, &mut dst, dst_size, RotationMode::Rotate90)?;
```

### 品質比較

```rust
use shiguredo_libyuv::{i420_psnr, i420_ssim, ImageSize};

let size = ImageSize::new(640, 480);

let psnr = i420_psnr(&original, &decoded, size)?;
let ssim = i420_ssim(&original, &decoded, size)?;
```

### NV12 から I420 への変換

```rust
use shiguredo_libyuv::{nv12_to_i420, ImageSize, Nv12Image};

let size = ImageSize::new(640, 480);

let src = Nv12Image {
    y: &y_plane,
    y_stride: 640,
    uv: &uv_plane,
    uv_stride: 640,
};

nv12_to_i420(&src, &mut dst, size)?;
```

### アルファチャンネル付き変換

```rust
use shiguredo_libyuv::{i420_alpha_to_argb, ImageSize};

let size = ImageSize::new(640, 480);

// attenuate: アルファ事前乗算を行うかどうか
i420_alpha_to_argb(&src, &alpha_plane, 640, &mut dst, size, true)?;
```

## 設定

### `FilterMode`

| バリアント | 説明 |
|---|---|
| `FilterMode::None` | フィルタなし (最速、最低品質) |
| `FilterMode::Linear` | 線形フィルタ (高速、適度な品質) |
| `FilterMode::Bilinear` | バイリニア (中程度) |
| `FilterMode::Box` | ボックスフィルタ (ダウンスケール時に有効) |

### `RotationMode`

| バリアント | 説明 |
|---|---|
| `RotationMode::None` | 回転なし (0 度) |
| `RotationMode::Rotate90` | 時計回り 90 度 |
| `RotationMode::Rotate180` | 180 度 |
| `RotationMode::Rotate270` | 時計回り 270 度 (反時計回り 90 度) |

### 画像バッファ型

フォーマットごとに専用の型を提供しています。各型は `Image` (読み取り専用) と `ImageMut` (書き込み可能) のペアです。

| 型の例 | フィールド | 用途 |
|---|---|---|
| `I420Image` / `I420ImageMut` | y, u, v + stride | 3 プレーン YUV 4:2:0 |
| `I422Image` / `I422ImageMut` | y, u, v + stride | 3 プレーン YUV 4:2:2 |
| `Nv12Image` / `Nv12ImageMut` | y, uv + stride | 2 プレーン NV12 |
| `ArgbImage` / `ArgbImageMut` | data, stride | パック ARGB |
| `I010Image` / `I010ImageMut` | y, u, v + stride (u16) | 10bit 3 プレーン YUV |
| `P010Image` / `P010ImageMut` | y, uv + stride (u16) | 10bit 2 プレーン |
| `Ar64Image` / `Ar64ImageMut` | data, stride (u16) | 16bit パック |

その他: `I444Image`, `J420Image`, `H420Image`, `Nv21Image`, `AbgrImage`, `Rgb24Image`, `I400Image` など全フォーマットに対応する型があります。

## モジュール構成

| モジュール | 関数数 | 内容 |
|---|---|---|
| `convert` | 244 | フォーマット変換 |
| `planar` | 67 | プレーン操作・ARGB 加工 |
| `rotate` | 20 | 回転・転置 |
| `scale` | 18 | スケーリング |
| `compare` | 9 | 品質比較 (PSNR / SSIM) |

## ライセンス

Apache License 2.0

```text
Copyright 2026-2026, Shiguredo Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
