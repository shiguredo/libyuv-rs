# build.rs のクロスコンパイル・source-build・Android・docs.rs 対応の欠損

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-build-critical-issues
- Polished: 2026-06-27
- Reporter: @voluntas

## 目的

`build.rs` において、クロスコンパイル、source-build、Android ターゲット、docs.rs ビルドで致命的な失敗を引き起こす複数の不備を修正する。

## 優先度根拠

以下の問題がそれぞれビルドを破壊する。

- Linux ターゲットへのクロスコンパイル時に panic する
- source-build 時に原因不明の panic を引き起こす
- Android ターゲットで C++ 標準ライブラリがリンクされない
- docs.rs でドキュメント生成に失敗する

これらは本 crate を利用するプロジェクトのビルド可否を左右するため、リリースをブロックするクリティカルな問題である。

## 現状

### 1. Linux ターゲット判定がビルドホストの `/etc/os-release` を読む

`build.rs:592-607` の `get_target_platform` は `CARGO_CFG_TARGET_OS == "linux"` のとき `detect_linux_distro()` を呼ぶが、`detect_linux_distro()` (`build.rs:610-626`) はビルドホストの `/etc/os-release` を読む。macOS / Windows から Linux ターゲットへのクロスコンパイル時には `unsupported Linux distribution` で panic する。

影響:
- macOS / Windows ホストで `cargo build --target x86_64-unknown-linux-gnu` 等を実行すると即座に panic する
- `LIBYUV_TARGET` 環境変数を設定すれば回避可能だが、未設定時のエラーメッセージが不親切

### 2. `build_from_source` で `remove_dir_all` のエラーを無視

`build.rs:376-378` で `let _ = fs::remove_dir_all(&out_source_dir);` としているため、削除に失敗しても次の `fs::create_dir` で `AlreadyExists` となり、原因のわからない panic になる。

影響:
- 前回ビルドの残骸が残っている状態（権限不足、他プロセスのファイルロック等）で source-build を実行すると、"failed to create source directory" という不十分なメッセージで失敗する
- 真の原因（削除失敗）が隠蔽される

### 3. Android ターゲットで C++ 標準ライブラリをリンクしない

`build.rs:158-163` で `CARGO_CFG_TARGET_OS` が `linux` / `macos` / `ios` 以外の場合、C++ 標準ライブラリをリンクしない。libyuv は C++ コードを含むため、Android 等で未解決シンボルが発生する。

影響:
- `cargo build --target aarch64-linux-android` 等で `__cxa_atexit` 等の C++ ランタイムシンボルが未解決となりリンクエラーになる
- Android NDK では `c++_shared` または `c++_static` をリンクする必要がある

### 4. `docs_rs_dummy_bindings` が実際の bindgen 出力より不完全

`build.rs:1059-1133` の docs.rs 用ダミー bindings は `MJPG*` 関数と基本的な enum 定数しか含んでおらず、`DOCS_RS=1 cargo build` で 352 件の未定義シンボルエラーが発生する。

影響:
- docs.rs 上で crate のドキュメントが生成できない
- `src/lib.rs` 等から参照されている他の FFI 関数がダミー bindings に存在しないため、リンク段階で大量の未定義シンボルが発生する

## 設計方針

各問題を `build.rs` 内で個別に修正する。ホスト / ターゲットの区別を正しく行い、docs.rs 用には実際の bindgen 出力を利用する。

- ターゲット判定は `CARGO_CFG_TARGET_*` 系環境変数のみを使用し、ホストファイルシステムへの依存を排除する
- エラー発生時には原因を特定できるメッセージを出し、エラーを隠蔽しない
- Android では NDK の C++ ランタイムを適切にリンクする
- docs.rs では実際の bindgen 出力と同等の bindings を生成するか、事前生成した bindings をリポジトリに含める

## 完了条件

- macOS / Windows ホストから Linux ターゲットへのクロスコンパイルで `LIBYUV_TARGET` 未設定時に適切なエラーメッセージを出す、または正しく判定する
  - 検証: macOS 上で `cargo build --target x86_64-unknown-linux-gnu` を実行し、`unsupported Linux distribution` 以外のメッセージで終了すること
- source-build 時に `remove_dir_all` の失敗が明確なエラーとして報告される
  - 検証: `CARGO_FEATURE_SOURCE_BUILD=1 cargo build` で `out_source_dir` の削除に失敗する状況を作り、削除失敗の原因がメッセージに含まれること
- `cargo build --target aarch64-linux-android` ( 等 ) で C++ 標準ライブラリがリンクされる
  - 検証: Android ターゲットでビルドがリンクエラーなく完了すること
- `DOCS_RS=1 cargo build` が成功する
  - 検証: `DOCS_RS=1 cargo build` が未定義シンボルエラーなく完了すること

## 解決方法

1. `get_target_platform` でホストが Linux 以外の場合のクロスコンパイルでは `LIBYUV_TARGET` を必須とするか、ターゲット情報のみで判定する
   - 案 A: ホストが Linux 以外なら `LIBYUV_TARGET` 未設定で明示的なエラーメッセージを出す
   - 案 B: Linux ターゲットのプラットフォーム名をターゲット情報だけで決定し、prebuilt 配布の命名規則と整合させる
   - どちらを採用するかは実装時に prebuilt 配布戦略と相談して決定する
2. `build_from_source` で `remove_dir_all` の結果を確認し、失敗時は panic メッセージに含める
   - `remove_dir_all` の `Err` を `.unwrap_or_else(|e| panic!("failed to remove source directory {}: {}", out_source_dir.display(), e))` 等で処理する
3. Android 用に `c++_shared` / `c++_static` リンク分岐を追加する
   - `CARGO_CFG_TARGET_OS == "android"` の場合、`cargo::rustc-link-lib=c++_shared` または `c++_static` を出力する
   - どちらをデフォルトとするかは、既存の時雨堂 Android crate との整合性を確認して決定する
4. docs.rs 用に実際の bindgen 出力をファイルに保存し、`include_str!` するか、Docs.rs 用にヘッダをリポジトリに含めて bindgen を実行する
   - 案 A: `source-build` 時に生成した `bindings.rs` を `src/` 以下にコミットし、`DOCS_RS=1` 時は `include_str!` で埋め込む
   - 案 B: `docs.rs` 用に libyuv ヘッダーの最小セットをリポジトリに含め、`bindgen` を実行する
   - 案 A を推奨: docs.rs のビルド環境では git clone も cmake も実行したくないため、静的な bindings をコミットする方が安定する

## 実装上の注意点

- `get_target_platform` の変更は prebuilt ダウンロード URL (`download_prebuilt`) に影響する。`LIBYUV_TARGET` の命名規則と整合させる必要がある
- Android の C++ ランタイム選択 (`c++_shared` vs `c++_static`) は、利用側アプリの NDK 設定と衝突しないよう注意する。一般的には `c++_shared` が推奨される
- docs.rs 用 bindings を `src/` にコミットする場合、libyuv の commit hash を bump するたびに同時更新が必要。更新忘れを防ぐため CI または `scripts/verify_*.sh` での検証を検討する
- `remove_dir_all` のエラー処理を追加しても、`AlreadyExists` 以外の理由で `create_dir` が失敗する可能性は残る。両方のエラーを区別して報告すること
- これらの修正は `build.rs` のみに留める。Rust API やテストロジックの変更は行わない
