# 正式リリース手順を整備する（canary.py の引き下げ対応含む）

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-release-procedure
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`canary.py` は canary 版のインクリメント（`2026.2.0-canary.N` → `.N+1`）と次バージョンの canary 開始（`2026.2.0` → `2026.3.0-canary.0`）の 2 経路しかなく、正式リリース時の「canary 版から正式版への引き下げ」（例: `2026.2.0-canary.5` → `2026.2.0`）の手順が存在しない。リリース手順を整備し、人為的ミス（タグと Cargo.toml の version 不一致等）を防ぐ。

## 優先度根拠

Medium。

- prebuilt のダウンロード URL は `CARGO_PKG_VERSION` をタグ名に使うため（`build.rs`）、Cargo.toml の version とリリースタグの不一致は利用者のビルドを 404 で壊す
- 正式リリース前に手順を整備しないと、リリース時に手動編集ミスが起きやすい

## 現状

- `canary.py`（リポジトリルート）: `update_version` が対応するのは canary 番号のインクリメントと「X.Y.Z → X.(Y+1).0-canary.0」の 2 経路のみ。正式版への引き下げは手動編集に依存
- タグ push（`feature/release-*` 等のタグ）で `.github/workflows/release.yml` が起動し、prebuilt 生成 → GitHub Release → crates.io publish まで一気に進む。タグ名と Cargo.toml の version の整合をワークフロー側で検証していない
- canary タグを push した場合も `publish` ジョブが走り crates.io に canary 版が公開される設計（意図的かどうかの明記がない）
- `canary.py` は `--dry-run` 時にも「Do you want to update the version?」の対話確認が走る

## 設計方針

- 正式リリース手順（canary 版 → 正式版への引き下げ → タグ push → publish）をドキュメント化する（README.md またはリポジトリ内の手順書）
- `canary.py` に正式版への引き下げコマンド（例: `--release` 相当）を追加するか、手順書で手動編集のチェックリストを提供する
- `.github/workflows/release.yml` にタグ名と Cargo.toml の version の整合検証ステップを追加する
- `--dry-run` 時の対話確認をスキップする
- どのブランチの push で publish まで進むのか（canary タグで publish してよいのか）をワークフローに明記する

## 完了条件

- 正式リリース手順がドキュメント化されていること
- `canary.py` が正式版への引き下げに対応している、または手順書で引き下げ手順が明確になっていること
- release.yml にタグと version の整合検証が含まれていること
- タグ push で publish される条件がワークフロー内に明記されていること

## 解決方法

1. リリース手順を調査・整理する（canary から正式への流れ）
2. 手順書を作成する
3. `canary.py` を必要に応じて拡張する
4. release.yml に整合検証を追加する
