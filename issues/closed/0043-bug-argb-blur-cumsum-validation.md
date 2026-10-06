# argb_blur の cumsum 検証が C のドキュメント契約と不一致で省メモリ利用を拒否する

- Priority: Medium
- Created: 2026-08-04
- Completed: 2026-08-12
- Model: DeepSeek V4 Flash
- Branch: feature/fix-argb-blur-cumsum-validation
- Polished: 2026-08-12
- Reporter: @voluntas

## 目的

`src/planar.rs` の `argb_blur` が cumsum バッファとして `stride32_cumsum * size.height` 要素を要求しており、libyuv が文書化している省メモリ利用（`radius * 2 + 2` 行バッファ）を Err で拒否する。検証を C 側 `ARGBBlur` の実際の必要行数（最大 `min(height, radius * 2 + 2)` 行）に合わせる。

## 優先度根拠

Medium。

- 不具合の実態は「C のドキュメント契約どおりの省メモリバッファを Err で拒否する」ことであり、安全性（領域外アクセス）の問題ではない
- 影響は radius が小さいほど顕著で、必要バッファ量の差が大きくなる（例: 3840x2160、radius=1 で約 246 KB と約 133 MB の差。最小ストライド `stride32_cumsum = width * 4` 前提）

## 現状

libyuv の `ARGBBlur`（`planar_functions.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は cumsum を循環バッファとして使用し、ヘッダのコメントに以下を明記している:

> Caller should allocate CumulativeSum table of width * height * 16 bytes aligned to 16 byte boundary. height can be radius * 2 + 2 to save memory as the buffer is treated as circular.

C 実装では `radius` は `min(radius, height)` と `min(radius, width / 2 - 1)` に clamp され、`max_cumsum_bot_row = &dst_cumsum[(radius * 2 + 2) * dst_stride32_cumsum];` をラップ境界として循環させる。cumsum への書き込みは合計 `height` 回（初期の cumsum 計算 `ARGBComputeCumulativeSum` を高さ `radius` で呼んだ `radius` 回 + メインループ内 `(y + radius) < height` のときのみ進行する高々 `height - radius` 回）だが、循環バッファにより書き込み先の行インデックスは 0 〜 `min(height - 1, radius * 2 + 1)` の範囲に収まる。つまり必要行数は最大 `min(height, radius * 2 + 2)` 行である。

一方 `src/planar.rs` の `argb_blur` は `cumsum.len() >= stride32_cumsum * size.height` で検証する。`height > radius * 2 + 2` のとき（radius が小さいほど顕著）、C の契約どおり `radius * 2 + 2` 行のバッファで足りるのに `height` 行を要求して Err を返す。例: 3840x2160、radius=1（最小ストライド）では C は 4 行（約 246 KB）で処理できるが、現行検証は 2160 行（約 133 MB）を要求する。また `argb_blur` の docstring は cumsum の必要サイズを一切明記しておらず、呼び出し側は C 側のドキュメントを読まないとサイズ要件を知れない。

## 設計方針

`argb_blur` の cumsum 検証を C 側の実使用行数で行う:

- 有効 radius を C と同じ規則（`min(radius, height)`、`min(radius, width / 2 - 1)`）で計算する。`width / 2 - 1` は usize でそのまま計算すると width == 0 / 1 でアンダーフローするため、width <= 3 は有効 radius の計算より先に `Err` と判定する（width <= 3 では C の clamp 後も有効 radius が 0 以下になり `-1` を返す条件と一致する）
- 必要な行数 = `min(size.height, effective_radius * 2 + 2)` として `checked_buf_size` で検証する。この式は C の実際の最大書き込み行数と一致するため、緩和後も領域外書き込みは発生しない
- `radius <= 0` / `height <= 1` は C が `-1` を返すため `Err` にする（width / height == 0 のゼロサイズ入力も同様。新規チェックは追加せず現行どおり C 経由の Err を維持し、ゼロサイズのエラーメッセージ形式の統一は 0064 のスコープに委譲する）
- docstring に cumsum の必要行数（`min(height, radius * 2 + 2)` 行）と、radius が C と同じ規則で clamp される旨を明記する（stride の単位が i32 要素であることは既に記載済み）
- 既存の `stride32_cumsum >= width * 4` チェック（1 行あたり ARGB の 4 チャンネル分）は維持する

なお `argb_compute_cumulative_sum` は C の `ARGBComputeCumulativeSum` がちょうど `height` 行書き込むため、現行の検証（`height` 行要求）が正しく、変更対象外とする。

## 完了条件

- `height > effective_radius * 2 + 2` のとき、必要行数（`min(height, effective_radius * 2 + 2)` 行）ちょうどの cumsum バッファで `Ok` になり、1 行不足（`(必要行数 - 1) * stride32_cumsum` 要素）で `Err` になること
- `width / 2 - 1 >= height` のとき、radius == height の入力の必要行数が `height` 行（現行と同じ要求）になること
- radius > height のとき clamp 後が radius == height と同じ挙動になること
- radius <= 0、height <= 1、width <= 3 で `Err` になること（C の clamp 後に radius <= 0、または height <= 1 の `-1` 返却条件と一致）
- 上記の期待値は C の clamp 規則（`min(radius, height)` / `min(radius, width / 2 - 1)`）と `radius <= 0 || height <= 1` の `-1` 返却から導出したものであり、実装時にそのままテストの期待値として使う
- 検証式 `min(height, radius * 2 + 2)` が C の最大書き込み行数と一致する根拠（循環バッファのラップ境界）が、実装のコメントに明記されていること（libyuv 更新時に見直すべき箇所として）
- `tests/test_planar.rs` に上記の境界値テストが追加されていること（0053 でテストファイルが分割された場合は `tests/test_planar/` 配下。argb_blur の cumsum 境界値テストは本 issue のスコープで追加し、0053 の選定対象外とする。0053 側にも 0042 / 0049 と同様に「0043 の対象分は選定対象外」の追記が必要なため、0053 の設計方針の該当箇所（関数固有のエラーパスの例示）から `argb_blur` の cumsum 不足を外すか、除外の注記を追加すること）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

`src/planar.rs` の `argb_blur` を次のとおり修正した:

1. `width <= 3`、`radius <= 0`、`height <= 1` の入力を Rust 側で明示的に `Err` にする（C の clamp 後に radius <= 0、または height <= 1 の `-1` 返却条件と一致。width == 0 / height == 0 のゼロサイズ入力も同様に `Err` になる）。設計方針の「新規チェックは追加せず現行どおり C 経由の Err を維持」からは実装を変更し、負の有効 radius を usize にキャストする際のデバッグビルドのオーバーフローパニックを防ぐ目的を兼ねて Rust 側チェックを採用した
2. 有効 radius を C と同じ規則（`min(radius, height)`、`min(radius, width / 2 - 1)`）で計算し、cumsum の必要行数（`min(height, 有効 radius * 2 + 2)` 行）を `checked_buf_size` で検証する。検証式が C の最大書き込み行数と一致する根拠（循環バッファのラップ境界 `max_cumsum_bot_row`）は実装コメントに明記した
3. docstring に cumsum の必要行数（`min(height, 有効 radius * 2 + 2)` 行）、`stride32_cumsum >= width * 4` の要件、radius の clamp 規則、Err 条件を明記した
4. `tests/test_planar.rs` に境界値テスト 12 本を追加した（必要行数ちょうど / 1 行不足 / 等号境界 / height == radius / radius > height / width clamp / 最小 height / 空バッファ / radius <= 0 / height <= 1 / width <= 3 / stride 不足）
5. `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加した
