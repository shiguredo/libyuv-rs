# libjpeg-turbo を組み込んで MJPEG 変換関数を有効化する

- Priority: High
- Created: 2026-06-12
- Completed:
- Model: Opus 4.7
- Branch: feature/add-libjpeg-turbo-mjpeg-support
- Polished: 2026-06-15

## 目的

libyuv の MJPEG デコード関数 (`MJPGToI420`, `MJPGToNV12`, `MJPGToNV21`, `MJPGToARGB`, `MJPGSize`) は libjpeg-turbo へのリンクが前提だが、現在のビルドシステムでは libjpeg-turbo を組み込んでいないため、これらの関数を Rust API として公開できない。libjpeg-turbo を組み込み、MJPEG 変換関数を Rust API として公開する。

## 範囲

本 issue で行うこと:

- libjpeg-turbo を `build.rs` から自動ビルドし、libyuv の `HAVE_JPEG` を有効化する
- MJPEG → I420 / NV12 / NV21 / ARGB の Rust API を追加する
- prebuilt バイナリにも libjpeg-turbo の静的ライブラリを同梱する
- **事前リファクタ** として `build.rs` の関連ヘルパー (定数群、`git_clone_external_lib` / `get_git_url_and_version` / `rewrite_symbols` / `find_static_library`) を複数ライブラリに対応できるよう汎用化する
- ライセンス対応 (libjpeg-turbo のライセンス文書をリポジトリと prebuilt アーカイブに同梱)
- CI / release ワークフローへの NASM セットアップ追加

本 issue で行わないこと:

- MJPEG エンコード (libyuv に対応関数が無いため対象外)
- libyuv の MJPG\* 関数が持つ **スケーリング機能** の Rust API 化 (本 issue では `src_size == dst_size` を強制する。将来別 issue で `_scaled` 系として追加する)
- macOS x86\_64 のサポート (現状の build.rs も非対応)
- PBT (`pbt/`) への追加 (有効な JPEG を proptest で生成するのは現実的でない)
- `issues/closed/0012-feat-android-mjpeg-hardware-formats.md` の記述訂正は本 issue とは別の作業で処理する

## 現状の問題

- `build.rs` に libjpeg-turbo 関連の設定が無く、libyuv の `find_package(JPEG)` が成功しないため `HAVE_JPEG` が定義されず、libyuv 上流の `source/convert_jpeg.cc` 等がコンパイルされない (libyuv の `include/libyuv/convert.h` / `convert_argb.h` 内の `MJPG*` 関数 **宣言** は `HAVE_JPEG` ガードの外にあり、bindgen は宣言を生成しているが、対応する **定義** が `libyuv.a` に含まれないためリンクできない)
- 結果として `src/convert.rs` には MJPEG 関連関数が存在しない

## 設計方針

### libjpeg-turbo のバージョンと取得元

libjpeg-turbo 3.1.1 を採用する。実装着手時に `git ls-remote --tags https://github.com/libjpeg-turbo/libjpeg-turbo` でタグ名と該当 commit hash を確認し、**commit hash で固定** する。

`Cargo.toml` への追加:

```toml
[package.metadata.external-dependencies.libjpeg-turbo]
git = "https://github.com/libjpeg-turbo/libjpeg-turbo"
version = "<確認した commit hash>"
```

### build.rs の汎用化リファクタ (事前リファクタ)

複数の外部ライブラリを扱えるよう、現状 libyuv 専用になっている定数群とヘルパーを汎用化する。

| 既存 (build.rs の行) | 変更内容 |
|---|---|
| `const LIB_NAME` / `LINK_NAME` / `SYMBOL_PREFIX` (L11-L25) | 削除。`LibraryConfig` 構造体に置き換える |
| `git_clone_external_lib(build_dir)` (L672) | `git_clone_external_lib(build_dir, lib: &LibraryConfig)` |
| `get_git_url_and_version()` (L700) | `get_git_url_and_version(lib_name: &str)` |
| `rewrite_symbols(lib_dir, out_dir)` (L423) | 定義シンボル書き換え用と未定義参照書き換え用を 2 つに分離 (下記 API 案) |
| `find_static_library(lib_dir)` (L457) | `find_static_library(lib_dir, unix_name, win_name)` |

`LibraryConfig` 例:

```rust
struct LibraryConfig {
    /// clone 先ディレクトリ名 / Cargo.toml メタデータキー
    name: &'static str,
    /// rustc-link-lib=static=<link_name> に使う名前
    link_name: &'static str,
    /// シンボル書き換え用プレフィックス
    symbol_prefix: &'static str,
    /// Unix での静的ライブラリファイル名 (cmake install 後)
    unix_lib_filename: &'static str,
    /// Windows での静的ライブラリファイル名 (cmake install 後)
    win_lib_filename: &'static str,
    /// CMake 変数 (キー, 値)
    cmake_defines: &'static [(&'static str, &'static str)],
    /// Windows MSVC 固有 CMake 変数 (キー, 値)
    cmake_defines_msvc: &'static [(&'static str, &'static str)],
    /// true なら x86_64 で NASM 必須
    requires_nasm_x86_64: bool,
    /// ライセンスファイルのソース内相対パス
    license_path: &'static str,
}
```

`LIBYUV` と `LIBJPEG_TURBO` の 2 つの `LibraryConfig` static を build.rs に置く。

`rewrite_symbols` は 2 関数に分割:

```rust
/// 定義済み外部シンボルにプレフィックスを付与し、bindgen 用 ParseCallbacks を返す
/// 内部で discover_llvm_tools / collect_defined_external_symbols / build_symbol_rename_maps /
/// write_objcopy_rename_map / rewrite_archive_symbols (既存) を呼ぶ
fn rename_defined_symbols(lib_path: &Path, out_dir: &Path,
                          prefix: &str, map_name: &str) -> SymbolLinkNameCallbacks;

/// 別ライブラリ向けに生成済みのリネームマップを適用して未定義参照シンボルを書き換える
/// 内部で discover_llvm_tools / rewrite_archive_symbols (既存) を呼ぶ
fn apply_external_rename_map(lib_path: &Path, map_path: &Path);
```

リネームマップファイル名は `OUT_DIR/symbol_rename_map_<link_name>.txt` のようにライブラリ名で分離する。

`cfg!(target_os = ...)` は使わず、ターゲット判定は `env::var("CARGO_CFG_TARGET_OS")` に統一する (クロスコンパイル時の取り違えを避ける)。

### libjpeg-turbo のビルド

`OUT_DIR/source/libjpeg-turbo/` に clone し、`shiguredo_cmake::Config` でビルド・install する (install prefix: `OUT_DIR/libjpeg-turbo-install/`)。

`cmake_defines` で渡す:

- `ENABLE_STATIC=ON`, `ENABLE_SHARED=OFF`
- `WITH_TURBOJPEG=OFF`, `WITH_TOOLS=OFF`, `WITH_TESTS=OFF`

`cmake_defines_msvc` で渡す:

- `CMAKE_MSVC_RUNTIME_LIBRARY=MultiThreadedDLL` (Rust の MSVC ターゲットの動的 CRT に揃える)

SIMD は **NASM 必須**:

- x86\_64: `build.rs` で `Command::new("nasm").arg("-v").status()` を確認し、失敗したら `panic!`
- aarch64: GAS の NEON SIMD のため NASM 不要 (NASM チェックを `requires_nasm_x86_64` フラグでスキップ)

### シンボル書き換え戦略

他クレートとのシンボル衝突を防ぐため、libyuv に `shiguredo_yuv_` プレフィックスを、libjpeg-turbo に `shiguredo_jpeg_` プレフィックスを付与する。リンク名も `shiguredo_yuv` / `shiguredo_jpeg` に統一する (汎用名 `jpeg` を rustc-link-lib に渡すと他クレートと衝突する可能性があるため)。

#### ビルドフロー

1. libjpeg-turbo を cmake install して `OUT_DIR/libjpeg-turbo-install/lib/libjpeg.a` (Windows: `lib/jpeg-static.lib`) を生成する
2. install 出力の `OUT_DIR/libjpeg-turbo-install/lib/libjpeg.a` (Windows: `jpeg-static.lib`) は **そのまま残す** (libyuv ビルドの `find_package(JPEG)` で参照させるため)。同時に `OUT_DIR/lib/libshiguredo_jpeg.a` (Windows: `OUT_DIR/lib/shiguredo_jpeg.lib`) にコピーする (こちらが最終的にリンクに使う実体、ステップ 4 でシンボル書き換える対象)
3. libyuv を cmake configure & build する。`shiguredo_cmake::Config` で `BUILD_SHARED_LIBS=OFF`、`CMAKE_PREFIX_PATH` に `OUT_DIR/libjpeg-turbo-install` を渡し、`find_package(JPEG)` を成功させて `add_definitions(-DHAVE_JPEG)` で MJPG\* 系のコンパイルを有効化する。libyuv の cmake はリネーム前の `OUT_DIR/libjpeg-turbo-install/lib/libjpeg.a` をリンク対象として認識するが、最終 Rust リンクではこれは使われない (libyuv.a 側の未定義参照はステップ 6 で `shiguredo_jpeg_*` に書き換えられる)
4. `OUT_DIR/lib/libshiguredo_jpeg.a` の定義シンボルを `llvm-nm --defined-only --extern-only` で収集し、`shiguredo_jpeg_` プレフィックス付きリネームマップ (`OUT_DIR/symbol_rename_map_jpeg.txt`) を生成。`llvm-objcopy --redefine-syms` で `libshiguredo_jpeg.a` を書き換える
5. libyuv の install 出力 `libyuv.a` (Windows: `yuv.lib`) を `OUT_DIR/lib/libshiguredo_yuv.a` (Windows: `OUT_DIR/lib/shiguredo_yuv.lib`) にコピーする
6. `libshiguredo_yuv.a` に **ステップ 4 のリネームマップを `--redefine-syms` で適用** する。`--redefine-syms` は未定義参照シンボルも書き換えるため、`libyuv.a` 内の `jpeg_*` への未定義参照 / グローバル関数ポインタ参照 (`jpeg_resync_to_restart` 等) が `shiguredo_jpeg_*` に書き換わる
7. `libshiguredo_yuv.a` の定義シンボルを収集し、`shiguredo_yuv_` プレフィックス付きリネームマップ (`OUT_DIR/symbol_rename_map_yuv.txt`) で書き換える。MJPG\* シンボルもステップ 3 で定義済みのため、ここで `shiguredo_yuv_MJPGToI420` 等にリネームされ、対応する `SymbolLinkNameCallbacks` の `rename_map` (= bindgen 用) に登録される

ステップ 4 と 6 は既存 `rewrite_archive_symbols` (build.rs L657) を内部実装としてそのまま再利用し、`apply_external_rename_map(lib_path, map_path)` (リネームマップ既存の場合に呼ぶ) と `rename_defined_symbols(lib_path, out_dir, prefix, map_name)` (定義シンボル収集 + マップ生成 + 適用) の 2 つを上位 API として用意する。

#### build.rs 各関数への展開

- `main()`: 既存 `should_use_prebuilt()` 分岐の後、新規ヘルパー `build_libjpeg_turbo(&out_dir) -> LibjpegTurboBuildResult` と既存 `build_from_source(&out_dir, &output_bindings_path, &libjpeg_turbo_result)` を順に呼ぶ。`LibjpegTurboBuildResult` には `install_prefix: PathBuf` (libyuv ビルドで `CMAKE_PREFIX_PATH` に渡す) と `rename_map_path: PathBuf` (libyuv の未定義参照書き換えに使う `symbol_rename_map_jpeg.txt`) の 2 つを格納する。`build_libjpeg_turbo` がステップ 1〜2、4 を担当し、`build_from_source` がステップ 3, 5〜7 を担当する
- `main()` のリンク出力 (現状 L86 の 1 行) を以下の 2 行に差し替える。`download_prebuilt()` 経路でも同じ 2 行を出す (どちらの経路でも `OUT_DIR/lib/` に `libshiguredo_yuv.a` と `libshiguredo_jpeg.a` が揃う前提):
  ```
  cargo::rustc-link-lib=static=shiguredo_yuv
  cargo::rustc-link-lib=static=shiguredo_jpeg
  ```
- bindgen 呼び出し (現状 `build_from_source()` L294) はステップ 7 の後に置く。`rename_defined_symbols` の戻り値 (`SymbolLinkNameCallbacks`) を `parse_callbacks` に渡す

#### マップファイルの相互汚染防止

- `symbol_rename_map_jpeg.txt` と `symbol_rename_map_yuv.txt` でファイル名を分離する (既存実装の単一 `symbol_rename_map.txt` ではステップ 4 と 7 で衝突する)
- ステップ 7 の `llvm-nm --defined-only --extern-only` フィルタにより、ステップ 6 で書き換わった未定義参照シンボル (`shiguredo_jpeg_*`) は採取対象外となる
- `bindgen` 用 `rename_map` には libyuv 側 (ステップ 7 で生成) のみを渡す。libjpeg-turbo の `rename_defined_symbols` 戻り値の `bindgen_map` は破棄する (libjpeg-turbo API は bindgen 生成対象に含まれないため)

#### 書き換え結果の検証 (CI で自動実施)

`scripts/verify_symbol_rewrite.sh` を本 issue で新設し、CI のビルド後に実行する。仕様:

- **呼び出し**: `bash scripts/verify_symbol_rewrite.sh <OUT_DIR>`
- **環境**: `llvm-nm` / `llvm-objdump` のパスは `rustc --print sysroot` から `lib/rustlib/$(rustc -vV | sed -n 's|host: ||p')/bin/` 配下を解決する
- **shebang / set**: `#!/usr/bin/env bash` + `set -euo pipefail`
- **assert 内容** (1 つでも失敗したら exit 1。`grep` の単語境界マッチでは `shiguredo_jpeg_jpeg_*` のような二重プレフィックスを誤検出しないよう `grep -v shiguredo_` で除外する):
  1. `llvm-nm -u "$OUT_DIR/lib/libshiguredo_yuv.a"` (Windows: `shiguredo_yuv.lib`) で未定義参照に `jpeg_` / `jsimd_` で始まるシンボルが残らない (`grep -E '^_?(jpeg_|jsimd_)' | grep -v shiguredo_` が空)
  2. `llvm-nm -u "$OUT_DIR/lib/libshiguredo_jpeg.a"` (Windows: `shiguredo_jpeg.lib`) でも同様に未定義参照が残らない (リネーム後、libjpeg-turbo 内部の `jsimd_*` 等のクロス参照もすべて `shiguredo_jpeg_*` に統一されている)
  3. `llvm-nm --defined-only --extern-only "$OUT_DIR/lib/libshiguredo_yuv.a"` で `shiguredo_yuv_MJPGSize`, `shiguredo_yuv_MJPGToI420`, `shiguredo_yuv_MJPGToNV12`, `shiguredo_yuv_MJPGToNV21`, `shiguredo_yuv_MJPGToARGB` の 5 つが定義されている (Mach-O は `_shiguredo_yuv_MJPG*`)

別途 `scripts/verify_libyuv_source.sh` を `ci.yml` の `test` ジョブで実行し、libyuv ソース (`target/debug/build/shiguredo_libyuv-*/out/source/libyuv/source/` 配下) に対して `grep -rE '\bjpeg_[a-z_]+\b'` を走らせ、libjpeg API 呼び出し以外の同名シンボルが無いことを確認する (libyuv の commit を bump するたびの安全策)。

**CI への組み込み位置** (Windows runner でも動かすため `shell: bash` を明示し、`rustc -vV` の出力で `\r` が混入する点を `tr -d '\r'` で除去する):

- `ci.yml` の `test` ジョブ: `cargo test --workspace --features source-build` の直後に下記ステップを追加する。`find -maxdepth 0` が複数候補をヒットしたら fail させる (`set -euo pipefail` でガード):
  ```yaml
  - name: Verify symbol rewrite
    shell: bash
    run: |
      candidates=$(find target/debug/build/shiguredo_libyuv-*/out -maxdepth 0 -type d)
      count=$(echo "$candidates" | wc -l)
      [ "$count" = "1" ] || { echo "expected 1 OUT_DIR but got $count"; exit 1; }
      OUT_DIR=$(echo "$candidates" | head -1)
      bash scripts/verify_symbol_rewrite.sh "$OUT_DIR"
      bash scripts/verify_libyuv_source.sh "$OUT_DIR"
  ```
- `release.yml` の `build-prebuilt` ジョブ: `Find OUT_DIR` ステップ (既存) の直後に「`bash scripts/verify_symbol_rewrite.sh "${{ steps.find_out_dir.outputs.OUT_DIR }}"`」を `shell: bash` 明示で追加する。同時に既存 `Find OUT_DIR` ステップにも `wc -l` ガードと `set -euo pipefail` を追加する

**検証失敗時の対応**: assert のいずれかが失敗したら実装作業を停止し、ユーザーに「どの assert がどのプラットフォームで失敗したか」を報告する。本 issue 内ではフォールバック実装 (libjpeg-turbo ビルド時のマクロ注入による自己整合書き換えなど) を行わず、別 issue (`feature/fix-symbol-rewrite-<platform>`) として切り出して対処する。

### bindgen の設定とリンク

bindgen は引き続き `libyuv.h` のみを入力する。MJPG\* 関数の宣言は libyuv ヘッダーで `HAVE_JPEG` ガードの外で行われているため、bindgen への `-DHAVE_JPEG=1` 注入は **不要**。libyuv ヘッダーから推移的に libjpeg-turbo の型 (`jpeg_decompress_struct` 等) が混入することを防ぐ保険として、bindgen に `.blocklist_type("jpeg_.*")` を追加する。

リンク設定 (build.rs メイン関数で出力):

```
cargo::rustc-link-search=native=<OUT_DIR>/lib/
cargo::rustc-link-lib=static=shiguredo_yuv
cargo::rustc-link-lib=static=shiguredo_jpeg
```

libyuv (依存する側) を先、libjpeg (依存される側) を後に書く。C++ 標準ライブラリのリンクは既存ロジック (`stdc++` / `c++`) を維持する。

### prebuilt アーカイブの構造

```
lib/
  libshiguredo_yuv.a      (Windows: shiguredo_yuv.lib)
  libshiguredo_jpeg.a     (Windows: shiguredo_jpeg.lib)
LICENSE                   (libyuv-rs の Apache-2.0)
THIRD_PARTY_LICENSES      (libjpeg-turbo のライセンス文書)
bindings.rs
```

`download_prebuilt()` の改修 (現状 build.rs L106-L189 の制御フロー内に挿入):

1. 既存の curl ダウンロード・SHA256 検証・tar 展開はそのまま
2. tar 展開後 (コピー前) に `prebuilt_dir/lib/libshiguredo_jpeg.a` (Windows: `shiguredo_jpeg.lib`) と `prebuilt_dir/THIRD_PARTY_LICENSES` の存在を確認し、いずれかが無ければ `panic!` する
3. `prebuilt_dir/lib/libshiguredo_yuv.a` / `prebuilt_dir/lib/libshiguredo_jpeg.a` を `OUT_DIR/lib/` にコピー
4. `prebuilt_dir/bindings.rs` を `OUT_DIR/bindings.rs` にコピー (既存どおり)

prebuilt URL は `CARGO_PKG_VERSION` を含むため、本 issue リリース時に `Cargo.toml` の `version` を bump する。

### Rust API

`src/convert.rs` に追加する。MJPEG は可変長の圧縮データなので入力は `&[u8]`。

```rust
/// MJPEG 圧縮データから画像サイズを取得する
pub fn mjpeg_size(src: &[u8]) -> Result<ImageSize, Error>

/// MJPEG 圧縮データから I420 にデコードする (スケーリング非対応)
pub fn mjpeg_to_i420(src: &[u8], dst: &mut I420ImageMut<'_>, size: ImageSize) -> Result<(), Error>
pub fn mjpeg_to_nv12(src: &[u8], dst: &mut Nv12ImageMut<'_>, size: ImageSize) -> Result<(), Error>
pub fn mjpeg_to_nv21(src: &[u8], dst: &mut Nv21ImageMut<'_>, size: ImageSize) -> Result<(), Error>
pub fn mjpeg_to_argb(src: &[u8], dst: &mut ArgbImageMut<'_>, size: ImageSize) -> Result<(), Error>
```

`mjpeg_validate` 系の専用 API は提供しない (用途があれば `mjpeg_size(src).is_ok()` で代用可能。専用 API を増やす積極的理由が無い)。

`size` 引数の役割: `dst` バッファサイズ検証のために呼び出し側が指定する。**内部で `mjpeg_size(src)` を呼んで二重に検査することはしない** (1 フレームあたり JPEG デコンプレッサ生成・破棄を 2 回行うことになり性能影響が無視できないため)。JPEG ヘッダの実サイズと `size` が不一致な場合は libyuv の C 関数が戻り値 `< 0` を返すので、`Error::check` でエラー化する。`size = ImageSize::new(0, 0)` のような空サイズは `dst.validate` 段階で弾く。

各関数で実施する入力検証 (関数頭部、libyuv 呼び出し前):

- `src.is_empty()` → `Err`
- `src.len()` の `c_int` 範囲チェック
- `size.width` / `size.height` の `c_int` 範囲チェック (`require_c_int` 既存ヘルパー)
- `size.width == 0 || size.height == 0` → `Err`
- `dst.validate(size, "<関数名>")?` で dst バッファサイズ検証

libyuv の C 関数 (例: `MJPGToI420`) は `src_width / src_height / dst_width / dst_height` を取るが、本 API ではすべてに `size.width` / `size.height` を 4 つとも同値で渡してスケーリングを無効化する。dst プレーンのストライドは既存 API と同じく `dst.y_stride` / `dst.u_stride` / `dst.v_stride` (NV12 / NV21 は `dst.y_stride` / `dst.uv_stride`、ARGB は `dst.stride`) を `c_int` キャストして渡す。

### Docs.rs ダミー bindings

`build.rs` の `DOCS_RS` 分岐で出力するダミー bindings に MJPG\* 関数の FFI シグネチャを追加する。libyuv の commit (`Cargo.toml` で固定したもの) の `include/libyuv/convert.h` (`MJPGSize`, `MJPGToI420`, `MJPGToNV12`, `MJPGToNV21`) と `include/libyuv/convert_argb.h` (`MJPGToARGB`) を直接確認して厳密に一致させる。

libjpeg-turbo は DOCS_RS=1 時に git clone をスキップする。

### CI / Release ワークフローの修正

`.github/workflows/ci.yml`:

- `test` ジョブの `steps:` で `rustup update stable` の直後、`bash scripts/verify_license_hash.sh` → `cargo fmt --all --check` の順に追加する。NASM インストールも `rustup update stable` の直後・`verify_license_hash.sh` の前に挟む (下記 yaml)
- `cargo test --workspace --features source-build` の直後に「OUT_DIR を探して `verify_symbol_rewrite.sh` と `verify_libyuv_source.sh` を実行する」ステップを追加する
- `docs-rs` ジョブは git clone をスキップするため NASM / 検証スクリプト追加不要
- `slack_notify` ジョブは触らない (依存ジョブの成否のみ判定するため)

`.github/workflows/release.yml`:

- `build-prebuilt` ジョブの `Checkout sources` 直後、`Build with source-build` 直前に NASM インストールを追加する
- `matrix` 既存の `lib_name` を `libyuv.a` → `libshiguredo_yuv.a` (Windows: `yuv.lib` → `shiguredo_yuv.lib`) に変更し、新たに `jpeg_lib_name` (`libshiguredo_jpeg.a` / `shiguredo_jpeg.lib`) を追加する (build.rs 側で既にこのファイル名で `OUT_DIR/lib/` に配置されているため、コピーで使用する)
- `Find OUT_DIR` ステップ直後に `bash scripts/verify_symbol_rewrite.sh "${{ steps.find_out_dir.outputs.OUT_DIR }}"` ステップを追加する
- `Create prebuilt archive` ステップで `${OUT_DIR}/lib/${{ matrix.lib_name }}` と `${OUT_DIR}/lib/${{ matrix.jpeg_lib_name }}` を `staging/lib/` に、`LICENSE` と `THIRD_PARTY_LICENSES` を `staging/` にコピーする
- アーカイブ作成後に `tar tzf libyuv-${{ matrix.target }}.tar.gz | grep -q "^./THIRD_PARTY_LICENSES$"` で同梱確認するステップを追加する (混入漏れの早期検出)
- `slack_notify` / `publish` ジョブは触らない

NASM インストールステップ (共通):

```yaml
- name: Install NASM
  shell: bash
  run: |
    case "${{ runner.os }}" in
      Linux)
        if [ "${{ runner.arch }}" = "X64" ]; then
          sudo apt-get update && sudo apt-get install -y nasm
        fi
        ;;
      macOS)   brew install nasm ;;
      Windows) choco install nasm -y && echo "C:\\Program Files\\NASM" >> $GITHUB_PATH ;;
    esac
```

Linux arm64 は GAS の NEON SIMD のため NASM 不要 (上記スクリプトで X64 のみインストール)。

### ライセンス対応

リポジトリルートに `THIRD_PARTY_LICENSES` を新設する。生成手順:

1. 本 issue 実装着手時に `Cargo.toml` の libjpeg-turbo commit hash を確定する
2. その commit の `LICENSE.md` 全文を取得する (`curl -fsSL https://raw.githubusercontent.com/libjpeg-turbo/libjpeg-turbo/<hash>/LICENSE.md`)
3. `THIRD_PARTY_LICENSES` の先頭に `# libjpeg-turbo (commit <hash>)` のセクションヘッダを書き、続けて取得した `LICENSE.md` 全文を貼り付ける
4. 将来 libjpeg-turbo の commit を bump する際は `THIRD_PARTY_LICENSES` も同時に更新する。これを CI で強制するため `scripts/verify_license_hash.sh` を本 issue で新設する。仕様:
   - shebang / set: `#!/usr/bin/env bash` + `set -euo pipefail`
   - 呼び出し: `bash scripts/verify_license_hash.sh`
   - 動作: `Cargo.toml` から libjpeg-turbo セクションの `version` を抽出する。`Cargo.toml` 内には複数の `version` キーがあるため `awk` でセクション境界を判定する (例: `awk '/^\[package\.metadata\.external-dependencies\.libjpeg-turbo\]/,/^\[/ {if ($1 == "version") print $3}' Cargo.toml | tr -d '"'`)。`THIRD_PARTY_LICENSES` 冒頭の `# libjpeg-turbo (commit <hash>)` から `sed -n '1s/.*(commit \([0-9a-f]*\)).*/\1/p' THIRD_PARTY_LICENSES` で hash を取り、両者を比較する。不一致なら exit 1
   - CI 組み込み: `ci.yml` の `test` ジョブで `cargo fmt --all --check` の直前に `shell: bash` 明示で実行ステップを追加する
5. `Cargo.toml` の `include` に `/THIRD_PARTY_LICENSES` を追加する (`/src/test_data/` は `/src/**` でカバーされるため別途追加不要)

prebuilt アーカイブには `release.yml` の `Create prebuilt archive` ステップでリポジトリの `THIRD_PARTY_LICENSES` をコピーする。

## テスト

### テストデータ

固定 JPEG バイト列を `include_bytes!` で参照する (libjpeg-turbo の `testimages/` 配下はライセンス混在のため不使用)。

`src/test_data/` に以下をコミットする:

- `mjpeg_8x8_yuv420.jpg`
- `mjpeg_16x16_yuv422.jpg`
- `mjpeg_64x48_yuv444.jpg`

生成は ImageMagick を使う (1 度生成してコミットし、CI からは再生成しない)。手順を `src/test_data/README.md` に書き留める:

```bash
magick -size 8x8   gradient:red-blue -sampling-factor 2x2 -quality 80 mjpeg_8x8_yuv420.jpg
magick -size 16x16 gradient:red-blue -sampling-factor 2x1 -quality 80 mjpeg_16x16_yuv422.jpg
magick -size 64x48 gradient:red-blue -sampling-factor 1x1 -quality 80 mjpeg_64x48_yuv444.jpg
```

サイズ選定理由は MCU 境界 (8x8 / 8x16 / 16x16 の境界) を網羅するため。生成後に `jpegtran -copy none` でメタデータを削除する。

### 単体テスト (`tests/mjpeg.rs`)

新規に `tests/mjpeg.rs` を追加 (`tests/` ディレクトリ自体も新設)。`include` 配列は `tests/` を含まない方針を維持する (integration テストはリポジトリ内のみで使用、crates.io 配布物には不要)。

- 正常系
  - 各サブサンプリングで `mjpeg_size()` が正しい幅・高さを返す
  - 各サブサンプリングで `mjpeg_to_i420` / `mjpeg_to_nv12` / `mjpeg_to_nv21` / `mjpeg_to_argb` が `Ok` を返す
- 異常系
  - 空バッファで全関数が `Err`
  - 先頭 2 バイト (`0xFF 0xD8`) を破壊した JPEG で全関数が `Err`
  - `size` が JPEG ヘッダのサイズと不一致なときに `mjpeg_to_*` が `Err`
  - `dst` バッファサイズが不足するとき `Err`
  - `size = ImageSize::new(0, 0)` で `Err`
  - `size.width = c_int::MAX as usize + 1` で `Err`

### fuzz ターゲット

`fuzz/fuzz_targets/fuzz_mjpeg.rs` を新設し (既存 `fuzz_convert.rs` 等の命名に揃える)、`fuzz/Cargo.toml` に対応する `[[bin]]` セクションを追加する:

```toml
[[bin]]
name = "fuzz_mjpeg"
path = "fuzz_targets/fuzz_mjpeg.rs"
test = false
doc = false
bench = false
```

fuzz ターゲット戦略:

- `mjpeg_size(src)` を最初に呼ぶ
  - `Ok(size)` で `size.width * size.height` が 4 MiB (RGBA で 4 bytes/px なので最大 16 MiB バッファ) を超える場合はスキップする (fuzz harness OOM 対策)
  - `Ok(size)` で上記制限内なら `mjpeg_to_i420` / `mjpeg_to_nv12` / `mjpeg_to_nv21` / `mjpeg_to_argb` の **すべて** に対し動的算出した dst バッファで呼び出す
  - `Err` の場合は固定の最小サイズ (`ImageSize::new(8, 8)`) と固定 dst バッファで `mjpeg_to_*` を呼び、入力検証が空 / 不正 src で適切にエラー化することを検査する

クラッシュしないこと、メモリ不正アクセスが起きないことを検査する。

## CHANGES.md 更新

`shiguredo-changelog` 規約 (種別並び CHANGE → ADD → UPDATE → FIX) に従い、既存 `[UPDATE] libyuv のハッシュを ... に更新する` の **上** に挿入する (既存エントリは触らない):

```
- [CHANGE] build.rs を複数の外部ライブラリに対応できるよう汎用化する
  - LIB_NAME / LINK_NAME / SYMBOL_PREFIX 定数を LibraryConfig 構造体に置き換える
  - git_clone_external_lib / get_git_url_and_version / rewrite_symbols / find_static_library を汎用化する
  - @voluntas
- [CHANGE] prebuilt アーカイブの構造を変更する
  - lib/ 配下のファイル名を libshiguredo_yuv.a (Windows: shiguredo_yuv.lib) / libshiguredo_jpeg.a (Windows: shiguredo_jpeg.lib) に変更する
  - THIRD_PARTY_LICENSES をアーカイブに同梱する
  - 旧バージョン prebuilt との互換性は無い (Cargo.toml の version を bump)
  - @voluntas
- [ADD] libjpeg-turbo をビルド依存として組み込む
  - libjpeg-turbo 3.1.1 (commit <hash>) を build.rs から自動ビルドする
  - 静的ライブラリのシンボルに shiguredo_jpeg_ プレフィックスを付与する
  - @voluntas
- [ADD] MJPEG 変換関数を追加する
  - mjpeg_size / mjpeg_to_i420 / mjpeg_to_nv12 / mjpeg_to_nv21 / mjpeg_to_argb
  - スケーリング非対応 (src_size == dst_size を強制)
  - @voluntas
```

`### misc` セクション:

```
- [ADD] MJPEG fuzz ターゲットを追加する
  - @voluntas
- [UPDATE] CI / release ワークフローに NASM のインストールを追加する
  - @voluntas
```

担当者表記は既存エントリの `@voluntas` 形式を踏襲する。

## 完了条件

- `source-build` feature の有無にかかわらず MJPEG 関数が利用可能 (source-build は自動ビルド、prebuilt はダウンロード)
- `mjpeg_size`, `mjpeg_to_i420`, `mjpeg_to_nv12`, `mjpeg_to_nv21`, `mjpeg_to_argb` が Rust API として公開される
- `tests/mjpeg.rs` の正常系・異常系テストがすべて通る
- `fuzz/fuzz_targets/fuzz_mjpeg.rs` が `cargo fuzz run fuzz_mjpeg` で起動できる (実行時間は問わない)
- CI の `test` ジョブで `cargo fmt --all --check` / `cargo clippy --workspace --features source-build -- -D warnings` / `cargo test --workspace --features source-build` がパスする
- CI の `docs-rs` ジョブで `cargo doc --no-deps` (`DOCS_RS=1`) がパスする
- `scripts/verify_symbol_rewrite.sh` が新設され、`ci.yml` の `test` ジョブで `cargo test` の直後、`release.yml` の `build-prebuilt` ジョブで `Find OUT_DIR` の直後に実行されている
- `scripts/verify_libyuv_source.sh` が新設され、`ci.yml` の `test` ジョブで実行されている
- `scripts/verify_license_hash.sh` が新設され、`ci.yml` の `test` ジョブで `cargo fmt` の直前に実行されている
- `THIRD_PARTY_LICENSES` がリポジトリに存在し、crates.io 公開物と prebuilt アーカイブに同梱される
- (リリース後手順、PR レビュー範囲外): 本 issue マージ → リリースタグ作成 → `release.yml` の prebuilt 生成完了後、ローカルで `cargo test --workspace` (source-build 無し) を実行し prebuilt 経路の MJPEG 関数動作を手動確認する
- `CHANGES.md` の `## develop` セクションに上記エントリが追記されている (種別並び厳守)
