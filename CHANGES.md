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

- [CHANGE] MSRV (rust-version) を 1.93 に上げる
  - @voluntas
- [CHANGE] build.rs を複数の外部ライブラリに対応できるよう汎用化する
  - `LIB_NAME` / `LINK_NAME` / `SYMBOL_PREFIX` 定数を `LibraryConfig` 構造体に置き換える
  - `git_clone_external_lib` / `get_git_url_and_version` / `rewrite_symbols` / `find_static_library` を汎用化する
  - @voluntas
- [CHANGE] prebuilt アーカイブの構造を変更する
  - `lib/` 配下のファイル名を `libshiguredo_yuv.a` (Windows: `shiguredo_yuv.lib`) / `libshiguredo_jpeg.a` (Windows: `shiguredo_jpeg.lib`) に変更する
  - `THIRD_PARTY_LICENSES` をアーカイブに同梱する
  - 旧バージョン prebuilt との互換性は無いため、リリース時に `Cargo.toml` の version を bump する
  - @voluntas
- [CHANGE] detile_split_uv_plane の戻り値型を Result<(), Error> に変更し入力検証を追加する
  - @voluntas
- [CHANGE] sum_square_error_to_psnr の戻り値型を Result<f64, Error> に変更し count == 0 の検証を追加する
  - @voluntas
- [CHANGE] hash_djb2 の戻り値型を Result<u32, Error> に変更する
  - @voluntas
- [CHANGE] Mm21ImageMut / Mt2tImageMut を削除する
  - libyuv に MM21 / MT2T を出力する変換が存在しないため、使う手段のない公開型を削除する
  - @voluntas
- [ADD] libjpeg-turbo をビルド依存として組み込む
  - libjpeg-turbo 3.1.90 (tag `3.1.90`, commit `e1dbfa7be7b7e54922020051dc77781e92739700`) を build.rs から自動ビルドする
  - 静的ライブラリのシンボルに `shiguredo_jpeg_` プレフィックスを付与する
  - @voluntas
- [ADD] MJPEG 変換関数を追加する
  - `mjpeg_size` / `mjpeg_to_i420` / `mjpeg_to_nv12` / `mjpeg_to_nv21` / `mjpeg_to_argb`
  - スケーリング非対応 (`src_size == dst_size` を強制)
  - @voluntas
- [ADD] Ubuntu 26.04 (x86_64 / arm64) をサポート対象に追加する
  - @voluntas
- [UPDATE] `libyuv` のハッシュを `d23308a2a7442be8e559b1b471862fd7588d6a57` に更新する
  - <https://chromium.googlesource.com/libyuv/libyuv/+/d23308a2a7442be8e559b1b471862fd7588d6a57>
  - @voluntas
- [UPDATE] scale.rs の _12/_16 関数のドキュメントを改善し 12bit/16bit データの意味的差異を明確化する
  - @voluntas
- [FIX] calc_frame_psnr と i420_psnr がゼロサイズ入力で意味のない値を返す問題を修正する
  - @voluntas
- [FIX] calc_frame_ssim と i420_ssim が 9x9 未満の小さな画像で NaN を返す問題を修正する
  - @voluntas
- [FIX] i420_ssim が 17x17 未満（width または height が 16 以下）の画像で NaN を返す問題を修正する
  - U/V プレーンの縮小（(width + 1) / 2 × (height + 1) / 2）を考慮し、Y プレーンが 9x9 以上でも U/V が 8 以下になるサイズは Err を返す
  - @voluntas
- [FIX] 奇数幅の変換で libyuv の SIMD 余り処理がバッファ行終端を読み書き越す問題を修正する
  - YUY2 / UYVY 系の変換 16 関数に、width 奇数時の最終行の読み書き量（width * 2 + 2 バイト）を考慮したバッファ検証を追加する
  - ソース読み越し: yuy2_to_y / uyvy_to_y / yuy2_to_argb / uyvy_to_argb / yuy2_to_i420 / uyvy_to_i420 / yuy2_to_i422 / uyvy_to_i422 / yuy2_to_nv12 / uyvy_to_nv12
  - デスティネーション書き込み越え: i420_to_yuy2 / i420_to_uyvy / i422_to_yuy2 / i422_to_uyvy / argb_to_yuy2 / argb_to_uyvy
  - 行合体 (Coalesce) が発動する関数は、合体後の幅 × 高さが奇数のときのみ +2 バイトを要求する
  - width 奇数で従来通過していた stride * height ちょうどのバッファは Err を返すようになる（2 バイト不足）
  - @voluntas
- [FIX] detile_plane と detile_plane_16 の入力検証をプロジェクト標準パターンに統一する
  - @voluntas
- [FIX] half_float_plane の stride 単位が libyuv 仕様（バイト）と不一致で出力が壊れる問題を修正する
  - 変換後の stride が c_int の範囲を超える場合は Err を返す
  - @voluntas
- [FIX] yuy2_to_y と uyvy_to_y のデスティネーション検証を標準パターンに統一する
  - dst_stride_y の c_int 範囲チェックと stride >= width チェックを追加し、非チェック乗算を checked_buf_size に置き換える
  - @voluntas
- [FIX] android420_to_* の pixel_stride_uv == 2 で U/V プレーンの検証が不足し領域外読み出しが発生する問題を修正する
  - pixel_stride_uv の値検証（1 / 2 以外は Err）と、インターリーブ前提の U/V ストライド・バッファ検証を追加する
  - @voluntas
- [FIX] Mm21Image / Mt2tImage のバッファ検証がタイル配置・10bit パックを反映しておらず領域外読み出しが発生する問題を修正する
  - タイル配置の必要サイズ（幅を 16 の倍数に切り上げた値 × タイル高）で検証する
  - MT2T は 10bit パック（10/8 倍）を必要サイズに反映する
  - ゼロサイズ入力（width / height == 0）は Err を返す（旧実装は height == 0 で成功していた）
  - @voluntas
- [FIX] detile_plane / detile_plane_16 / detile_to_yuy2 のソースバッファ検証が線形サイズのままで領域外読み出しが発生する問題を修正する
  - ソースの必要サイズをタイル配置（src_stride * ceil(height / tile_height) * tile_height）で検証し、ストライドに round_up(width, 16) の下限を要求する
  - detile_to_yuy2 は Y / UV それぞれのタイル配置サイズ（UV はタイル高 tile_height / 2）で検証し、tile_height の検証（2 以上かつ 2 累乗）と c_int 範囲チェックを追加する
  - ゼロサイズ入力（width / height == 0）は no-op で Ok を返す（detile 系の既存セマンティクスを維持する）
  - @voluntas
- [FIX] p210_to_p410 / nv12_to_nv24 / nv16_to_nv24 の手書き検証が不十分で領域外アクセスのリスクがある問題を修正する
  - 手書き検証を生成済みの validate に置き換え、require_c_int チェック・最小ストライドチェック・オーバーフロー安全なサイズ計算を追加する
  - stride 不足（Y は width 未満、UV は chroma 幅未満）と stride の c_int 範囲超過は従来通過していたが Err を返すようになる（p210_to_p410 は src 側、nv12_to_nv24 は dst 側、nv16_to_nv24 は src / dst 側）
  - @voluntas
- [FIX] argb_blur の cumsum 検証が C の循環バッファ契約と不一致で省メモリ利用を拒否する問題を修正する
  - cumsum の必要行数を min(height, 有効 radius * 2 + 2) に緩和し、radius の clamp 規則（min(radius, height)、min(radius, width / 2 - 1)）を C と一致させる
  - radius <= 0 / height <= 1 / width <= 3 は従来 C 経由で Err になっていたが、Rust 側の検証として明示的に Err を返すようになる
  - @voluntas
- [FIX] planar の 16bit 変換関数群で depth パラメータが未検証のまま libyuv に渡される問題を修正する
  - 関数ごとの有効範囲（merge_uv_plane_16 / split_uv_plane_16 / merge_argb16_to_8_plane / convert_to_lsb_plane_16 / convert_to_msb_plane_16 は 8..=16、merge_ar64_plane は 1..=16、merge_xr30_plane は 10..=16）を検証し、範囲外は Err を返す
  - 範囲外の depth は従来 C 側のシフト演算で未定義動作になる可能性があった
  - @voluntas
- [FIX] アルファプレーンの stride 検証（c_int 範囲・stride >= width）が欠落している問題を修正する
  - i420_alpha_to_argb / i420_alpha_to_abgr / i422_alpha_to_argb / i422_alpha_to_abgr / i444_alpha_to_argb / i444_alpha_to_abgr / argb_to_i420_alpha のアルファプレーンに require_c_int チェックと stride >= width チェックを追加する
  - stride の c_int 範囲超過と stride < width は従来通過していたが Err を返すようになる
  - @voluntas

### misc

- [CHANGE] convert.rs をサブモジュールに分割する
  - @voluntas
- [ADD] MJPEG fuzz ターゲットを追加する
  - @voluntas
- [ADD] 欠落テストファイルを作成しバリデーション内部関数の単体テストを追加する
  - @voluntas
- [UPDATE] lint 抑制を #[allow(...)] から #[expect(...)] に置換する
  - @voluntas
- [UPDATE] CI / release ワークフローに NASM のインストールを追加する
  - @voluntas
- [UPDATE] 全 unsafe ブロック (357 箇所) に SAFETY コメントを追加する
  - @voluntas
- [FIX] libyuv の util ツール (cpuid / yuvconvert / yuvconstants) をビルド対象から外す
  - `util/cpuid.c` に Intel APX 命令 (`vdpphps`) が含まれ GitHub Actions の binutils ではアセンブルできないため、build.rs で libyuv の `CMakeLists.txt` をパッチして util ツールビルドを除外する
  - 静的ライブラリ (`libyuv.a`) の機能には影響しない
  - @voluntas
- [FIX] fuzz ターゲットが旧汎用画像型を参照してコンパイル不能になっていたのを修正する
  - @voluntas
- [FIX] release ワークフローの prebuilt 生成失敗を修正する
  - `Find OUT_DIR` で BSD `wc -l` の先頭空白による文字列比較が常に失敗していたのを数値比較に修正する
  - `Verify archive contents` で `tar tzf | grep -q` の SIGPIPE が `pipefail` 下でジョブ失敗となるのを変数経由の検査に変更して回避する
  - @voluntas
- [FIX] verify_symbol_rewrite.sh の未解決シンボル検査が Linux / Windows で常に成功してしまう問題を修正する
  - `llvm-nm -u` に `--format=just-symbols` を追加し、プラットフォーム間の出力形式を統一する（macOS と Linux ELF / Windows COFF の bsd 形式の差で行頭アンカーがマッチせず検査が不発になっていた）
  - Windows の \r 混入対策として `tr -d '\r'` を追加する
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
