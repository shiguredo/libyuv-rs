# iOS / Android 向けの prebuilt を追加する

- Created: 2026-10-03
- Completed: 2026-10-03
- Branch: feature/add-mobile-prebuilt
- Polished: {YYYY-MM-DD}

## 目的

モバイル向けの Rust アプリケーションでも、libyuv と libjpeg-turbo をソースからビルドせずに prebuilt で利用できるようにする。
ユーザーからの「opus-rs / aom-rs / dav1d-rs と同様に iOS / Android 向けの prebuilt を用意したい」という要望に対応する。

## 現状

- `build.rs` の `get_target_platform` は Linux、macOS、Windows 向けのみを扱い、iOS / Android のターゲットを指定すると panic する
- `build.rs` の `rename_defined_symbols` は `is_target_macos` で Mach-O のシンボル先頭 `_` を判定しており、iOS を追加するとシンボル書き換えが壊れる
- `build.rs` の `build_from_source` に iOS の Xcode SDK と Android NDK を使う設定がなく、モバイル向けのソースビルドができない
- `main()` の C++ 標準ライブラリのリンクは Linux / macOS / iOS のみを扱い、libyuv が C++ コードを含む Android 向けのリンク設定がない
- `.github/workflows/ci.yml` と `.github/workflows/release.yml` にモバイル向けのビルドジョブがない
- `README.md` に iOS / Android 向けのビルド手順と prebuilt の記述がない

## 設計方針

### 対象ターゲット

| 対象 | Rust ターゲット | prebuilt のプラットフォーム名 |
| --- | --- | --- |
| iOS 実機 arm64 | `aarch64-apple-ios` | `ios_arm64` |
| iOS シミュレーター arm64 | `aarch64-apple-ios-sim` | `ios-sim_arm64` |
| Android arm64-v8a | `aarch64-linux-android` | `android_arm64` |
| Android x86_64 | `x86_64-linux-android` | `android_x86_64` |

### ビルド

- iOS の prebuilt は実機が iOS 13.0 以降、arm64 シミュレーターが iOS 14.0 以降を対象とする
- Android の prebuilt は arm64-v8a と x86_64 の 2 ABI、API level 21 以降を対象とし、NDK `28.2.13676358` でビルドする
- iOS では `xcrun` で Xcode SDK のパスを解決し、CMake と bindgen に同じ SDK、アーキテクチャ、最小 OS バージョンを指定する
  - libyuv と libjpeg-turbo の両方を同じ設定でビルドし、libyuv の `find_package(JPEG)` には libjpeg-turbo の install prefix を `CMAKE_PREFIX_PATH` で渡す
  - cc クレートの既定フラグと CMake の SDK 選択が競合しないよう、コンパイラのターゲット設定は CMake に任せる
- Android では NDK の `android.toolchain.cmake` を使い、`ANDROID_ABI`、`ANDROID_PLATFORM`、`ANDROID_STL=c++_static` を指定する
  - libyuv は C++ コードを含むため、Rust 側でも Android 向けに C++ 標準ライブラリをリンクする
  - cmake クレートのコンパイラ判定に NDK の C / C++ コンパイラを指定できるよう、`build-dependencies` に `cc` を追加する
  - `ANDROID_PLATFORM` は数値または `android-<数値>` を受け入れ、21 未満を拒否する
- Mach-O のシンボル書き換えは Apple プラットフォーム判定 (`CARGO_CFG_TARGET_VENDOR == "apple"`) に変更して iOS でも動くようにする

### 配布と検証

- `.github/workflows/mobile.yml` を追加し、CI とリリースから共用する
- ワークフローではソースビルド、Rust のリンク、シンボルのプレフィックス検証、アーカイブ生成、SHA256 検証、展開物の一致確認を行う
- シンボル検証は既存の `scripts/verify_symbol_rewrite.sh` を再利用し、libyuv と libjpeg-turbo の両方を対象にする
- アーカイブにはシンボル書き換え済みの `lib/libshiguredo_yuv.a`、`lib/libshiguredo_jpeg.a`、`bindings.rs`、`THIRD_PARTY_LICENSES` を収録する
- リリースではアップロード後に prebuilt の自動選択とリンクを検証し、成功を `publish` の前提にする

## 完了条件

- 全 4 ターゲットで静的ライブラリ 2 種とバインディングを生成できる
- 全定義済み外部シンボルが、Mach-O 固有の先頭 `_` を除いて `shiguredo_yuv_` / `shiguredo_jpeg_` プレフィックスを持つ
- 各アーカイブの SHA256 が一致し、収録したライブラリとバインディングを用いて Rust のリンクが成功する
- GitHub Actions の CI でモバイル向けビルドとリンクの検証が通る
- 次回リリースでアーカイブとチェックサムのアップロード、公開された prebuilt の自動選択とリンクの検証、crates.io への公開の順に進むワークフローを構成し、検証に失敗した場合は `publish` を開始しない
- 既存の単体テスト、PBT、フォーマット、Clippy が通る

## 解決方法

`build.rs` に `configure_mobile_build` を追加し、iOS では `xcrun` で解決した Xcode SDK、Android では `ANDROID_NDK_HOME` の NDK ツールチェーンを使って libyuv と libjpeg-turbo をビルドするようにした。

- iOS は cmake クレートが自動設定する `CMAKE_SYSTEM_NAME=iOS` と `CMAKE_OSX_SYSROOT` / `CMAKE_OSX_ARCHITECTURES` / `CMAKE_OSX_DEPLOYMENT_TARGET` で実機とシミュレーターを分ける。最小バージョンは `IPHONEOS_DEPLOYMENT_TARGET` で指定でき、既定は実機 `13.0`、arm64 シミュレーター `14.0` とする
- Android は NDK の `android.toolchain.cmake` と NDK の clang / clang++ を指定し、`ANDROID_ABI`、`ANDROID_PLATFORM`、`ANDROID_STL=c++_static` を設定する。`ANDROID_PLATFORM` は数値または `android-<数値>` を受け付け、21 未満を拒否する
- モバイルでは libyuv の `find_package(JPEG)` が install prefix を SDK 配下に再ルートして失敗するため、`JPEG_INCLUDE_DIR` / `JPEG_LIBRARY` を直接指定する
- Android の C++ 標準ライブラリは NDK の `libc++.a` リンカスクリプト (`INPUT(-lc++_static -lc++abi)`) を `static` 指定で解決する
- `get_target_platform` にモバイル 4 ターゲットを `TARGET` の完全一致で追加し、Mach-O のシンボル書き換えは `CARGO_CFG_TARGET_VENDOR == "apple"` の判定に変更した
- bindgen には CMake と同じ SDK / ABI の clang 引数を渡す
- `.github/workflows/mobile.yml` を追加して CI とリリースで共用し、`ci.yml` の `mobile` ジョブと `release.yml` の `build-mobile-prebuilt` ジョブから呼び出す
- `build-dependencies` に `cc` を追加し、README に iOS / Android 向けの prebuilt とビルド手順、CHANGES に [ADD] を記載した
- `rust-toolchain.toml` の `channel` を `rust-version` と同じ `1.93` に揃え、CI の `rustup update stable` を `rustup show` に置き換えた

### 検証結果

- iOS 実機 arm64 と arm64 シミュレーターでソースビルド、テスト実行ファイルのリンク、`scripts/verify_symbol_rewrite.sh`、全定義シンボルの `shiguredo_yuv_` / `shiguredo_jpeg_` プレフィックス検証、アーカイブの SHA256 と展開物一致を確認した
- Android arm64-v8a と x86_64 は GitHub Actions の CI でソースビルド、リンク、シンボル検証、アーカイブ検証を確認した
- ホストの全テスト、Clippy、フォーマット、prek のフックが通過した
- Rust 1.99 で追加された `suspicious_runtime_symbol_definitions` が bindgen 生成バインディングの `malloc` / `realloc` 宣言を検出したため、`libyuv/row.h` が取り込む `stdlib.h` 由来の両宣言を blocklist に追加した (クレートからは呼ばない)
- iOS x86_64 シミュレーターは aom-rs / dav1d-rs の最新構成に合わせて対象外とした
