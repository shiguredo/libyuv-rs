# release.yml の GitHub Release 権限と crates.io publish の事前検証を追加する

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-release-workflow
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`.github/workflows/release.yml` のリリースワークフローに、GitHub Release 作成の権限（`permissions: contents: write`）の未指定と、crates.io publish の dry-run 事前検証がない問題がある。リリース失敗時の宙ぶらりん状態を防ぐため修正する。

## 優先度根拠

Medium。

- `github-release` ジョブに `permissions: contents: write` がなく、リポジトリのデフォルト権限が read-only だと `gh release create` が 403 で失敗する
- `cargo publish` が一発本番のため、include リストの欠落や README / LICENSE 不足で失敗した場合に「GitHub Release と prebuilt 資産は作成済みだが crates.io 未公開」の宙ぶらりん状態になる

## 現状

- `.github/workflows/release.yml` の `github-release` ジョブ（先頭ジョブ）に `permissions` が未指定。他のジョブ（`build-prebuilt` / `publish`）は明示している
- `publish` ジョブは `cargo publish` を直接実行し、`cargo publish --dry-run` による事前検証がない
- 他ジョブは checkout をコミットハッシュ固定 + バージョンコメント（例: `actions/checkout@de0fac2e... # v6.0.2`）で参照しているが、slack-notify のみ `shiguredo/github-actions/.../slack-notify@main` のブランチ参照になっている（サプライチェーン整合の観点で不統一）

## 設計方針

- `github-release` ジョブに `permissions: contents: write` を明示する
- `publish` ジョブで `cargo publish --dry-run` を先に実行し、成功した場合のみ本番の `cargo publish` を実行する
- slack-notify の参照をコミットハッシュ固定 + バージョンコメントに揃える（`shiguredo/github-actions` のリリース運用に合わせる。可能であれば既存の固定方式を確認する）

## 完了条件

- `github-release` ジョブに `permissions: contents: write` が指定されていること
- `publish` ジョブが dry-run を経てから本番 publish を行うこと
- slack-notify の参照がコミットハッシュ固定になっていること（固定方式の確認が取れた場合）
- ワークフローが正常に動作すること（検証は次のリリース時、またはワークフローのドライラン）

## 解決方法

1. release.yml の `github-release` ジョブに `permissions: contents: write` を追加する
2. `publish` ジョブに `cargo publish --dry-run` ステップを追加する
3. slack-notify の参照を固定する（可能であれば）
