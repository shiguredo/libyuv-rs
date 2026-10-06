# THIRD_PARTY_LICENSES に libyuv のライセンスを追加し README に記載する

- Priority: High
- Created: 2026-10-06
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4.1 Flash
- Branch: develop
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

prebuilt アーカイブおよびクレートに同梱する `THIRD_PARTY_LICENSES` に libyuv のライセンス条文が欠落している。libyuv は BSD 3-Clause ライセンスで、バイナリ形式で再配布する場合は著作権表示・条件・免責事項をドキュメント等に再掲することが条件となる。`libshiguredo_yuv.a` を静的ライブラリとして配布している現状はライセンス条件を満たしていないため、条文を追加する。

## 優先度根拠

High。

- `THIRD_PARTY_LICENSES` の見出しは libjpeg-turbo の 1 件のみで、libyuv の記載が無い（`grep -i libyuv THIRD_PARTY_LICENSES` が 0 件）
- `release.yml` の `Create prebuilt archive` と `mobile.yml` の `Create prebuilt archive` は prebuilt アーカイブに `THIRD_PARTY_LICENSES` を同梱して配布するため、欠落は配布物そのものの欠陥になる
- libyuv の LICENSE は BSD 3-Clause で、バイナリ再配布時の通知再掲が条件（ライセンス条件違反）
- `Cargo.toml` の `include` は `/THIRD_PARTY_LICENSES` を含むため crates.io に公開されるパッケージにも影響する

## 現状

- `THIRD_PARTY_LICENSES` の先頭は `# libjpeg-turbo (tag 3.1.90, commit e1dbfa7be7b7e54922020051dc77781e92739700)` の 1 セクションのみ
- `scripts/verify_license_hash.sh` は `THIRD_PARTY_LICENSES` の 1 行目から libjpeg-turbo の tag / commit を抽出して検証するため、1 行目の見出しは libjpeg-turbo のまま維持する必要がある
- `README.md` には libyuv のライセンス条文を記載した節が無い（他プロジェクトでは dav1d-rs の `## dav1d ライセンス` のように本体ライブラリのライセンス節を置いている）

## 設計方針

- `THIRD_PARTY_LICENSES` に libyuv のセクションを追加する。見出しは `# libyuv (commit eb8eda9c7973704d1103a535059224df7a6063b5)` の形式とし、libjpeg-turbo の後に置く（1 行目は `scripts/verify_license_hash.sh` が参照するため変更しない）
- 条文は libyuv の `LICENSE`（build.rs が参照する commit）から原文のまま転記する
- `README.md` に `## libyuv ライセンス` 節を追加し、条文と参照 URL を記載する（dav1d-rs / aom-rs / libvpx-rs と同じ構成）
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加する

## 完了条件

- `THIRD_PARTY_LICENSES` に libyuv の BSD 3-Clause 条文が含まれていること
- `scripts/verify_license_hash.sh` が引き続き成功すること（1 行目の libjpeg-turbo 見出しが維持されていること）
- `README.md` に libyuv のライセンス節が追加されていること
- prebuilt アーカイブ (`release.yml` / `mobile.yml`) に同梱される `THIRD_PARTY_LICENSES` に反映されること
- `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリが追加されていること

## 解決方法

1. `THIRD_PARTY_LICENSES` の libjpeg-turbo セクションの後に libyuv の BSD 3-Clause 条文を追加する
2. `README.md` に `## libyuv ライセンス` 節を追加する
3. `CHANGES.md` の `## develop` の `### misc` に `[ADD]` エントリを追加する
4. `bash scripts/verify_license_hash.sh` で 1 行目の見出しが壊れていないことを確認する
