# planar.rs の unsafe 呼び出し前の入力検証が不十分

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/change-planar-unsafe-validation-gap
- Polished: 2026-06-27
- Reporter:

## 目的

`planar.rs` の `interpolate_plane_16` と `mirror_plane` において、libyuv への unsafe 呼び出し前の入力検証が不十分な箇所を修正し、メモリ安全性を確保する。

同時に、同ファイルの他の plane 関数と同じく `Result<(), Error>` を返す形に API を統一し、利用者に対して一貫したエラー処理を提供する。

## 優先度根拠

High とする。

- `size.width` / `size.height` / stride を `c_int` に縮小変換する際に範囲外の値を検証していない
- `stride * size.height` の計算で `usize` のオーバーフローが発生する可能性がある
- バッファサイズを十分に検証せずに libyuv を呼び出すため、範囲外メモリアクセスのリスクがある
- `interpolate_plane_16` は libyuv からの戻り値を無視しており、内部的なエラーを検出できない
- `mirror_plane` は `Result` を返さないため、入力検証に失敗してもエラーを伝播できない

## 現状

### `interpolate_plane_16`

`src/planar.rs` において、以下の検証が欠落している。

- `size.width` / `size.height` / 各 stride の `c_int` 範囲チェック
- `stride >= width` の最小幅チェック
- `stride * size.height` の `checked_mul` によるオーバーフロー防止
- libyuv 戻り値の `Error::check`

また、8bit 版の `interpolate_plane` も入力検証は実装済みだが、libyuv 戻り値を `Error::check` しておらず、同様の問題を抱えている。

### `mirror_plane`

`src/planar.rs` において、以下の問題がある。

- `Result` を返さない
- `size.width` / `size.height` / 各 stride の `c_int` 範囲チェックなし
- `stride >= width` チェックなし
- バッファサイズチェックなし

同ファイルの `i400_mirror` / `i420_mirror` / `nv12_mirror` / `argb_mirror` / `rgb24_mirror` は `Result<(), Error>` を返し、`Image` 型の `validate` による検証を行っているため、`mirror_plane` だけが API 間で不整合になっている。

### 呼び出し側への影響

`pbt/tests/prop_planar.rs` の `mirror_plane_twice` と `fuzz/fuzz_targets/fuzz_planar.rs` の `mirror_plane` 呼び出しは、現状は戻り値を無視している。`mirror_plane` を `Result<(), Error>` に変更した場合、これらの呼び出し側も合わせて修正する必要がある。

## 設計方針

- `interpolate_plane_16` には、他の plane 関数と同じく `require_c_int`、`checked_buf_size`、バッファサイズチェックを適用する
- `interpolate_plane_16` の libyuv 戻り値は `Error::check` する
- 8bit 版 `interpolate_plane` の libyuv 戻り値も合わせて `Error::check` する
- `mirror_plane` を `Result<(), Error>` を返すように変更し、以下の検証を追加する
  - `size.width` / `size.height` / 各 stride の `c_int` 範囲チェック
  - `stride >= width` チェック
  - ソース・ディスティネーション双方のバッファサイズチェック（`checked_buf_size` を使用）
- `mirror_plane` がラップする `MirrorPlane` は libyuv 側で void 関数だが、入力検証の結果を呼び出し側に伝播するため `Result<(), Error>` を返す
- 本 issue は入力検証不足という bug が根幹だが、`mirror_plane` の戻り値型変更により後方互換が失われるため、ブランチ名は `feature/change-` とし、`CHANGES.md` に `[CHANGE]` エントリを追加する

## 完了条件

- `interpolate_plane_16` と `mirror_plane` が不正入力に対して `Error` を返すこと
- 8bit 版 `interpolate_plane` の libyuv 戻り値を `Error::check` すること
- `pbt/tests/prop_planar.rs` の `mirror_plane_twice` で `mirror_plane` の戻り値を処理すること
- `fuzz/fuzz_targets/fuzz_planar.rs` の `mirror_plane` 呼び出しを `let _ = mirror_plane(...)` に変更すること
- `tests/test_planar.rs` を新設し、`interpolate_plane_16` と `mirror_plane` の境界値・エラーパステストを追加すること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリを追加すること

## 解決方法

1. `interpolate_plane_16` に 8bit 版 `interpolate_plane` と同じ検証パターンを適用する
2. `interpolate_plane_16` の libyuv 戻り値を `Error::check` する
3. 8bit 版 `interpolate_plane` の libyuv 戻り値も `Error::check` する
4. `mirror_plane` を `Result<(), Error>` を返すように変更し、検証を追加する
5. `pbt/tests/prop_planar.rs` と `fuzz/fuzz_targets/fuzz_planar.rs` の `mirror_plane` 呼び出しを修正する
6. `tests/test_planar.rs` を新設し、以下の境界値テストを追加する
   - `width` / `height` / stride が `c_int` 範囲を超えるケース
   - `stride < width` のケース
   - ソース・ディスティネーションのバッファサイズが不足するケース
   - 正常系の動作確認（`interpolation` が `0` / `255` / `128`）
   - `width = 0` / `height = 0` のケース（libyuv 側の挙動と合わせて検証）
7. `CHANGES.md` に `[CHANGE]` エントリを追加する
