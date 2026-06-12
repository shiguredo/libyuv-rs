# libjpeg-turbo を組み込んで MJPEG 変換関数を有効化する

- Priority: High
- Created: 2026-06-12
- Completed:
- Model: qwen3.7-max
- Branch: feature/add-libjpeg-turbo-mjpeg-support
- Polished: 2026-06-12

## 目的

libyuv の MJPEG デコード関数 (`MJPGToI420`, `MJPGToNV12`, `MJPGToNV21`, `MJPGToARGB`, `MJPGSize`) は libjpeg-turbo への依存が必要だが、現在のビルドシステムでは libjpeg-turbo を組み込んでいないため、これらの関数が利用不可能な状態になっている。libjpeg-turbo をビルドシステムに統合し、MJPEG 変換関数を Rust API として公開する。

## 優先度根拠

MJPEG は WebRTC のカメラストリームで広く利用されており、これらが使えないと本クレートの利便性が大きく損なわれる。

## 現状

### ビルドシステム

- `build.rs` に libjpeg-turbo 関連の設定が一切存在しない
- libyuv の `CMakeLists.txt` は `find_package(JPEG)` で JPEG ライブラリを探索するが、`build.rs` から JPEG のパスを渡していない
- `HAVE_JPEG` マクロが定義されないため、MJPEG 関数がコンパイルされない
- prebuilt バイナリも JPEG 無効の状態でビルドされている

### Rust API

- `src/convert.rs` に MJPEG 関連の関数 (`mjpeg_to_i420` 等) が存在しない
- issue 0012 で MJPEG 関数の追加が試みられたが、libjpeg-turbo の依存がないため C++ 側の関数がコンパイルされず、bindgen も FFI 宣言を生成できなかった

## 設計方針

### libjpeg-turbo のバージョン

libjpeg-turbo 3.1.90 を使用する。

### Cargo.toml のメタデータ追加

```toml
[package.metadata.external-dependencies.libjpeg-turbo]
git = "https://github.com/libjpeg-turbo/libjpeg-turbo"
version = "3.1.90"
```

### build.rs のビルドフロー再設計

現在の `build_from_source()` は libyuv 1 つだけをビルドするフローにハードコードされている。libjpeg-turbo を追加するには以下のフローに再設計する:

1. **libjpeg-turbo のソース取得**: `git_clone_external_lib()` を汎用化し、`lib_name: &str` を引数に取るように変更する。`get_git_url_and_version()` も同様に汎用化する
2. **libjpeg-turbo のビルド**: `shiguredo_cmake::Config` を使って libjpeg-turbo を `OUT_DIR/libjpeg-turbo/` にビルドする。具体的なコマンド列:
   ```
   cmake -S OUT_DIR/source/libjpeg-turbo -B OUT_DIR/libjpeg-turbo/build \
     -DCMAKE_INSTALL_PREFIX=OUT_DIR/libjpeg-turbo/install \
     -DENABLE_STATIC=ON -DENABLE_SHARED=OFF \
     -DWITH_TURBOJPEG=OFF -DWITH_TOOLS=OFF -DWITH_TESTS=OFF
   cmake --build OUT_DIR/libjpeg-turbo/build --config Release
   cmake --install OUT_DIR/libjpeg-turbo/build --config Release
   ```
3. **libjpeg-turbo のシンボル書き換え**: `rewrite_symbols()` を libjpeg-turbo の `libjpeg.a` に対して呼び出す。プレフィックスは `shiguredo_jpeg_` を使用する。`rewrite_symbols()` のシグネチャを `fn rewrite_symbols(lib_dir: &Path, out_dir: &Path, prefix: &str) -> SymbolLinkNameCallbacks` に変更し、プレフィックスを引数で受け取る
4. **libyuv のビルド**: libjpeg-turbo のヘッダーとライブラリのパスを CMake に渡す。**重要**: libyuv のビルド時に、libjpeg-turbo のリネームマップから `-Djpeg_read_header=shiguredo_jpeg_read_header` のマクロを生成し、`CMAKE_C_FLAGS` で渡す。これによりコンパイル時にシンボル名が置換され、libyuv 内の libjpeg-turbo への未定義参照も書き換え済みになる
5. **libyuv のシンボル書き換え**: 既存の `rewrite_symbols()` を呼び出す (プレフィックスは `shiguredo_yuv_`)
6. **リンク設定**: `cargo::rustc-link-search=native=` で libjpeg-turbo のライブラリディレクトリを追加し、`cargo::rustc-link-lib=static=yuv` の後に `cargo::rustc-link-lib=static=jpeg` を追加する (リンク順序は依存元が先)

### libjpeg-turbo の CMake ビルドオプション

- `ENABLE_STATIC=ON`, `ENABLE_SHARED=OFF` で静的ライブラリのみビルド
- `WITH_TURBOJPEG=OFF` (TurboJPEG API は不要)
- `WITH_TOOLS=OFF`, `WITH_TESTS=OFF` でツールとテストをスキップ
- SIMD はプラットフォームに応じて自動判定:
  - x86_64: `Command::new("nasm").arg("--version")` で NASM の有無を検出し、失敗時は `WITH_SIMD=OFF` を渡す
  - aarch64: NASM 不要 (NEON SIMD は GAS アセンブラ)

### libyuv の CMake ビルド設定

libjpeg-turbo のビルド成果物を libyuv の CMake に渡す:

```rust
let dst = Config::new(&src_dir)
    .define("BUILD_SHARED_LIBS", "OFF")
    .define("JPEG_INCLUDE_DIR", jpeg_include_dir.display().to_string())
    .define("JPEG_LIBRARY", jpeg_lib_path.display().to_string())
    .profile("Release")
    .build();
```

`find_package(JPEG)` は `JPEG_INCLUDE_DIR` と `JPEG_LIBRARY` が事前に設定されている場合、それらを使用する。

### libjpeg-turbo の有効化方針

libjpeg-turbo は**デフォルトで有効**とする。feature フラグによるオプトイン/オプトアウトは導入しない (優先度根拠は前述の通り)。

### Rust API

`src/convert.rs` に以下の関数を追加する。MJPEG は**可変長の圧縮データ** (`&[u8]`) を入力に取るため、既存の画像型パターンとは異なるシグネチャになる。エラーハンドリングは既存の `Error` 型を使用する:

```rust
/// MJPEG データから画像サイズを取得する
pub fn mjpeg_size(src: &[u8]) -> Result<ImageSize, Error>

/// MJPEG から I420 に変換する (スケーリング非対応、src_size は mjpeg_size() で取得した値を使用)
pub fn mjpeg_to_i420(src: &[u8], dst: &mut I420ImageMut<'_>, src_size: ImageSize) -> Result<(), Error>

/// MJPEG から NV12 に変換する
pub fn mjpeg_to_nv12(src: &[u8], dst: &mut Nv12ImageMut<'_>, src_size: ImageSize) -> Result<(), Error>

/// MJPEG から NV21 に変換する
pub fn mjpeg_to_nv21(src: &[u8], dst: &mut Nv21ImageMut<'_>, src_size: ImageSize) -> Result<(), Error>

/// MJPEG から ARGB に変換する
pub fn mjpeg_to_argb(src: &[u8], dst: &mut ArgbImageMut<'_>, src_size: ImageSize) -> Result<(), Error>

/// MJPEG データの妥当性を検証する (MJPGSize() を呼び、成功したら true を返す)
pub fn mjpeg_validate(src: &[u8]) -> bool
```

### prebuilt バイナリ

- libjpeg-turbo 込みで**1 種類だけ**ビルドし、GitHub Releases にアップロードする
- JPEG あり/なしの 2 種類は用意しない（CI・リリース管理コストを削減）
- prebuilt アーカイブの構造:
  ```
  lib/
    libyuv.a (または yuv.lib)
    libjpeg.a (または jpeg-static.lib)
  bindings.rs
  ```
- `download_prebuilt()` を改修し、`libjpeg.a` (または `jpeg-static.lib`) も `OUT_DIR/lib/` にコピーする
- SHA256 チェックサムも再計算する
- 過去のリリースとの互換性: 古いバージョンの prebuilt をダウンロードしたら JPEG なしになる (後方互換性は維持)

### シンボル書き換え

- libjpeg-turbo の静的ライブラリには `shiguredo_jpeg_` プレフィックスを適用する
- libyuv の静的ライブラリには既存の `shiguredo_yuv_` プレフィックスを適用する
- ビルドフローでの書き換え順序は前述の通り

### Docs.rs ビルド対応

Docs.rs では git clone ができないため、`build.rs` はダミーの `bindings.rs` を出力している。MJPEG 関数を Rust API として公開する場合、ダミー bindings にも `MJPGSize`, `MJPGToI420`, `MJPGToNV12`, `MJPGToNV21`, `MJPGToARGB` の FFI 定義を含める必要がある。各関数の完全なシグネチャは libyuv の `include/libyuv/convert.h` を参照すること。

### Windows (MSVC) ビルド対応

- 静的ライブラリの出力名は `jpeg-static.lib`
- NASM のパスは CMake に `CMAKE_ASM_NASM_COMPILER` で渡す
- CRT のリンク方式 (`/MT` vs `/MD`) は libyuv と libjpeg-turbo で揃える必要がある

## 完了条件

- `source-build` feature 有効時に libjpeg-turbo が自動ビルドされ、MJPEG 関数が利用可能になる
- prebuilt バイナリでも MJPEG 関数が利用可能になる
- `mjpeg_size()`, `mjpeg_to_i420()`, `mjpeg_to_nv12()`, `mjpeg_to_nv21()`, `mjpeg_to_argb()`, `mjpeg_validate()` が Rust API として公開されている
- 全プラットフォーム (Linux x86_64/aarch64, macOS aarch64, Windows x86_64) でビルドが通る
- MJPEG 変換関数のテストを追加する:
  - テスト用 MJPEG データ: libjpeg-turbo の `testimages/` からサンプル JPEG を使用するか、テスト用に小さな JPEG データを生成する
  - 正常系: 有効な MJPEG データでの変換
  - 異常系: 不正な MJPEG データ、空バッファ、サイズ不一致
- 既存のテストが全て通る
- CHANGES.md の develop セクションに `[ADD]` エントリを追記する
