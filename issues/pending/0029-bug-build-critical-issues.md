# build.rs における致命的なビルドシステムの不備

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-build-critical-issues
- Polished: {YYYY-MM-DD}
- Reporter:

## 目的

`build.rs` において、クロスコンパイル、source-build、Android ターゲット、docs.rs ビルドで致命的な失敗を引き起こす複数の不備を修正する。

## 優先度根拠

以下の問題がそれぞれビルドを破壊する。

- Linux ターゲットへのクロスコンパイル時に panic する
- source-build 時に原因不明の panic を引き起こす
- Android ターゲットで C++ 標準ライブラリがリンクされない
- docs.rs でドキュメント生成に失敗する

## 現状

### 1. Linux ターゲット判定がビルドホストの `/etc/os-release` を読む

`build.rs:592-607` / `build.rs:610-626` の `get_target_platform` は `CARGO_CFG_TARGET_OS == "linux"` のとき `detect_linux_distro()` を呼ぶが、これはビルドホストの `/etc/os-release` を読む。macOS / Windows から Linux ターゲットへのクロスコンパイル時には `unsupported Linux distribution` で panic する。

### 2. `build_from_source` で `remove_dir_all` のエラーを無視

`build.rs:376-378` で `let _ = fs::remove_dir_all(&out_source_dir);` としているため、削除に失敗しても次の `fs::create_dir` で `AlreadyExists` となり、原因のわからない panic になる。

### 3. Android ターゲットで C++ 標準ライブラリをリンクしない

`build.rs:158-163` で `CARGO_CFG_TARGET_OS` が `linux` / `macos` / `ios` 以外の場合、C++ 標準ライブラリをリンクしない。libyuv は C++ コードを含むため、Android 等で未解決シンボルが発生する。

### 4. `docs_rs_dummy_bindings` が実際の bindgen 出力より不完全

`build.rs:1059-1133` の docs.rs 用ダミー bindings は `MJPG*` 関数と基本的な enum 定数しか含んでおらず、`DOCS_RS=1 cargo build` で 352 件の未定義シンボルエラーが発生する。

## 設計方針

各問題を `build.rs` 内で個別に修正する。ホスト / ターゲットの区別を正しく行い、docs.rs 用には実際の bindgen 出力を利用する。

## 完了条件

- macOS / Windows ホストから Linux ターゲットへのクロスコンパイルで `LIBYUV_TARGET` 未設定時に適切なエラーメッセージを出す、または正しく判定する
- source-build 時に `remove_dir_all` の失敗が明確なエラーとして報告される
- `cargo build --target aarch64-linux-android` ( 等 ) で C++ 標準ライブラリがリンクされる
- `DOCS_RS=1 cargo build` が成功する

## 解決方法

1. `get_target_platform` でホストが Linux 以外の場合のクロスコンパイルでは `LIBYUV_TARGET` を必須とするか、ターゲット情報のみで判定する
2. `build_from_source` で `remove_dir_all` の結果を確認し、失敗時は panic メッセージに含める
3. Android 用に `c++_shared` / `c++_static` リンク分岐を追加する
4. docs.rs 用に実際の bindgen 出力をファイルに保存し、`include_str!` するか、Docs.rs 用にヘッダをリポジトリに含めて bindgen を実行する
