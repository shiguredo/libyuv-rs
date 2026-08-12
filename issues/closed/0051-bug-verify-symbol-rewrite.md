# verify_symbol_rewrite.sh の未解決シンボル検査が Linux / Windows で常に不発になる

- Priority: Medium
- Created: 2026-08-04
- Completed: 2026-08-12
- Model: DeepSeek V4 Flash
- Branch: feature/fix-verify-symbol-rewrite
- Polished: 2026-08-12
- Reporter: @voluntas

## 目的

`scripts/verify_symbol_rewrite.sh` の「未解決の `jpeg_` / `jsimd_` 参照が残っていないこと」の検査が、Linux / Windows の `llvm-nm -u` 出力形式（bsd 形式）とマッチせず常に成功してしまう。シンボル書き換えのリグレッションを検出できるように修正する。

## 優先度根拠

Medium。

- 検査が機能していないため、シンボル書き換えの壊れ（objcopy 無効化、上流 API 追加等）を CI で検出できない。この検査の固有価値は、クレート自身のリンクでは検出されない未使用メンバを含む出荷アーティファクト（prebuilt 配布）の品質保証にある
- 影響範囲: CI の 9 マトリックスのうち 7 件（Linux 6 + Windows 1。macOS 2 件は正常）、release の 8 ターゲットのうち 7 件（Linux 6 + Windows 1）

## 現状

`scripts/verify_symbol_rewrite.sh` の該当箇所は以下:

```bash
"$LLVM_NM" -u "$lib" | grep -E '^_?(jpeg_|jsimd_)[A-Za-z0-9_]+$' | grep -v shiguredo_
```

`llvm-nm -u` の出力形式はプラットフォームで異なる:

- macOS（darwin 形式）: `_jpeg_CreateDecompress`（裸のシンボル名）→ 現パターンでマッチ
- Linux（ELF, bsd 形式）: `                 U jpeg_CreateDecompress`（行頭空白 + 型列 `U`）→ 行頭アンカーの `^_?` がマッチせず検査が不発
- Windows（COFF, bsd 形式）: `         U jpeg_CreateDecompress` → 同様に不発

なお 2 段目の `grep -v shiguredo_` は、書き換え済みシンボル（`shiguredo_jpeg_jpeg_*` 等）が行頭アンカー `^_?(jpeg_|jsimd_)` の 1 段目で既に除外されるため実質冗長である（無害な防御的記述。コメントの意図は維持する）。

## 設計方針

`llvm-nm -u` に `--format=just-symbols` を併用し、全プラットフォームでシンボル名のみの出力を得る（macOS では `_` プレフィックスが維持されるため、現行パターンの `^_?` がそのまま機能する。bsd 形式の行頭空白 + 型列 `U` は消える）。アーカイブメンバのヘッダ行（`foo.o:` のような）は just-symbols でも出力されるが、末尾 `:` により現行 grep パターンの `[A-Za-z0-9_]+$` で除外される（既存コメントの説明は修正後も有効）。

このオプションは `build.rs` の `collect_defined_external_symbols` が同一ツール（rustc 同梱の llvm-nm）で `--defined-only --extern-only` と併用して使用しており、動作実績がある。`-u`（未定義のみ）との併用はローカル検証で確認する。

なお Windows では `llvm-nm.exe` の出力に `\r` が混入することがあり、行末アンカー `$` が `\r` にマッチしないため、`grep` の前に `tr -d '\r'` を通す必要がある（スクリプト冒頭の host 行の CR 除去と同じ対策。追加する）。

## 完了条件

- Linux / Windows でも未解決の `jpeg_` / `jsimd_` シンボルが残っている場合に検査が失敗すること（Windows は `\r` 混入対策込みで）
- macOS での既存の動作が維持されること（通常ビルドの成果物に対してスクリプトが成功すること。CI の macos-26 / macos-15 がこれを検証する）
- 検証: 意図的に未解決参照を含むアーカイブでスクリプトが失敗することを確認する。ローカル（macOS）では `clang --target=x86_64-unknown-linux-gnu` / `--target=x86_64-pc-windows-msvc` のクロスコンパイルで ELF / COFF オブジェクトを作成し、**rustup 同梱の `llvm-ar`** でアーカイブ化して実行する（macOS ネイティブの `ar` は ELF / COFF メンバを破棄して空アーカイブになる（警告のみで exit 0）ため使用しない。形式はターゲットオブジェクト形式で決まり、ツールは同一の rustup 同梱 llvm-nm のため、ホスト macOS での ELF / COFF クロス検証で代替できる）。検証用アーカイブは、未解決の `jpeg_` / `jsimd_` 参照と `shiguredo_yuv_MJPG*` 定義 5 種（MJPGSize / MJPGToI420 / MJPGToNV12 / MJPGToNV21 / MJPGToARGB。スクリプトの `expected_symbols` と一致させる）を含む C ソースをクロスコンパイルして作成する。スクリプトは `OUT_DIR/lib/` に `libshiguredo_yuv.a` と `libshiguredo_jpeg.a` の両方が必要（置き換えるのは `libshiguredo_yuv.a` のみで足りる。`libshiguredo_jpeg.a` には実物を残す（Assertion 2 が実物に対して実行され通過する）。実物は退避して検証後に復元する）。検証用アーカイブに `shiguredo_yuv_MJPG*` の定義も含めておくと、修正前は exit 0（不発）、修正後は `ERROR: ... has unrewritten jpeg_/jsimd_ undefined references`（Assertion 1 の失敗。Assertion 2 は同一のメッセージ形式）で exit 1 となり、exit コードだけで検出を確認できる（MJPG 定義を含めない場合はエラーメッセージで Assertion 3 の失敗と区別する）。あわせて、書き換え済みの正常な ELF / COFF アーカイブ（未解決の `jpeg_` / `jsimd_` 参照を含まない。`shiguredo_yuv_MJPG*` 定義 5 種は含める）でスクリプトが成功する（偽陽性がない）ことも確認する
- Windows の `\r` 混入対策は、`llvm-nm -u --format=just-symbols` の出力に `\r` を意図的に付加して（例: `sed 's/$/\r/'` でパイプ）、`tr -d '\r'` がないと行末アンカー `$` がマッチせず検出漏れ（検査が通ってしまう）になることと、`tr -d '\r'` を通すと検出されることの両方をローカルで確認する（Windows ホスト特有の混入は macOS では再現できないため）
- `CHANGES.md` の `## develop` の `### misc` セクションに `[FIX]` エントリを追加すること（検証スクリプトの修正はライブラリ機能に直接影響しないため `### misc`。前例: release ワークフローの FIX エントリ）

## 解決方法

`scripts/verify_symbol_rewrite.sh` を次のとおり修正した:

1. `llvm-nm -u` に `--format=just-symbols` を追加した。既定形式では macOS（darwin）が裸のシンボル名（先頭に `_`）だけを出力するのに対し、Linux（ELF）/ Windows（COFF）は bsd 形式で行頭空白 + 型列（U）が付くため、行頭アンカーの `^_?` がマッチせず検査が不発になっていた。just-symbols で出力形式を統一し、macOS の `_` プレフィックス維持も確認した
2. 出力に `tr -d '\r'` を通した（Windows の CR 混入対策。行末アンカー `$` が `\r` にマッチしないため）
3. コメントを書き直した（`--format=just-symbols` の導入理由、`tr -d '\r'` の理由、`grep -v shiguredo_` が 1 段目の行頭アンカーで除外済みの実態に合わせた説明、llvm-nm の失敗が set -e で検出されるためのコマンド置換と改行復元の説明）
4. 完了条件の手順でローカル検証した: `clang --target=x86_64-unknown-linux-gnu` / `--target=x86_64-pc-windows-msvc` のクロスコンパイルで ELF / COFF オブジェクトを作成し、rustup 同梱の `llvm-ar` でアーカイブ化して実行した。未解決参照入りで exit 1（検出）、正常で exit 0（偽陽性なし）、macOS 実物で exit 0（既存動作維持）、`\r` 混入時は `tr -d '\r'` なしで検出漏れ・ありで検出、破損アーカイブで exit 1 を確認した
5. `CHANGES.md` の `## develop` の `### misc` に `[FIX]` エントリを追加した
