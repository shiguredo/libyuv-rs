# 変更履歴

- UPDATE
  - 後方互換がある変更
- ADD
  - 後方互換がある追加
- CHANGE
  - 後方互換のない変更
- FIX
  - バグ修正

## develop

- [CHANGE] build.rs を複数の外部ライブラリに対応できるよう汎用化する
  - `LIB_NAME` / `LINK_NAME` / `SYMBOL_PREFIX` 定数を `LibraryConfig` 構造体に置き換える
  - `git_clone_external_lib` / `get_git_url_and_version` / `rewrite_symbols` / `find_static_library` を汎用化する
  - @voluntas
- [CHANGE] prebuilt アーカイブの構造を変更する
  - `lib/` 配下のファイル名を `libshiguredo_yuv.a` (Windows: `shiguredo_yuv.lib`) / `libshiguredo_jpeg.a` (Windows: `shiguredo_jpeg.lib`) に変更する
  - `THIRD_PARTY_LICENSES` をアーカイブに同梱する
  - 旧バージョン prebuilt との互換性は無いため、リリース時に `Cargo.toml` の version を bump する
  - @voluntas
- [ADD] libjpeg-turbo をビルド依存として組み込む
  - libjpeg-turbo 3.1.90 (commit `e1dbfa7be7b7e54922020051dc77781e92739700`) を build.rs から自動ビルドする
  - 静的ライブラリのシンボルに `shiguredo_jpeg_` プレフィックスを付与する
  - @voluntas
- [ADD] MJPEG 変換関数を追加する
  - `mjpeg_size` / `mjpeg_to_i420` / `mjpeg_to_nv12` / `mjpeg_to_nv21` / `mjpeg_to_argb`
  - スケーリング非対応 (`src_size == dst_size` を強制)
  - @voluntas
- [UPDATE] `libyuv` のハッシュを `d23308a2a7442be8e559b1b471862fd7588d6a57` に更新する
  - <https://chromium.googlesource.com/libyuv/libyuv/+/d23308a2a7442be8e559b1b471862fd7588d6a57>
  - @voluntas

### misc

- [ADD] MJPEG fuzz ターゲットを追加する
  - @voluntas
- [UPDATE] CI / release ワークフローに NASM のインストールを追加する
  - @voluntas
- [FIX] libyuv の util ツール (cpuid / yuvconvert / yuvconstants) をビルド対象から外す
  - `util/cpuid.c` に Intel APX 命令 (`vdpphps`) が含まれ GitHub Actions の binutils ではアセンブルできないため、build.rs で libyuv の `CMakeLists.txt` をパッチして util ツールビルドを除外する
  - 静的ライブラリ (`libyuv.a`) の機能には影響しない
  - @voluntas
- [FIX] release ワークフローの prebuilt 生成失敗を修正する
  - `Find OUT_DIR` で BSD `wc -l` の先頭空白による文字列比較が常に失敗していたのを数値比較に修正する
  - `Verify archive contents` で `tar tzf | grep -q` の SIGPIPE が `pipefail` 下でジョブ失敗となるのを変数経由の検査に変更して回避する
  - @voluntas

## 2026.1.0

**リリース日**: 2026-03-31

- [ADD] シンボル書き換え機能を追加する
  - 静的ライブラリ内の全シンボルに `shiguredo_yuv_` プレフィックスを付与し、他ライブラリとの衝突を回避する
  - llvm-nm でシンボル収集、llvm-objcopy で書き換え、bindgen の `#[link_name]` で透過的にリンクする
  - source-build / prebuilt 両パスに対応する
  - @voluntas
- [ADD] compare モジュールを追加する
  - `i420_psnr()` / `i420_ssim()` で I420 画像間の品質比較を行う
  - `compute_sum_square_error()` / `compute_hamming_distance()` / `hash_djb2()` を提供する
  - @voluntas
- [ADD] rotate モジュールを追加する
  - I420, I010, I210, I410, I422, I444, ARGB の回転に対応する
  - プレーン単位の回転・転置、UV 分割回転を提供する
  - @voluntas
- [ADD] scale モジュールにフォーマットスケーリングを追加する
  - I422, I444, NV12, NV24, ARGB のスケーリングに対応する
  - 10bit/12bit/16bit プレーンのスケーリングに対応する
  - @voluntas
- [ADD] convert モジュールにフォーマット変換を追加する
  - I420, I422, I444, NV12, NV21, ARGB, ABGR, YUY2, UYVY, RGB565 等の相互変換に対応する
  - 10bit/12bit フォーマットの変換に対応する
  - @voluntas
- [ADD] planar モジュールに機能を追加する
  - `split_uv_plane()` / `merge_uv_plane()` / `swap_uv_plane()` で UV プレーン操作を行う
  - `split_rgb_plane()` / `merge_rgb_plane()` で RGB プレーン操作を行う
  - `i420_mirror()` / `nv12_mirror()` / `argb_mirror()` 等のミラー機能を提供する
  - @voluntas
- [ADD] 10bit/12bit/16bit 画像フォーマットを追加する
  - 3-プレーン: I010, I210, I410, I012, I212, I412, H010, H210, H410, U010, U210
  - 2-プレーン: P010, P210, P410, P012, P212
  - パック: Ar64, Ab64
  - @voluntas
- [ADD] 8bit 画像型を追加する
  - 3-プレーン: I422, I444, J420, J422, J444, H420, H422, H444, U420, U422, U444
  - 2-プレーン: NV21, NV16, NV24, MM21, MT2T
  - パック: ARGB, ABGR, RGBA, BGRA, RGB565, ARGB1555, ARGB4444, YUY2, UYVY, YUV24, AR30, AB30, AYUV
  - グレースケール: I400, J400
  - Android: Android420
  - @voluntas
- [ADD] `RotationMode` enum を追加する
  - @voluntas
- [CHANGE] prebuilt バイナリダウンロード機能を追加する
  - `source-build` feature でソースからのビルドに切り替え可能にする
  - デフォルトでは GitHub Releases から prebuilt バイナリをダウンロードする
  - SHA256 チェックサムで整合性を検証する
  - @voluntas
- [CHANGE] ビルド依存の `cmake` クレートを `shiguredo_cmake` に置き換える
  - @voluntas
- [CHANGE] ビルド依存の `toml` クレートを `shiguredo_toml` に置き換える
  - @voluntas
- [CHANGE] 画像型をマクロベースの統一的な型定義に再設計する
  - `I420Planes` / `Nv12Planes` / `Rgb24Image` 等の個別定義を廃止する
  - 3-プレーン / 2-プレーン / パック形式のマクロで全画像型を統一的に定義する
  - @voluntas

### misc

- [ADD] PBT テストを追加する (proptest)
  - @voluntas
- [ADD] Fuzzing ターゲットを追加する (cargo-fuzz)
  - @voluntas
- [ADD] Docs.rs ビルド対応を追加する
  - @voluntas

## 2025.2.0

**リリース日**: 2025-10-08

- [ADD] nv12 と i420 の相互変換関数を追加する
  - @voluntas

## 2025.1.0

**リリース日**: 2025-09-26
