use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use shiguredo_cmake::Config;

// ============================================================
// 外部ライブラリ設定
// ============================================================

/// 外部ライブラリのビルド・リンク・シンボル書き換えに必要な設定
///
/// 複数の外部ライブラリ (libyuv, libjpeg-turbo) を同じビルドフローで扱えるように、
/// ライブラリごとに差分となる名前 / 拡張子 / CMake 変数を構造体に集約する。
struct LibraryConfig {
    /// clone 先ディレクトリ名 兼 Cargo.toml の `[package.metadata.external-dependencies.<name>]` キー
    name: &'static str,
    /// `cargo::rustc-link-lib=static=<link_name>` に使う名前
    ///
    /// 汎用名 (例: `jpeg`) を直接使うと他クレートと衝突するため、必ず時雨堂専用名にする。
    link_name: &'static str,
    /// シンボル書き換え用プレフィックス (例: `shiguredo_yuv`)
    symbol_prefix: &'static str,
    /// Unix 系での静的ライブラリファイル名 (CMake install 後)
    unix_lib_filename: &'static str,
    /// Windows での静的ライブラリファイル名 (CMake install 後)
    win_lib_filename: &'static str,
    /// 共通 CMake 変数 (キー, 値)
    cmake_defines: &'static [(&'static str, &'static str)],
    /// Windows MSVC 固有 CMake 変数 (キー, 値)
    cmake_defines_msvc: &'static [(&'static str, &'static str)],
    /// true の場合、x86_64 ターゲットで NASM 必須
    requires_nasm_x86_64: bool,
}

impl LibraryConfig {
    /// CMake install 直後の **オリジナル名** の静的ライブラリファイル名を返す。
    ///
    /// 例: libyuv → `libyuv.a` / `yuv.lib`、libjpeg-turbo → `libjpeg.a` / `jpeg-static.lib`
    /// シンボル書き換え前のファイル名なので、find_package(JPEG) などの cmake 連携で参照される。
    fn cmake_install_lib_filename(&self) -> &'static str {
        if is_target_windows() {
            self.win_lib_filename
        } else {
            self.unix_lib_filename
        }
    }

    /// `OUT_DIR/lib/` に置く時雨堂プレフィックス付き静的ライブラリファイル名を返す。
    ///
    /// 例: libyuv → `libshiguredo_yuv.a` / `shiguredo_yuv.lib`
    ///     libjpeg-turbo → `libshiguredo_jpeg.a` / `shiguredo_jpeg.lib`
    /// シンボル書き換え後のリンク対象。`prebuilt` / `source-build` 両経路で同じ名前を使う。
    fn staged_lib_filename(&self) -> String {
        if is_target_windows() {
            format!("{}.lib", self.link_name)
        } else {
            format!("lib{}.a", self.link_name)
        }
    }
}

/// libyuv 用の設定
///
/// libyuv にはタグが無いので Cargo.toml ではコミットハッシュで固定している。
const LIBYUV: LibraryConfig = LibraryConfig {
    name: "libyuv",
    link_name: "shiguredo_yuv",
    symbol_prefix: "shiguredo_yuv",
    unix_lib_filename: "libyuv.a",
    win_lib_filename: "yuv.lib",
    cmake_defines: &[],
    cmake_defines_msvc: &[],
    requires_nasm_x86_64: false,
};

/// libjpeg-turbo 用の設定
///
/// MJPEG 系の libyuv 関数 (MJPGToI420 等) は libjpeg-turbo へのリンクが必要。
/// CMAKE_MSVC_RUNTIME_LIBRARY は Rust の MSVC ターゲット (動的 CRT) と揃える。
const LIBJPEG_TURBO: LibraryConfig = LibraryConfig {
    name: "libjpeg-turbo",
    link_name: "shiguredo_jpeg",
    symbol_prefix: "shiguredo_jpeg",
    unix_lib_filename: "libjpeg.a",
    win_lib_filename: "jpeg-static.lib",
    cmake_defines: &[
        ("ENABLE_STATIC", "ON"),
        ("ENABLE_SHARED", "OFF"),
        ("WITH_TURBOJPEG", "OFF"),
        ("WITH_TOOLS", "OFF"),
        ("WITH_TESTS", "OFF"),
    ],
    cmake_defines_msvc: &[("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL")],
    requires_nasm_x86_64: true,
};

// ============================================================
// エントリポイント
// ============================================================

fn main() {
    // Cargo.toml か build.rs が更新されたら、依存ライブラリを再ビルドする
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_SOURCE_BUILD");
    println!("cargo::rerun-if-env-changed=LIBYUV_TARGET");

    // 各種変数やビルドディレクトリのセットアップ
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("infallible"));
    let output_metadata_path = out_dir.join("metadata.rs");
    let output_bindings_path = out_dir.join("bindings.rs");

    // 各種メタデータを書き込む (libyuv の git URL とリビジョン)
    let (git_url, version) = get_git_url_and_version(LIBYUV.name);
    fs::write(
        output_metadata_path,
        format!(
            concat!(
                "pub const BUILD_METADATA_REPOSITORY: &str={:?};\n",
                "pub const BUILD_METADATA_VERSION: &str={:?};\n",
            ),
            git_url, version
        ),
    )
    .expect("failed to write metadata file");

    if env::var("DOCS_RS").is_ok() {
        // Docs.rs 向けのビルドでは git clone ができないので build.rs の処理はスキップして、
        // 代わりに、ドキュメント生成時に最低限必要な定義だけをダミーで出力している。
        // See also: https://docs.rs/about/builds
        //
        // MJPG* 関数のシグネチャは bindgen で生成される C プロトタイプと揃える必要があり、
        // libyuv ヘッダ (`convert.h` / `convert_argb.h`) を直接参照して維持する。
        fs::write(output_bindings_path, docs_rs_dummy_bindings())
            .expect("failed to write dummy bindings");
        return;
    }

    let output_lib_dir = if should_use_prebuilt() {
        download_prebuilt(&out_dir)
    } else {
        build_from_source(&out_dir, &output_bindings_path)
    };

    println!(
        "cargo::rustc-link-search=native={}",
        output_lib_dir.display()
    );
    // libyuv (依存する側) を先、libjpeg (依存される側) を後にリンクする
    println!("cargo::rustc-link-lib=static={}", LIBYUV.link_name);
    println!("cargo::rustc-link-lib=static={}", LIBJPEG_TURBO.link_name);

    // libyuv は C++ コード (convert_jpeg.cc 等) を含むため C++ 標準ライブラリのリンクが必要
    let target = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target.as_str() {
        "linux" => println!("cargo::rustc-link-lib=stdc++"),
        "macos" | "ios" => println!("cargo::rustc-link-lib=c++"),
        _ => {}
    }
}

// source-build feature が有効でなければ prebuilt を使う
fn should_use_prebuilt() -> bool {
    if env::var("CARGO_FEATURE_SOURCE_BUILD").is_ok() {
        return false;
    }
    true
}

// ============================================================
// prebuilt ダウンロード経路
// ============================================================

/// prebuilt バイナリをダウンロードして展開する。
///
/// アーカイブには以下が含まれている前提:
///   - lib/libshiguredo_yuv.a   (Windows: shiguredo_yuv.lib)
///   - lib/libshiguredo_jpeg.a  (Windows: shiguredo_jpeg.lib)
///   - bindings.rs
///   - THIRD_PARTY_LICENSES
fn download_prebuilt(out_dir: &Path) -> PathBuf {
    let target = get_target_platform();
    let version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is not set");
    let base_url = format!(
        "https://github.com/shiguredo/libyuv-rs/releases/download/{}",
        version
    );
    let archive_name = format!("libyuv-{}.tar.gz", target);
    let archive_url = format!("{}/{}", base_url, archive_name);
    let sha256_url = format!("{}/{}.sha256", base_url, archive_name);

    let archive_path = out_dir.join("prebuilt.tar.gz");
    let sha256_path = out_dir.join("prebuilt.sha256");
    let prebuilt_dir = out_dir.join("prebuilt");
    fs::create_dir_all(&prebuilt_dir).expect("failed to create prebuilt directory");

    // curl でアーカイブをダウンロード
    eprintln!("downloading prebuilt library: {}", archive_url);
    let status = Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(&archive_path)
        .arg(&archive_url)
        .status()
        .expect("failed to execute curl. Ensure curl is installed");
    if !status.success() {
        panic!("failed to download prebuilt library: {}", archive_url);
    }

    // curl で SHA256 チェックサムをダウンロード
    let status = Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(&sha256_path)
        .arg(&sha256_url)
        .status()
        .expect("failed to execute curl");
    if !status.success() {
        panic!("failed to download SHA256 checksum: {}", sha256_url);
    }

    // SHA256 を検証
    verify_sha256(&archive_path, &sha256_path);

    // tar で展開
    let status = Command::new("tar")
        .args(["xzf"])
        .arg(&archive_path)
        .arg("-C")
        .arg(&prebuilt_dir)
        .status()
        .expect("failed to execute tar. Ensure tar is installed");
    if !status.success() {
        panic!("failed to extract prebuilt archive");
    }

    // アーカイブに必要なファイルが含まれていることを確認する。
    // prebuilt ファイル名は build_from_source 経路の出力と同一の `libshiguredo_*` 形式を使う。
    let prebuilt_lib_dir = prebuilt_dir.join("lib");
    let yuv_filename = LIBYUV.staged_lib_filename();
    let jpeg_filename = LIBJPEG_TURBO.staged_lib_filename();
    let prebuilt_yuv = prebuilt_lib_dir.join(&yuv_filename);
    let prebuilt_jpeg = prebuilt_lib_dir.join(&jpeg_filename);
    let prebuilt_license = prebuilt_dir.join("THIRD_PARTY_LICENSES");
    if !prebuilt_yuv.exists() {
        panic!("prebuilt archive is missing {}", prebuilt_yuv.display());
    }
    if !prebuilt_jpeg.exists() {
        panic!("prebuilt archive is missing {}", prebuilt_jpeg.display());
    }
    if !prebuilt_license.exists() {
        panic!("prebuilt archive is missing {}", prebuilt_license.display());
    }

    // ライブラリファイルを OUT_DIR/lib/ にコピーする。
    // (リリースビルド時にシンボル書き換えが済んでいる前提なので追加処理は不要)
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).expect("failed to create lib directory");
    fs::copy(&prebuilt_yuv, lib_dir.join(&yuv_filename))
        .unwrap_or_else(|e| panic!("failed to copy {}: {}", yuv_filename, e));
    fs::copy(&prebuilt_jpeg, lib_dir.join(&jpeg_filename))
        .unwrap_or_else(|e| panic!("failed to copy {}: {}", jpeg_filename, e));

    // bindings.rs を OUT_DIR/ にコピー
    fs::copy(
        prebuilt_dir.join("bindings.rs"),
        out_dir.join("bindings.rs"),
    )
    .expect("failed to copy bindings.rs");

    lib_dir
}

// SHA256 チェックサムを検証する
fn verify_sha256(file_path: &Path, sha256_path: &Path) {
    let expected = fs::read_to_string(sha256_path)
        .expect("failed to read SHA256 checksum file")
        .split_whitespace()
        .next()
        .expect("SHA256 checksum file is empty")
        .to_lowercase();

    let actual = compute_sha256(file_path);
    if actual != expected {
        panic!(
            "SHA256 checksum mismatch:\n  expected: {}\n  actual:   {}",
            expected, actual
        );
    }
    eprintln!("SHA256 checksum verified: {}", actual);
}

// ファイルの SHA256 ハッシュを計算する
fn compute_sha256(path: &Path) -> String {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let output = match target_os.as_str() {
        "macos" => {
            // macOS: shasum を使用
            Command::new("shasum")
                .args(["-a", "256"])
                .arg(path)
                .output()
                .expect("failed to execute shasum. Ensure shasum is installed")
        }
        "windows" => {
            // Windows: certutil を使用
            Command::new("certutil")
                .args(["-hashfile"])
                .arg(path)
                .arg("SHA256")
                .output()
                .expect("failed to execute certutil")
        }
        _ => {
            // Linux 他: sha256sum を使用
            Command::new("sha256sum")
                .arg(path)
                .output()
                .expect("failed to execute sha256sum. Ensure coreutils is installed")
        }
    };

    if !output.status.success() {
        panic!("failed to compute SHA256 checksum");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if target_os == "windows" {
        // certutil 出力形式:
        // SHA256 hash of <file>:
        // <hash>
        // CertUtil: -hashfile command completed successfully.
        stdout
            .lines()
            .nth(1)
            .expect("unexpected certutil output format")
            .trim()
            .to_lowercase()
    } else {
        // shasum / sha256sum 出力形式: <hash>  <filename>
        stdout
            .split_whitespace()
            .next()
            .expect("unexpected shasum/sha256sum output format")
            .to_lowercase()
    }
}

// ============================================================
// source-build 経路
// ============================================================

/// libjpeg-turbo のビルド結果。
///
/// libyuv 側のビルド・リンクで参照する 2 つのパスをまとめる:
///   - install_prefix: libyuv の cmake configure 時に `CMAKE_PREFIX_PATH` で渡し、
///     libyuv 側の `find_package(JPEG)` を成功させて `HAVE_JPEG` を有効化する。
///   - rename_map_path: libyuv.a 内の libjpeg 未定義参照シンボルを `shiguredo_jpeg_*` に
///     書き換えるために使う objcopy 用リネームマップ。
struct LibjpegTurboBuildResult {
    install_prefix: PathBuf,
    rename_map_path: PathBuf,
}

/// ソースからビルドする
///
/// 全体のビルドフロー:
///   1. libjpeg-turbo を clone → cmake build → install → コピー → 定義シンボル書き換え
///   2. libyuv を clone → cmake build (HAVE_JPEG を有効化) → install → コピー
///   3. libshiguredo_yuv.a に libjpeg のリネームマップを適用 (未定義参照を書き換え)
///   4. libshiguredo_yuv.a の定義シンボルを `shiguredo_yuv_` で書き換える
///   5. 4. のリネームマップを bindgen の ParseCallbacks に渡してバインディング生成
fn build_from_source(out_dir: &Path, output_bindings_path: &Path) -> PathBuf {
    let out_source_dir = out_dir.join("source/");
    let _ = fs::remove_dir_all(&out_source_dir);
    fs::create_dir(&out_source_dir).expect("failed to create source directory");

    // 出力先 lib ディレクトリを準備する (yuv / jpeg の両方をここに集める)
    let output_lib_dir = out_dir.join("lib");
    fs::create_dir_all(&output_lib_dir).expect("failed to create output lib directory");

    shiguredo_cmake::set_cmake_env();

    // llvm-tools を 1 度だけ解決して使い回す (rustc 起動コスト削減)
    let tools = discover_llvm_tools();

    // ステップ 1, 2, 4: libjpeg-turbo をビルドして `shiguredo_jpeg_*` 化する
    let libjpeg_turbo = build_libjpeg_turbo(out_dir, &out_source_dir, &output_lib_dir, &tools);

    // ステップ 3: libyuv を clone & cmake build & install する (HAVE_JPEG を有効化)
    let libyuv_src_dir = out_source_dir.join(LIBYUV.name);
    git_clone_external_lib(&out_source_dir, &LIBYUV);

    // libyuv の util ツール (cpuid / yuvconvert / yuvconstants) はビルド対象から外す。
    // `util/cpuid.c` には Intel APX 命令 (`vdpphps`) が含まれており、
    // GitHub Actions の binutils ではアセンブルできないため。
    // Rust バインディングからこれらツールは参照しないので静的ライブラリの機能には影響しない。
    patch_libyuv_skip_util_tools(&libyuv_src_dir);

    let mut libyuv_cfg = Config::new(&libyuv_src_dir);
    libyuv_cfg
        .define("BUILD_SHARED_LIBS", "OFF")
        .profile("Release");
    // libyuv の find_package(JPEG) に libjpeg-turbo の install prefix を渡す
    libyuv_cfg.define(
        "CMAKE_PREFIX_PATH",
        libjpeg_turbo.install_prefix.display().to_string(),
    );
    apply_cmake_defines(&mut libyuv_cfg, &LIBYUV);

    let libyuv_install = libyuv_cfg.build();

    // ステップ 5: libyuv の install 出力を `OUT_DIR/lib/libshiguredo_yuv.a` にコピーする
    let libyuv_install_lib = libyuv_install.join("lib");
    let libyuv_src_path = find_static_library(
        &libyuv_install_lib,
        LIBYUV.unix_lib_filename,
        LIBYUV.win_lib_filename,
    );
    let libshiguredo_yuv_path = output_lib_dir.join(LIBYUV.staged_lib_filename());
    fs::copy(&libyuv_src_path, &libshiguredo_yuv_path).unwrap_or_else(|e| {
        panic!(
            "failed to copy {} to {}: {}",
            LIBYUV.cmake_install_lib_filename(),
            libshiguredo_yuv_path.display(),
            e
        )
    });

    // ステップ 6: libshiguredo_yuv.a 内の libjpeg 未定義参照を `shiguredo_jpeg_*` に書き換える
    // `--redefine-syms` は未定義参照シンボルも書き換えるため、jpeg リネームマップを
    // libyuv.a に当てるだけで libyuv → libjpeg 呼び出しが `shiguredo_jpeg_*` に統一される
    rewrite_archive_symbols(
        &tools.objcopy,
        &libshiguredo_yuv_path,
        &libjpeg_turbo.rename_map_path,
    );

    // ステップ 7: libshiguredo_yuv.a の定義シンボルを `shiguredo_yuv_` で書き換える
    let yuv_rename = rename_defined_symbols(
        &tools,
        &libshiguredo_yuv_path,
        out_dir,
        LIBYUV.symbol_prefix,
        "symbol_rename_map_yuv.txt",
    );

    // bindings を生成する (libyuv 側のリネームマップを bindgen に渡す)
    let install_include_dir = libyuv_install.join("include");
    bindgen::Builder::default()
        .clang_arg(format!("-I{}", install_include_dir.display()))
        .header(install_include_dir.join("libyuv.h").display().to_string())
        // libyuv ヘッダから推移的に libjpeg-turbo の型が混入することを防ぐ
        .blocklist_type("jpeg_.*")
        .parse_callbacks(Box::new(yuv_rename.callbacks))
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(output_bindings_path)
        .expect("failed to write bindings");

    output_lib_dir
}

/// libjpeg-turbo を clone → cmake build → install し、定義シンボルを書き換える。
///
/// 戻り値の `install_prefix` は libyuv の `find_package(JPEG)` に渡すために必要。
/// `rename_map_path` は libyuv.a 内の libjpeg 未定義参照を書き換えるために必要。
fn build_libjpeg_turbo(
    out_dir: &Path,
    out_source_dir: &Path,
    output_lib_dir: &Path,
    tools: &LlvmTools,
) -> LibjpegTurboBuildResult {
    // x86_64 ターゲットでは NASM が必須
    ensure_nasm_for(&LIBJPEG_TURBO);

    // libjpeg-turbo を git clone する
    git_clone_external_lib(out_source_dir, &LIBJPEG_TURBO);
    let src_dir = out_source_dir.join(LIBJPEG_TURBO.name);

    // CMake で build → install (install prefix: OUT_DIR/libjpeg-turbo-install/)
    let mut cfg = Config::new(&src_dir);
    cfg.profile("Release");
    apply_cmake_defines(&mut cfg, &LIBJPEG_TURBO);
    let install_prefix = cfg.build();

    // install 後の静的ライブラリは
    //   Unix : <install>/lib/libjpeg.a
    //   Windows: <install>/lib/jpeg-static.lib
    let install_lib_dir = install_prefix.join("lib");
    let installed_static = find_static_library(
        &install_lib_dir,
        LIBJPEG_TURBO.unix_lib_filename,
        LIBJPEG_TURBO.win_lib_filename,
    );

    // 最終リンク用の `libshiguredo_jpeg.a` を OUT_DIR/lib/ に作る
    // (install 出力のオリジナルは libyuv ビルドの find_package(JPEG) でも参照させるので残す)
    let output_path = output_lib_dir.join(LIBJPEG_TURBO.staged_lib_filename());
    fs::copy(&installed_static, &output_path).unwrap_or_else(|e| {
        panic!(
            "failed to copy {} to {}: {}",
            installed_static.display(),
            output_path.display(),
            e
        )
    });

    // 定義シンボルを `shiguredo_jpeg_` で書き換える。
    // 戻り値の `bindgen_map` は libjpeg-turbo の API が Rust 側から呼ばれないので破棄するが、
    // `map_file_path` は libyuv.a に同じマップを適用するために返す。
    let jpeg_rename = rename_defined_symbols(
        tools,
        &output_path,
        out_dir,
        LIBJPEG_TURBO.symbol_prefix,
        "symbol_rename_map_jpeg.txt",
    );

    LibjpegTurboBuildResult {
        install_prefix,
        rename_map_path: jpeg_rename.map_file_path,
    }
}

/// `LibraryConfig` の cmake_defines を `shiguredo_cmake::Config` に流し込む。
///
/// Windows MSVC でビルドする場合は `cmake_defines_msvc` も追加する。
fn apply_cmake_defines(cfg: &mut Config, lib: &LibraryConfig) {
    for (k, v) in lib.cmake_defines {
        cfg.define(*k, *v);
    }
    if is_target_windows() && is_target_msvc() {
        for (k, v) in lib.cmake_defines_msvc {
            cfg.define(*k, *v);
        }
    }
}

/// `LibraryConfig` が x86_64 で NASM 必須としていれば、NASM の存在を確認する。
///
/// NASM が見つからない場合は panic でビルドを停止する。
fn ensure_nasm_for(lib: &LibraryConfig) {
    if !lib.requires_nasm_x86_64 {
        return;
    }
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if arch != "x86_64" {
        return;
    }
    let ok = Command::new("nasm")
        .arg("-v")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        panic!(
            "nasm is required to build {} on x86_64. Install nasm and retry.",
            lib.name
        );
    }
}

// ============================================================
// プラットフォーム検出
// ============================================================

/// ターゲット OS が Windows かどうかを判定する。
fn is_target_windows() -> bool {
    env::var("CARGO_CFG_TARGET_OS")
        .map(|v| v == "windows")
        .unwrap_or(false)
}

/// ターゲット OS が macOS かどうかを判定する。
fn is_target_macos() -> bool {
    env::var("CARGO_CFG_TARGET_OS")
        .map(|v| v == "macos")
        .unwrap_or(false)
}

/// ターゲットの env (例: msvc / gnu) が `msvc` かどうかを判定する。
fn is_target_msvc() -> bool {
    env::var("CARGO_CFG_TARGET_ENV")
        .map(|v| v == "msvc")
        .unwrap_or(false)
}

// CARGO_CFG_TARGET_OS + CARGO_CFG_TARGET_ARCH からプラットフォーム名を生成する
fn get_target_platform() -> String {
    if let Ok(target) = env::var("LIBYUV_TARGET") {
        return target;
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    match (target_os.as_str(), target_arch.as_str()) {
        ("linux", "x86_64") => format!("{}_x86_64", detect_linux_distro()),
        ("linux", "aarch64") => format!("{}_arm64", detect_linux_distro()),
        ("macos", "aarch64") => "macos_arm64".to_string(),
        ("windows", "x86_64") => "windows_x86_64".to_string(),
        _ => panic!("unsupported target: os={}, arch={}", target_os, target_arch),
    }
}

// /etc/os-release から Ubuntu バージョンを検出する
fn detect_linux_distro() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(version) = line.strip_prefix("VERSION_ID=") {
                let version = version.trim_matches('"');
                match version {
                    "22.04" | "24.04" => return format!("ubuntu-{}", version),
                    _ => {}
                }
            }
        }
    }
    panic!(
        "unsupported Linux distribution. \
         set LIBYUV_TARGET environment variable to specify the target explicitly"
    );
}

// ============================================================
// シンボル書き換え
// ============================================================
//
// 他のライブラリとのシンボル衝突を回避するため、静的ライブラリ内の全シンボルに
// プレフィックスを付与する仕組み。
//
// llvm-nm / llvm-objcopy は rustup の llvm-tools コンポーネントに含まれるものを使用する。
// rust-toolchain.toml に components = ["llvm-tools"] の記載が必要。
//
// プラットフォームごとのシンボル形式の違い:
//   - macOS (Mach-O): シンボル先頭に `_` が付く (例: _I420ToNV12)
//   - Linux (ELF): 先頭 `_` なし (例: I420ToNV12)
//   - Windows x64 (COFF): 先頭 `_` なし (例: I420ToNV12)
//
// bindgen の generated_link_name_override は返した文字列に \u{1} プレフィックスを
// 自動付加する。\u{1} はコンパイラに「この名前をそのまま使え（マングリングするな）」と
// 指示するため、プラットフォーム固有のシンボル名 (macOS なら _shiguredo_yuv_I420ToNV12)
// をそのまま返す必要がある。

/// llvm-nm / llvm-objcopy のパスを保持する
struct LlvmTools {
    nm: PathBuf,
    objcopy: PathBuf,
}

/// objcopy 用と bindgen 用の 2 つのリネームマップを保持する
///
/// 2 つのマップが必要な理由:
///   - objcopy_map: ライブラリ内の実シンボル名を書き換えるため、プラットフォーム依存の名前を使う
///   - bindgen_map: Rust コードからリンクする際の名前を指定するため、C シンボル名をキーにする
struct SymbolRenameMaps {
    /// llvm-objcopy の --redefine-syms 用マップ
    ///
    /// キー: 元のシンボル名 (例: macOS なら _I420ToNV12、Linux なら I420ToNV12)
    /// 値: 書き換え後のシンボル名 (例: macOS なら _shiguredo_yuv_I420ToNV12)
    objcopy_map: HashMap<String, String>,

    /// bindgen の #[link_name] 用マップ
    ///
    /// キー: C シンボル名 (プラットフォーム非依存、例: I420ToNV12)
    /// 値: 書き換え後のシンボル名 (プラットフォーム依存、例: macOS なら _shiguredo_yuv_I420ToNV12)
    ///
    /// bindgen は \u{1} プレフィックスを付加してマングリングを抑制するため、
    /// 値にはプラットフォーム固有のシンボル名を格納する必要がある。
    bindgen_map: HashMap<String, String>,
}

/// bindgen の ParseCallbacks 実装
///
/// バインディング生成時に、書き換え後のシンボル名を `#[link_name = "..."]` として付与する。
/// これにより lib.rs 側のコード変更なしでシンボル書き換えが透過的に動作する。
#[derive(Debug)]
struct SymbolLinkNameCallbacks {
    /// C シンボル名 → 書き換え後シンボル名のマップ
    rename_map: HashMap<String, String>,
}

impl bindgen::callbacks::ParseCallbacks for SymbolLinkNameCallbacks {
    /// bindgen がバインディングを生成する際に呼ばれるコールバック
    ///
    /// 戻り値が Some の場合、bindgen は #[link_name = "\u{1}<戻り値>"] を生成する。
    /// \u{1} プレフィックスによりコンパイラのシンボルマングリングが抑制されるため、
    /// 戻り値にはプラットフォーム固有のシンボル名を返す必要がある。
    fn generated_link_name_override(
        &self,
        item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        self.rename_map.get(item_info.name).cloned()
    }
}

/// `rename_defined_symbols` の戻り値。
///
/// bindgen 用コールバックと、生成したマップファイルのパスを併せて返す。
/// マップファイルパスは別ライブラリ (例: libyuv.a) に同じマップを当てるために必要。
struct RenameResult {
    callbacks: SymbolLinkNameCallbacks,
    map_file_path: PathBuf,
}

/// 静的ライブラリの定義済み外部シンボルを収集し、プレフィックスを付与して書き換える。
///
/// 処理の流れ:
///   1. llvm-nm で静的ライブラリの定義済み外部シンボルを収集する
///   2. 収集したシンボルに対してリネームマップを生成する
///   3. マップファイルを書き出し、llvm-objcopy でライブラリ内のシンボルを書き換える
///   4. bindgen 用の ParseCallbacks とマップファイルのパスを返す
fn rename_defined_symbols(
    tools: &LlvmTools,
    lib_path: &Path,
    out_dir: &Path,
    prefix: &str,
    map_filename: &str,
) -> RenameResult {
    let is_macos = is_target_macos();

    // シンボル名の変換ルール
    //   例 (yuv): I420ToNV12 → shiguredo_yuv_I420ToNV12
    //   例 (jpeg): jpeg_create_decompress → shiguredo_jpeg_jpeg_create_decompress
    let rename_symbol = |name: &str| -> Option<String> { Some(format!("{prefix}_{name}")) };

    let symbols = collect_defined_external_symbols(&tools.nm, lib_path);
    let maps = build_symbol_rename_maps(&symbols, is_macos, &rename_symbol);

    let map_file_path = out_dir.join(map_filename);
    write_objcopy_rename_map(&maps.objcopy_map, &map_file_path);
    rewrite_archive_symbols(&tools.objcopy, lib_path, &map_file_path);

    RenameResult {
        callbacks: SymbolLinkNameCallbacks {
            rename_map: maps.bindgen_map,
        },
        map_file_path,
    }
}

/// 静的ライブラリのパスを `<lib_dir>/<unix_name>` または `<lib_dir>/<win_name>` で解決する。
///
/// `cfg!` ではなく `CARGO_CFG_TARGET_OS` を判定基準にすることでクロスコンパイル時の取り違えを避ける。
fn find_static_library(lib_dir: &Path, unix_name: &str, win_name: &str) -> PathBuf {
    let unix_path = lib_dir.join(unix_name);
    if unix_path.exists() {
        return unix_path;
    }
    let win_path = lib_dir.join(win_name);
    if win_path.exists() {
        return win_path;
    }
    panic!(
        "static library not found in {} (looked for {} and {})",
        lib_dir.display(),
        unix_name,
        win_name
    );
}

/// rustc --print sysroot の結果を取得する
///
/// llvm-tools は rustup が管理する sysroot 配下にインストールされるため、
/// sysroot のパスを取得して llvm-nm / llvm-objcopy の探索に使用する。
fn get_rustc_sysroot() -> PathBuf {
    let output = Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .expect("failed to run rustc --print sysroot");
    if !output.status.success() {
        panic!("rustc --print sysroot failed");
    }
    PathBuf::from(
        String::from_utf8(output.stdout)
            .expect("invalid UTF-8")
            .trim(),
    )
}

/// Windows 対応の実行ファイル名を生成する
///
/// Windows では実行ファイルに .exe 拡張子が必要。ホスト OS 基準で判定する
/// (llvm-tools はホスト上で動作するため)。
fn exe_name(name: &str) -> String {
    if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}

/// rustup の sysroot から llvm-nm / llvm-objcopy を探す
///
/// llvm-tools コンポーネントのバイナリは以下のパスに配置される:
///   <sysroot>/lib/rustlib/<host>/bin/llvm-nm
///   <sysroot>/lib/rustlib/<host>/bin/llvm-objcopy
///
/// rust-toolchain.toml に llvm-tools コンポーネントの記載が必要。
///
/// llvm-nm / llvm-objcopy はホスト上で実行するツールなので、クロスコンパイル時は
/// TARGET ではなく HOST のパスから探す必要がある。
fn discover_llvm_tools() -> LlvmTools {
    let sysroot = get_rustc_sysroot();
    let host = env::var("HOST").expect("HOST environment variable not set");
    let tools_dir = sysroot.join("lib/rustlib").join(host).join("bin");

    let nm = tools_dir.join(exe_name("llvm-nm"));
    let objcopy = tools_dir.join(exe_name("llvm-objcopy"));

    if !nm.exists() {
        panic!(
            "llvm-nm not found at {}. Run: rustup component add llvm-tools",
            nm.display()
        );
    }
    if !objcopy.exists() {
        panic!(
            "llvm-objcopy not found at {}. Run: rustup component add llvm-tools",
            objcopy.display()
        );
    }

    LlvmTools { nm, objcopy }
}

/// llvm-nm で静的ライブラリから定義済み外部シンボルを収集する
///
/// 出力にはオブジェクトファイル名 (例: planar_functions.cc.o:) も含まれるため、
/// `is_c_identifier()` でフィルタリングして純粋なシンボル名のみを抽出する。
fn collect_defined_external_symbols(nm_path: &Path, lib_path: &Path) -> Vec<String> {
    let output = Command::new(nm_path)
        .arg("--defined-only")
        .arg("--extern-only")
        .arg("--format=just-symbols")
        .arg(lib_path)
        .output()
        .expect("failed to run llvm-nm");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("llvm-nm failed: {stderr}");
    }

    let stdout = String::from_utf8(output.stdout).expect("llvm-nm output is not valid UTF-8");
    let mut symbols: Vec<String> = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty() && is_c_identifier(s))
        .collect();
    symbols.sort();
    symbols.dedup();
    symbols
}

/// C 識別子として有効かどうかを判定する
///
/// llvm-nm の --format=just-symbols 出力にはオブジェクトファイル名 (planar_functions.cc.o: 等) も
/// 含まれるため、この関数で C 識別子のみをフィルタリングする。
///
/// macOS の Mach-O ではシンボル先頭に `_` が付くため、`_` で始まる文字列も受け入れる。
fn is_c_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

/// objcopy 用と bindgen 用のリネームマップを生成する
fn build_symbol_rename_maps(
    symbols: &[String],
    is_macos: bool,
    rename_symbol: &dyn Fn(&str) -> Option<String>,
) -> SymbolRenameMaps {
    let mut objcopy_map = HashMap::new();
    let mut bindgen_map = HashMap::new();

    for sym in symbols {
        // プラットフォーム固有のプレフィックスを除去して C シンボル名を取得する
        //   macOS: _I420ToNV12 → I420ToNV12
        //   Linux/Windows: I420ToNV12 → I420ToNV12 (変化なし)
        let c_name = if is_macos {
            sym.strip_prefix('_').unwrap_or(sym)
        } else {
            sym.as_str()
        };

        if let Some(new_c_name) = rename_symbol(c_name) {
            // objcopy 用: プラットフォーム固有のプレフィックスを再付与する
            let new_sym = if is_macos {
                format!("_{new_c_name}")
            } else {
                new_c_name.clone()
            };
            objcopy_map.insert(sym.clone(), new_sym.clone());

            // bindgen 用: generated_link_name_override は \u{1} プレフィックスを付加して
            // シンボル名をそのまま使うため、プラットフォーム固有のシンボル名で管理する
            bindgen_map.insert(c_name.to_string(), new_sym);
        }
    }

    SymbolRenameMaps {
        objcopy_map,
        bindgen_map,
    }
}

/// --redefine-syms 用のマップファイルを書き出す
///
/// ファイル形式は 1 行に "旧シンボル名 新シンボル名" を空白区切りで記述する。
fn write_objcopy_rename_map(map: &HashMap<String, String>, path: &Path) {
    let mut lines: Vec<String> = map
        .iter()
        .map(|(old, new)| format!("{old} {new}"))
        .collect();
    // 出力を決定的にするためソートする
    lines.sort();
    fs::write(path, lines.join("\n")).expect("failed to write symbol rename map");
}

/// llvm-objcopy でアーカイブ内のシンボルを書き換える
///
/// --redefine-syms はマップファイルに従ってシンボル名を一括置換する。
/// `--redefine-syms` は未定義参照シンボルも書き換える点に注意。
/// ライブラリファイルはインプレースで更新される。
fn rewrite_archive_symbols(objcopy_path: &Path, lib_path: &Path, map_file: &Path) {
    let status = Command::new(objcopy_path)
        .arg("--redefine-syms")
        .arg(map_file)
        .arg(lib_path)
        .status()
        .expect("failed to run llvm-objcopy");
    if !status.success() {
        panic!("llvm-objcopy failed");
    }
}

// ============================================================
// 外部リポジトリの git clone
// ============================================================

// 外部ライブラリのリポジトリを git clone する
fn git_clone_external_lib(build_dir: &Path, lib: &LibraryConfig) {
    let (git_url, version) = get_git_url_and_version(lib.name);

    let success = Command::new("git")
        .arg("clone")
        .arg(&git_url)
        .arg(lib.name)
        .current_dir(build_dir)
        .status()
        .is_ok_and(|status| status.success());
    if !success {
        panic!("failed to clone {} repository", lib.name);
    }

    let repo_dir = build_dir.join(lib.name);

    let success = Command::new("git")
        .arg("checkout")
        .arg(&version)
        .current_dir(&repo_dir)
        .status()
        .is_ok_and(|status| status.success());
    if !success {
        panic!(
            "failed to checkout commit {} in {} repository",
            version, lib.name
        );
    }
}

// ============================================================
// libyuv CMakeLists.txt パッチ
// ============================================================

/// libyuv の `CMakeLists.txt` から util ツール (cpuid / yuvconvert / yuvconstants) のビルドを除外する。
///
/// 必要性:
///   libyuv commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 以降の `util/cpuid.c` には
///   Intel APX 命令 (`vdpphps`) が含まれており、GitHub Actions の binutils ではアセンブルできない。
///   Rust バインディングからは util ツールは参照しないので、静的ライブラリ (`libyuv.a`) には
///   影響せずビルドをスキップできる。
fn patch_libyuv_skip_util_tools(src_dir: &Path) {
    let cmake_path = src_dir.join("CMakeLists.txt");
    let content = fs::read_to_string(&cmake_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", cmake_path.display(), e));
    let patched = strip_libyuv_util_tools(&content);
    if patched == content {
        panic!(
            "libyuv CMakeLists.txt patch did not match any line. \
             Upstream may have changed; please review the util-tool removal logic."
        );
    }
    fs::write(&cmake_path, patched)
        .unwrap_or_else(|e| panic!("failed to write {}: {}", cmake_path.display(), e));
}

/// `CMakeLists.txt` の本文から util ツール関連の行 (add_executable / target_link_libraries /
/// install) を取り除く。
fn strip_libyuv_util_tools(content: &str) -> String {
    const TOOL_NAMES: &[&str] = &["cpuid", "yuvconvert", "yuvconstants"];
    let mut lines: Vec<&str> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim_start();
        let mentions_tool = TOOL_NAMES.iter().any(|name| line.contains(name));
        let is_tool_line = mentions_tool
            && (trimmed.starts_with("add_executable")
                || trimmed.starts_with("target_link_libraries")
                || trimmed.starts_with("install"));
        if !is_tool_line {
            lines.push(line);
        }
    }
    lines.join("\n")
}

// Cargo.toml から依存ライブラリの Git URL とバージョンタグを取得する
fn get_git_url_and_version(lib_name: &str) -> (String, String) {
    let cargo_toml =
        shiguredo_toml::from_str(include_str!("Cargo.toml")).expect("failed to parse Cargo.toml");
    let cargo_toml = shiguredo_toml::Value::Table(cargo_toml);
    if let Some((Some(git_url), Some(version))) = cargo_toml
        .get("package")
        .and_then(|v| v.get("metadata"))
        .and_then(|v| v.get("external-dependencies"))
        .and_then(|v| v.get(lib_name))
        .map(|v| {
            (
                v.get("git").and_then(|s| s.as_str()),
                v.get("version").and_then(|s| s.as_str()),
            )
        })
    {
        (git_url.to_string(), version.to_string())
    } else {
        panic!(
            "Cargo.toml does not contain a valid [package.metadata.external-dependencies.{}] table",
            lib_name
        );
    }
}

// ============================================================
// Docs.rs 向けダミー bindings
// ============================================================

/// Docs.rs ビルド向けに最低限必要な型・関数定義を埋め込む。
///
/// MJPG* のシグネチャは libyuv の `convert.h` / `convert_argb.h` を直接参照しているため、
/// libyuv 側でシグネチャが変わったときは Cargo.toml の commit hash bump と同時に
/// ここも更新する必要がある。
fn docs_rs_dummy_bindings() -> &'static str {
    concat!(
        "pub type FilterMode = ::std::os::raw::c_uint;\n",
        "pub const FilterMode_kFilterNone: FilterMode = 0;\n",
        "pub const FilterMode_kFilterLinear: FilterMode = 1;\n",
        "pub const FilterMode_kFilterBilinear: FilterMode = 2;\n",
        "pub const FilterMode_kFilterBox: FilterMode = 3;\n",
        "pub type RotationMode = ::std::os::raw::c_uint;\n",
        "pub const RotationMode_kRotate0: RotationMode = 0;\n",
        "pub const RotationMode_kRotate90: RotationMode = 90;\n",
        "pub const RotationMode_kRotate180: RotationMode = 180;\n",
        "pub const RotationMode_kRotate270: RotationMode = 270;\n",
        // MJPGSize: convert.h の `int MJPGSize(const uint8_t* sample, size_t sample_size,\n",
        //                                    int* width, int* height);`
        "unsafe extern \"C\" {\n",
        "    pub fn MJPGSize(\n",
        "        sample: *const u8,\n",
        "        sample_size: usize,\n",
        "        width: *mut ::std::os::raw::c_int,\n",
        "        height: *mut ::std::os::raw::c_int,\n",
        "    ) -> ::std::os::raw::c_int;\n",
        "}\n",
        // MJPGToI420: convert.h
        "unsafe extern \"C\" {\n",
        "    pub fn MJPGToI420(\n",
        "        sample: *const u8,\n",
        "        sample_size: usize,\n",
        "        dst_y: *mut u8, dst_stride_y: ::std::os::raw::c_int,\n",
        "        dst_u: *mut u8, dst_stride_u: ::std::os::raw::c_int,\n",
        "        dst_v: *mut u8, dst_stride_v: ::std::os::raw::c_int,\n",
        "        src_width: ::std::os::raw::c_int,\n",
        "        src_height: ::std::os::raw::c_int,\n",
        "        dst_width: ::std::os::raw::c_int,\n",
        "        dst_height: ::std::os::raw::c_int,\n",
        "    ) -> ::std::os::raw::c_int;\n",
        "}\n",
        // MJPGToNV12: convert.h
        "unsafe extern \"C\" {\n",
        "    pub fn MJPGToNV12(\n",
        "        sample: *const u8,\n",
        "        sample_size: usize,\n",
        "        dst_y: *mut u8, dst_stride_y: ::std::os::raw::c_int,\n",
        "        dst_uv: *mut u8, dst_stride_uv: ::std::os::raw::c_int,\n",
        "        src_width: ::std::os::raw::c_int,\n",
        "        src_height: ::std::os::raw::c_int,\n",
        "        dst_width: ::std::os::raw::c_int,\n",
        "        dst_height: ::std::os::raw::c_int,\n",
        "    ) -> ::std::os::raw::c_int;\n",
        "}\n",
        // MJPGToNV21: convert.h
        "unsafe extern \"C\" {\n",
        "    pub fn MJPGToNV21(\n",
        "        sample: *const u8,\n",
        "        sample_size: usize,\n",
        "        dst_y: *mut u8, dst_stride_y: ::std::os::raw::c_int,\n",
        "        dst_vu: *mut u8, dst_stride_vu: ::std::os::raw::c_int,\n",
        "        src_width: ::std::os::raw::c_int,\n",
        "        src_height: ::std::os::raw::c_int,\n",
        "        dst_width: ::std::os::raw::c_int,\n",
        "        dst_height: ::std::os::raw::c_int,\n",
        "    ) -> ::std::os::raw::c_int;\n",
        "}\n",
        // MJPGToARGB: convert_argb.h
        "unsafe extern \"C\" {\n",
        "    pub fn MJPGToARGB(\n",
        "        sample: *const u8,\n",
        "        sample_size: usize,\n",
        "        dst_argb: *mut u8, dst_stride_argb: ::std::os::raw::c_int,\n",
        "        src_width: ::std::os::raw::c_int,\n",
        "        src_height: ::std::os::raw::c_int,\n",
        "        dst_width: ::std::os::raw::c_int,\n",
        "        dst_height: ::std::os::raw::c_int,\n",
        "    ) -> ::std::os::raw::c_int;\n",
        "}\n",
    )
}
