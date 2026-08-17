# verify_symbol_rewrite.sh の未解決シンボル検査が jpeg_/jsimd_ 以外の書き換え対象を検出できない

- Priority: Medium
- Created: 2026-08-12
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-verify-symbol-rewrite-coverage
- Polished: 2026-08-17
- Reporter: @voluntas

## 目的

`scripts/verify_symbol_rewrite.sh` の未解決シンボル検査が `jpeg_` / `jsimd_` のみを対象としており、build.rs が jpeg 側で書き換えている他の定義済み外部シンボル（`jinit_*` / `jpeg12_*` / `jdiv_round_up` 等）が未書き換えのまま残っても検出できない。検査範囲を jpeg リネームマップ（`symbol_rename_map_jpeg.txt`）の書き換え対象と一致させる。yuv 定義シンボルのリネーム（`symbol_rename_map_yuv.txt` / Assertion 3）は対象外である。

## 優先度根拠

Medium。

- 0051 の修正で Linux / Windows の検査不発は解消されたが、検査対象が `jpeg_` / `jsimd_` に限定されたままである。現行 grep が拾わないマップ旧名（`jinit_*` / `jpeg12_*` 等）だけが未定義として残る場合、検査を通過する（完了条件の `jinit_master_decompress` 注入がこのケース）
- この検査の固有価値は、クレート自身のリンクでは検出されない未使用メンバを含む出荷アーティファクト（prebuilt 配布）の品質保証にある。漏れの実戦場は主に `libshiguredo_jpeg.a` のオブジェクト間未定義参照である（libyuv から libjpeg への未定義参照は `jpeg_*` 公開 API のみで、現行パターンでも拾える）

## 現状

- `build.rs` の `collect_defined_external_symbols` は静的ライブラリの全定義済み外部シンボルを収集し、`rename_defined_symbols` が全部に `shiguredo_jpeg_` プレフィックスを付与する。マップは `OUT_DIR/symbol_rename_map_jpeg.txt` に書き、同じマップを `libshiguredo_yuv.a` の未定義参照にも `rewrite_archive_symbols` で当てる（0022 の方針。0051 は `--format=just-symbols` による出力形式の統一と `tr -d '\r'` の追加だけを行い、grep パターンは変更していない）
- libjpeg-turbo 3.1.90 の実マップ（290 件）では、現行 grep `^_?(jpeg_|jsimd_)[A-Za-z0-9_]+$` が拾うのは 190 件（`jpeg_` 92 + `jsimd_` 98）である。拾えない例: `jinit_*`（34）、`jpeg12_*`（26。grep パターンの `jpeg_` は `jpeg` の直後に `_` を要求するため `jpeg12_` にはマッチしない）、`j12init_*` / `j16init_*`、`jpeg16_*`、`jcopy_*`、`jdiv_round_up` / `jround_up` / `jzero_far`
- `scripts/verify_symbol_rewrite.sh` の `check_unresolved` は上記 grep のみである。0051 で入れた `--format=just-symbols` と `tr -d '\r'` はそのまま使う。進捗ログと ERROR は `jpeg_/jsimd_` 専用の文言のままである

## 設計方針

`$OUT_DIR/symbol_rename_map_jpeg.txt` の第 1 フィールド（空白 1 個区切りの書き換え前名。シンボル名に空白は無い）と、`llvm-nm -u --format=just-symbols` の未定義シンボルを突合する。比較の前に、両側の先頭 `_` を高々 1 個落とす（現行 grep の `^_?` と同じ。macOS のマップ左列は `_jinit_*`、ELF / COFF の nm は `jinit_*` のため、0051 のクロス足場でも一致する）。正規化後は **完全一致**（集合所属）で判定する。判定の方向は「未定義シンボルがマップ旧名（第 1 フィールド）の集合に属するか」のみ（逆方向の全列挙はしない）。マップに含まれない `jpeg_` / `jsimd_` 接頭辞の参照は従来 grep の付随的カバレッジから外れるが、定義元が無い参照はクレート自身のリンクで検出されるため実害は限定的。部分一致の `grep -F` は使わない（旧名 `jinit_master_decompress` は右列 `shiguredo_jpeg_jinit_master_decompress` の部分文字列であり、書き換え済みアーカイブの未定義に右列が残るため常に失敗する）。

第 2 フィールド（`shiguredo_jpeg_*`）は使わない。書き換え済みアーカイブの未定義に右列が残る（yuv から jpeg 公開 API への未定義参照、jpeg のオブジェクト間参照）ため、両列を集合にすると正常アーカイブが常に失敗する。行全体やタブ区切り前提の切り出しもしない。マップ末尾に改行が無い（`write_objcopy_rename_map` の `lines.join("\n")`）ため、最終行を落とす読み方（`while read`）は使わない。

マップは `rename_defined_symbols` が既に書いており、スクリプトは既に `OUT_DIR` を引数に取る。`build.rs` / `ci.yml` / `release.yml` は変更しない。マップファイルが無い・空なら検査をスキップせずエラーにする（CI / release の Verify は source-build 直後なのでマップは存在する。prebuilt 消費経路では本スクリプトは走らない）。Windows の `\r` 除去（0051）は維持する。完全一致にしたあとでも `jpeg_foo\r` は `jpeg_foo` と一致しないため、`tr -d '\r'` は行末 `$` のためではなくこの一致のため残す。

`llvm-nm` の失敗は 0051 どおりコマンド置換で受け、`set -e` でスクリプトを落とす。`if` 条件のパイプラインに置くと grep 不発で検査が通ってしまう。

ERROR / 進捗ログ / ファイル先頭の Assertion 説明は、`jpeg_/jsimd_` 専用ではなくマップ旧名の未書き換え全般を指す文言に更新する（0051 の識別は exit 1 と新しい ERROR 文字列で行う）。

採用しない:

- grep の族リストを `jinit_` / `j12` / `j16` 等へ拡大する。今日の実マップでも `jpeg12_*` / `jpeg16_*` / `jcopy_*` / `jdiv_round_up` / `jround_up` / `jzero_far` が漏れる。上流で族が増えるたびに再列挙が要り、目的（書き換え対象との一致）を持続できない
- `collect_defined_external_symbols` をスクリプト向けに新たにエクスポートする。同じ集合はマップ左列として既にある
- 本番のリネームフロー（`rename_defined_symbols`）を再構成する
- 先頭 `_` を落とさず「そのまま一致」させる。macOS マップと ELF / COFF の nm が一致しない
- 部分一致で突合する。右列名経由で正常アーカイブが常に失敗する

## 完了条件

- 現行 grep が拾わないマップ旧名（例: C 名 `jinit_master_decompress`。macOS マップ左列は `_jinit_master_decompress`）の未書き換え参照を含むアーカイブで検査が失敗すること（カバレッジ拡大の実証のため、同じアーカイブを修正前スクリプトで実行し exit 0（不発）になることも確認する）。このランには `jpeg_` / `jsimd_` を混ぜない（混ぜると修正前スクリプトでも `jpeg_` で落ち、カバレッジ拡大を検証できない）。足場は 0051 と同じ（clang の ELF / COFF クロス、rustup 同梱 `llvm-ar`、置き換えるのは `libshiguredo_yuv.a` のみ、`libshiguredo_jpeg.a` は実物を残す、`shiguredo_yuv_MJPG*` 定義 5 種で Assertion 3 と区別する）。仮 `OUT_DIR` を使う場合は `symbol_rename_map_jpeg.txt` も置く。注入名は C 識別子（先頭 `_` なし）でよい（突合前に両側の `_` を落とす）
- 既存の `jpeg_` / `jsimd_` 未書き換えも、これまでどおり失敗すること。これは **別アーカイブ・別ラン** で確認する（注入例: `jpeg_CreateDecompress`。マップ左列に実在する公開 API）
- 正常なアーカイブ（書き換え済み。libc の `memcpy` 等はマップに無く、未定義の `shiguredo_jpeg_*` は右列なので **完全一致** ではヒットしない）で検査が成功すること（偽陽性がないこと）
- macOS での既存の動作が維持されること（通常ビルドの `OUT_DIR` に対してスクリプトが成功すること）
- マップファイル欠落時および空マップ時はスキップせず失敗すること
- `ci.yml` / `release.yml` の Verify symbol rewrite ステップが成功すること（ワークフローファイルは変更しない。スクリプトは `cargo test` では実行されない）
- `CHANGES.md` の `## develop` の `### misc` セクションに `[FIX]` エントリを追加すること（検証スクリプトの修正はライブラリ機能に直接影響しないため `### misc`。0051 と同じ）

## 解決方法

1. `scripts/verify_symbol_rewrite.sh` の `check_unresolved` を、マップ第 1 フィールドと `llvm-nm -u --format=just-symbols` の **完全一致** 突合に替える（判定は「未定義シンボルがマップ旧名の集合に属するか」。先頭 `_` を高々 1 個落とす。`--format=just-symbols` と `tr -d '\r'` と nm 失敗のコマンド置換は 0051 のまま。ERROR / 進捗ログ / Assertion 説明の `jpeg_/jsimd_` 専用文言を更新し、ERROR には検出した未書き換えシンボル名を含める）
2. 検査範囲の根拠（jpeg マップ第 1 フィールドと一致させること、右列は使わないこと、部分一致は使わないこと、最終行を落とさないこと）をコメントで明記する
3. 0051 と同じ足場で、**(a) と (b) は別ラン** とする。(a) C 名 `jinit_master_decompress` のみの未書き換え検出、(b) `jpeg_CreateDecompress` の未書き換え検出、(c) 書き換え済みでの偽陽性なし、(d) macOS 実物成功、を確認する
4. `CHANGES.md` の `## develop` の `### misc` セクションに `[FIX]` エントリを追加する
