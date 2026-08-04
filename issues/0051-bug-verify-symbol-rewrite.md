# verify_symbol_rewrite.sh の未解決シンボル検査が Linux / Windows で常に不発になる

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-verify-symbol-rewrite
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`scripts/verify_symbol_rewrite.sh` の「未解決の `jpeg_` / `jsimd_` 参照が残っていないこと」の検査が、Linux / Windows の `llvm-nm -u` 出力形式（bsd 形式）とマッチせず常に成功してしまう。シンボル書き換えのリグレッションを検出できるように修正する。

## 優先度根拠

Medium。

- 検査が機能していないため、シンボル書き換えの壊れ（objcopy 無効化、上流 API 追加等）を CI で検出できない
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
- 検証: 意図的に未解決参照を含むアーカイブでスクリプトが失敗することを確認する。ローカル（macOS）では `clang --target=x86_64-unknown-linux-gnu` / `--target=x86_64-pc-windows-msvc` のクロスコンパイルで ELF / COFF オブジェクトを作成し、**rustup 同梱の `llvm-ar`** でアーカイブ化して実行する（macOS ネイティブの `ar` は ELF / COFF メンバを拒否し空アーカイブになるため使用しない。形式はターゲットオブジェクト形式で決まり、ツールは同一の rustup 同梱 llvm-nm のため、ホスト macOS での ELF / COFF クロス検証で代替できる）。スクリプトは `OUT_DIR/lib/` に `libshiguredo_yuv.a` と `libshiguredo_jpeg.a` の両方が必要で、Assertion 3 が `shiguredo_yuv_MJPG*` の定義を要求するため、検証用アーカイブには未解決参照を含む成果物を置き、`ERROR: ... has unrewritten jpeg_/jsimd_ undefined references` の出力（Assertion 1 / 2 の失敗）で検出を確認する（MJPG 定義欠如による失敗と区別する）
- Windows の `\r` 混入対策は、`llvm-nm -u --format=just-symbols` の出力に `\r` を意図的に付加して検査が失敗することをローカルで確認する（Windows ホスト特有の混入は macOS では再現できないため）
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `scripts/verify_symbol_rewrite.sh` の `llvm-nm -u` に `--format=just-symbols` を追加し、出力に `tr -d '\r'` を通す（Windows の CR 対策）
2. コメントに `--format=just-symbols` の導入理由（プラットフォーム間の出力形式統一）と `tr -d '\r'` の理由（Windows の CR 混入対策。理由を知らないと不要な防御として削除され得る）を追記する（既存の誤検出防止説明は修正後も有効なため維持する）
3. 上記の完了条件の手順（`llvm-ar` 使用）でローカル検証する
4. `CHANGES.md` に `[FIX]` エントリを追加する
