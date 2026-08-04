# SAFETY コメントを実際の検証内容と一致させる

- Priority: Medium
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/refactor-fix-safety-comments
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

unsafe ブロック直前の `// SAFETY: .validate() が全前提条件を検査済み。` というコメントが 100 箇所以上で一律に書かれているが、実際の検査内容（`.validate()` を呼ぶ関数・手動チェックのみの関数・検証ゼロの関数）は関数ごとに異なる。虚偽または不正確な SAFETY コメントを実際の検証内容と一致する記述に修正する。

## 優先度根拠

Medium。

- SAFETY コメントは「unsafe の前提が何によって満たされるか」を示すべき場所であり、虚偽記載はメモリ安全上の誤解を招く
- 実際に検証ゼロの関数（`src/planar.rs` の `mirror_plane`、`src/convert/hardware.rs` の `detile_to_yuy2` 等）に「検証済み」と書かれている箇所は、issue 0042〜0050 の修正で検証が正しくなった後に整合を取る必要がある

## 現状

`// SAFETY: .validate() が全前提条件を検査済み。` という文面が `src/rotate.rs` / `src/scale.rs` / `src/compare.rs` / `src/planar.rs` / `src/convert/*.rs` に広く存在する。実際は以下の 3 パターンに分かれる:

- 画像型の `validate` を呼んでいる関数（正しい）
- `require_c_int` / `checked_buf_size` の手動チェックのみの関数（コメントの「.validate()」が不正確）
- 検証が欠落している関数（虚偽。issue 0042〜0050 の対象）

また `src/compare.rs` の `compute_hamming_distance` は `checked_buf_size` を使っていないのに「require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み」と書かれている。

## 設計方針

- 各 unsafe ブロックの SAFETY コメントを、その関数が実際に行う検査（`validate` / `require_c_int` / `stride チェック` / `checked_buf_size` の列挙）を記述する形に書き直す
- 検証欠落のある関数は、issue 0042〜0050 の修正後に正しい検査が入ったことを確認してから整合させる（本 issue はコメントの正確性のみを目的とする）
- コメントは日本語で、根拠（どの検査がどの前提を満たすか）を明記する

## 完了条件

- `src/` の全 unsafe ブロックの SAFETY コメントが、実際の検査内容と一致していること
- 「.validate() が全前提条件を検査済み」という一律文面が、実際に `.validate()` を呼ぶ関数以外に存在しないこと（grep で確認）
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` の `### misc` にエントリを追加すること

## 解決方法

1. 全 unsafe ブロックの SAFETY コメントを実態に合わせて書き直す
2. 検証欠落が残る関数は issue 0042〜0050 との関連をコメントに明記し、整合を確認する
3. `CHANGES.md` の `### misc` にエントリを追加する
