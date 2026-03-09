use std::{
    path::{Path, PathBuf},
    process::Command,
};

use shiguredo_cmake::Config;

// 依存ライブラリの名前
const LIB_NAME: &str = "libyuv";
const LINK_NAME: &str = "yuv";

fn main() {
    // Cargo.toml か build.rs が更新されたら、依存ライブラリを再ビルドする
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=build.rs");

    // 各種変数やビルドディレクトリのセットアップ
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("infallible"));
    let out_source_dir = out_dir.join("source/");
    let src_dir = out_source_dir.join(LIB_NAME);
    let output_metadata_path = out_dir.join("metadata.rs");
    let output_bindings_path = out_dir.join("bindings.rs");
    let _ = std::fs::remove_dir_all(&out_source_dir);
    std::fs::create_dir(&out_source_dir).expect("failed to create source directory");

    // 各種メタデータを書き込む
    let (git_url, version) = get_git_url_and_version();
    std::fs::write(
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

    if std::env::var("DOCS_RS").is_ok() {
        // Docs.rs 向けのビルドでは git clone ができないので build.rs の処理はスキップして、
        // 代わりに、ドキュメント生成時に最低限必要な定義だけをダミーで出力している。
        // See also: https://docs.rs/about/builds
        std::fs::write(
            output_bindings_path,
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
            ),
        )
        .expect("write file error");
        return;
    }

    // 依存ライブラリのリポジトリを取得する
    git_clone_external_lib(&out_source_dir);

    // 上流のバグを修正するパッチを適用する
    apply_patches(&src_dir);

    // 依存ライブラリをビルドする
    shiguredo_cmake::set_cmake_env();
    let dst = Config::new(&src_dir)
        .define("BUILD_SHARED_LIBS", "OFF")
        .profile("Release")
        .build();

    // バインディングを生成する
    let install_include_dir = dst.join("include");
    bindgen::Builder::default()
        .clang_arg(format!("-I{}", install_include_dir.display()))
        .header(install_include_dir.join("libyuv.h").display().to_string())
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(output_bindings_path)
        .expect("failed to write bindings");

    println!(
        "cargo::rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo::rustc-link-lib=static={LINK_NAME}");

    // libyuv は C++ コード (convert_jpeg.cc 等) を含むため C++ 標準ライブラリのリンクが必要
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target.as_str() {
        "linux" => println!("cargo::rustc-link-lib=stdc++"),
        "macos" | "ios" => println!("cargo::rustc-link-lib=c++"),
        _ => {}
    }
}

// 外部ライブラリのリポジトリを git clone する
fn git_clone_external_lib(build_dir: &Path) {
    let (git_url, version) = get_git_url_and_version();

    let success = Command::new("git")
        .arg("clone")
        .arg(&git_url)
        .arg(LIB_NAME)
        .current_dir(build_dir)
        .status()
        .is_ok_and(|status| status.success());
    if !success {
        panic!("failed to clone {LIB_NAME} repository");
    }

    let repo_dir = build_dir.join(LIB_NAME);

    let success = Command::new("git")
        .arg("checkout")
        .arg(&version)
        .current_dir(&repo_dir)
        .status()
        .is_ok_and(|status| status.success());
    if !success {
        panic!("failed to checkout commit {version} in {LIB_NAME} repository");
    }
}

// 上流のバグを修正するパッチを適用する
fn apply_patches(repo_dir: &Path) {
    let patches_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("patches");
    if !patches_dir.exists() {
        return;
    }

    let mut patches: Vec<_> = std::fs::read_dir(&patches_dir)
        .expect("failed to read patches directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "patch") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    patches.sort();

    for patch in &patches {
        let success = Command::new("git")
            .arg("apply")
            .arg(patch)
            .current_dir(repo_dir)
            .status()
            .is_ok_and(|status| status.success());
        if !success {
            panic!(
                "failed to apply patch: {}",
                patch.file_name().unwrap().to_string_lossy()
            );
        }
    }
}

// Cargo.toml から依存ライブラリの Git URL とバージョンタグを取得する
fn get_git_url_and_version() -> (String, String) {
    let cargo_toml =
        shiguredo_toml::from_str(include_str!("Cargo.toml")).expect("failed to parse Cargo.toml");
    let cargo_toml = shiguredo_toml::Value::Table(cargo_toml);
    if let Some((Some(git_url), Some(version))) = cargo_toml
        .get("package")
        .and_then(|v| v.get("metadata"))
        .and_then(|v| v.get("external-dependencies"))
        .and_then(|v| v.get(LIB_NAME))
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
            "Cargo.toml does not contain a valid [package.metadata.external-dependencies.{LIB_NAME}] table"
        );
    }
}
