# verify_symbol_rewrite.sh の未解決シンボル検査が Linux / Windows で常に不発になる

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-verify-symbol-rewrite
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`scripts/verify_symbol_rewrite.sh` の「未解決の `jpeg_` / `jsimd_` 参照が残っていないこと」の検査が、Linux / Windows の `llvm-nm -u` 出力形式（bsd 形式）とマッチせず常に成功してしまう。シンボル書き換えのリグレッションを検出できるように修正する。

## 優先度根拠

Medium。

- 検査が機能していないため、シンボル書き換えの壊れ（objcopy 無効化、上流 API 追加等）を CI で検出できない
- 影響範囲: CI の 9 マトリックスのうち 6 件、release の 8 ターゲットのうち 7 件

## 現状

`scripts/verify_symbol_rewrite.sh` の該当箇所は以下:

```bash
"$LLVM_NM" -u "$lib" | grep -E '^_?(jpeg_|jsimd_)[A-Za-z0-9_]+$' | grep -v shiguredo_
```

`llvm-nm -u` の出力形式はプラットフォームで異なる:

- macOS（darwin 形式）: `_jpeg_create_decompress`（裸のシンボル名）→ 現パターンでマッチ
- Linux（ELF, bsd 形式）: `                 U jpeg_create_decompress`（行頭空白 + 型列 `U`）→ 行頭アンカーの `^_?` がマッチせず検査が不発
- Windows（COFF, bsd 形式）: `         U jpeg_create_decompress` → 同様に不発

## 設計方針

`llvm-nm -u` に `--format=just-symbols` を併用し、全プラットフォームで裸のシンボル名を得る。これにより現行パターンがそのまま機能する（実機検証済みの修正案）。

## 完了条件

- Linux / Windows でも未解決の `jpeg_` / `jsimd_` シンボルが残っている場合に検査が失敗すること
- macOS での既存の動作が維持されること
- 検証: 意図的に未解決参照を含むアーカイブでスクリプトが失敗することを確認（ローカル検証でよい）

## 解決方法

1. `scripts/verify_symbol_rewrite.sh` の `llvm-nm -u` に `--format=just-symbols` を追加する
2. 必要に応じてコメント（日本語）を更新する
3. ローカルで検証する
