# detile_split_uv_plane が Result を返さず入力検証も欠落している

- Priority: Medium
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/fix-detile-split-uv-plane-return
- Polished: 2026-07-08
- Reporter: @voluntas

## 目的

`src/convert.rs` の `detile_split_uv_plane` 関数だけが戻り値型 `()` で、バッファサイズの入力検証も一切持たない問題を修正する。同一ファイル内の `convert.rs` のほぼ全公開関数（約 200 関数）が `Result<(), Error>` を返している中、この関数だけが例外となっている。

## 優先度根拠

Medium。呼び出し元がエラーを検出できず、`size.width` / `size.height` / stride が `c_int` 範囲を超える場合やバッファ不足時に検証なく libyuv を呼び出し、未定義動作のリスクがある。ただし grep の結果、現状 `detile_split_uv_plane` の呼び出し側はコードベース内に存在しないため、破壊的変更の影響はない。

## 現状

`src/convert.rs:4068-4091` の `detile_split_uv_plane` は以下の問題を持つ:

1. **戻り値型が `()`**: 同一セクションの `detile_plane` (L3990)、`detile_plane_16` (L4029)、`detile_to_yuy2` (L4094) はすべて `Result<(), Error>` を返す
2. **入力検証が皆無**: `require_c_int`（c_int 範囲チェック）、`checked_buf_size`（オーバーフロー安全なバッファサイズ計算）、stride >= width チェックのいずれもない
3. **`as c_int` キャストが未検証**: `size.width` / `size.height` / 各 stride / `tile_height` が c_int 範囲外でもそのまま FFI に渡される

なお、`detile_plane` と `detile_plane_16` も同様に `require_c_int` / `checked_buf_size` を使わない手動の簡易バッファチェックのみであり、プロジェクト標準の検証パターンに準拠していない。これらは本 issue のスコープ外とし、別途対応する。

## 設計方針

検証パターンの参照先として `src/planar.rs` の `split_uv_plane()` (L182-260) を使用する。この関数は:

- `require_c_int` で width, height, src_stride_uv, dst_stride_u, dst_stride_v を検証
- `checked_buf_size` で 3 プレーン分のバッファサイズをオーバーフロー安全に計算
- インターリーブ UV の src_stride_uv には `>= width * 2` チェック
- 戻り値 `Result<(), Error>`、FFI は void なので `Ok(())`

`DetileSplitUVPlane` FFI の戻り値型については `build.rs` の bindgen 出力 (`src/sys.rs` 経由で参照される `bindings.rs`) で実際のシグネチャを確認し、int を返す場合は `Error::check` を追加する。void の場合は `Ok(())` とする。

## 完了条件

- `detile_split_uv_plane` が `Result<(), Error>` を返すこと
- 以下の入力検証が追加されていること:
  - `require_c_int` による width, height, src_stride_uv, dst_stride_u, dst_stride_v, tile_height の c_int 範囲チェック
  - `checked_buf_size` による src_uv, dst_u, dst_v 全プレーンのオーバーフロー安全なバッファサイズ検証
  - src_stride_uv >= size.width * 2（インターリーブ UV のため）
  - dst_stride_u >= size.width、dst_stride_v >= size.width
- `tests/test_convert.rs` に以下のテストを追加すること:
  - 正常系: 適切なサイズのバッファで `Ok(())` が返ること
  - 異常系: src_uv / dst_u / dst_v の各バッファが不足するケースで `Err` が返ること
  - 異常系: 各 stride が最小幅未満のケースで `Err` が返ること
  - 境界値: width=0 / height=0 のケース（libyuv の挙動に依存）
- `CHANGES.md` の `## develop` セクションに `[CHANGE] detile_split_uv_plane の戻り値型を Result<(), Error> に変更し入力検証を追加する` を追記すること（@voluntas 署名付き）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --all` が成功すること

## 解決方法

1. `detile_split_uv_plane` のシグネチャを `pub fn detile_split_uv_plane(...) -> Result<(), Error>` に変更する
2. `src/convert.rs` の import に `checked_buf_size` を追加する（`require_c_int` は既に import 済み）
3. `require_c_int` で以下を c_int 範囲チェックする:
   - `size.width`, `size.height`
   - `src_stride_uv`, `dst_stride_u`, `dst_stride_v`
   - `tile_height`
   - function 名文字列は `"DetileSplitUVPlane"` を使用する
4. `checked_buf_size` で src_uv, dst_u, dst_v の 3 プレーンのバッファサイズを計算する。各プレーンの height には `size.height` を使用する（呼び出し側が適切なプレーン高さを渡す前提）
5. stride チェック: `src_stride_uv < size.width * 2`、`dst_stride_u < size.width`、`dst_stride_v < size.width` のいずれかで `Err` を返す
6. `bindings.rs`（bindgen 出力）で `DetileSplitUVPlane` の実際の戻り値型を確認し、int なら `Error::check(result, "DetileSplitUVPlane")` を使用、void なら `Ok(())` を返す
7. `tests/test_convert.rs` に正常系・異常系のテストを追加する
8. `CHANGES.md` に `[CHANGE]` エントリを追加する
