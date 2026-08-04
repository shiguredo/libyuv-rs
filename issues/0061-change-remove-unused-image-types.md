# 未使用の公開画像型 10 個を削除する

- Priority: Low
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/change-remove-unused-image-types
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/lib.rs` で定義されている公開画像型のうち、ソースコード・テスト・PBT・fuzz のすべてで 1 回も使用されていない型が 10 個ある。使用されない公開型は削除して API サーフェスを整理する。

## 優先度根拠

Low。

- 動作には影響しない（未使用の型定義）
- ただし公開 API の削除は破壊的変更のため、`CHANGES.md` に `[CHANGE]` エントリが必要

## 現状

以下の型は `src/` / `tests/` / `pbt/` / `fuzz/` の全走査で 0 出現（grep 確認済み）:

- `Ab30Image`
- `AyuvImageMut`
- `Yuv24Image`
- `Nv16ImageMut`
- `H010ImageMut`
- `U010ImageMut`
- `H210ImageMut`
- `U210ImageMut`
- `P212Image`
- `P410Image`

これらは 2026.1.0 で公開された型（CHANGES.md に記載あり）のため、削除は後方互換を失う破壊的変更である。一方で対応する変換関数（例: `ab30_to_*` 等）が存在するかは関数ごとに確認が必要（型が未使用でも、対応する変換関数が `Ab30Image` を引数に取る場合は型は使用されているため削除できない）。削除前に各型の対応関数を確認する。

## 設計方針

- 各型について「対応する変換関数が存在し、その関数の引数・戻り値で使われているか」を確認する
- 型が未使用のままのものは削除する
- 型の削除に伴い、対応する関数が型を使う必要がなくなった場合は関数側の検証（`validate`）の扱いも整理する（issue 0057 と連携）
- 削除は公開 API の破壊的変更として `CHANGES.md` に `[CHANGE]` エントリを追加する

## 完了条件

- 未使用と確認された型が削除されていること
- 削除後も `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリ（削除した型の一覧）が追加されていること

## 解決方法

1. 10 型それぞれについて対応関数と使用箇所を確認する
2. 未使用の型を削除する
3. 関連する `#[allow(dead_code)]` / デッド `validate` を整理する（issue 0057 と連携）
4. `CHANGES.md` に `[CHANGE]` エントリを追加する
