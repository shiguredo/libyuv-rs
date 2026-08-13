# verify_symbol_rewrite.sh の未解決シンボル検査が jpeg_/jsimd_ 以外の書き換え対象を検出できない

- Priority: Medium
- Created: 2026-08-12
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-verify-symbol-rewrite-coverage
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`scripts/verify_symbol_rewrite.sh` の未解決シンボル検査が `jpeg_` / `jsimd_` のみを対象としており、libjpeg-turbo の他のシンボル族（`jinit_*` / `jerr_*` / `j12*` / `j16*` 等）が未書き換えのまま残っても検出できない。検査範囲を build.rs の書き換え対象と一致させる。

## 優先度根拠

Medium。

- 0051 の修正で Linux / Windows の検査不発は解消されたが、検査対象のシンボル族が限定的なまま。書き換えが部分的に壊れた場合（objcopy の一部無効化、上流 API 追加等）、`jinit_*` 参照だけが未書き換えで残っても検査を通過する
- この検査の固有価値は、クレート自身のリンクでは検出されない未使用メンバを含む出荷アーティファクト（prebuilt 配布）の品質保証にある

## 現状

- `build.rs` の `collect_defined_external_symbols` は静的ライブラリの**全定義済み外部シンボル**を収集し、`build_symbol_rename_maps` で全部に `shiguredo_jpeg_` プレフィックスを付与する（0051 のレビューで確認済み）
- libjpeg-turbo には `jpeg_*` / `jsimd_*` 以外にも `jinit_*`（`jinit_master_decompress` 等）、`jerr_*`、`j12*` / `j16*` 系（12bit / 16bit 対応の `j12init_*` / `j16init_*` 等）、`jdiv_round_up` 等のシンボル族が存在し、これらも `shiguredo_jpeg_` に書き換えられている
- `scripts/verify_symbol_rewrite.sh` の `check_unresolved` は `grep -E '^_?(jpeg_|jsimd_)'` のみを検査するため、`jinit_*` 等の未書き換え参照は検出されない（実物の `libshiguredo_jpeg.a` で `shiguredo_jpeg_j12init_*` / `shiguredo_jpeg_j16init_*` 等の定義を確認済み）

## 設計方針

次のいずれかを採用する（本 issue で確定する）:

- 検査パターンを書き換え対象の全シンボル族（`jpeg_` / `jsimd_` / `jinit_` / `jerr_` / `j12` / `j16` 等）に拡大する
- または、`collect_defined_external_symbols`（書き換え前のシンボル一覧）をスクリプトに渡して、未解決シンボルと書き換え前シンボルの突合で判定する

後者は build.rs との依存が生じるが、上流でシンボル族が増えても追従できる。前者は grep パターンのメンテナンスが必要。実装時に build.rs の書き換えフローを確認して確定する（libyuv 更新時に見直すべき箇所としてコメントに明記する）。

## 完了条件

- `jinit_*` 等の未書き換え参照を含むアーカイブで検査が失敗すること（0051 と同じ llvm-ar を使った ELF / COFF クロス検証手順）
- 正常なアーカイブ（書き換え済み）で検査が成功すること（偽陽性がないこと）
- 既存の `jpeg_` / `jsimd_` 検出が維持されること
- macOS での既存の動作が維持されること
- `cargo test --workspace --features source-build` と CI の Verify symbol rewrite ステップが成功すること
- `CHANGES.md` の `## develop` の `### misc` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `scripts/verify_symbol_rewrite.sh` の検査対象を書き換え対象の全シンボル族に拡大する（設計方針で確定した方式）
2. 検査範囲の根拠（build.rs の書き換え対象との一致）をコメントで明記する
3. 0051 と同じ検証手順（llvm-ar + clang クロスコンパイル）で未書き換え検出・偽陽性なし・既存維持を確認する
4. `CHANGES.md` の `## develop` の `### misc` セクションに `[FIX]` エントリを追加する
