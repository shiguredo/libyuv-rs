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

- [UPDATE] `libyuv` のハッシュを `d23308a2a7442be8e559b1b471862fd7588d6a57` に更新する
  - <https://chromium.googlesource.com/libyuv/libyuv/+/d23308a2a7442be8e559b1b471862fd7588d6a57>
  - @voluntas

### misc

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
