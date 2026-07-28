# detile_split_uv_plane が Result を返さず入力検証も欠落している

- Priority: Medium
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/fix-detile-split-uv-plane-return
- Polished: 2026-07-29
- Reporter: @voluntas

## 目的

`src/convert.rs` の `detile_split_uv_plane` 関数だけが戻り値型 `()` で、バッファサイズの入力検証も一切持たない問題を修正する。

## 優先度根拠

Medium。呼び出し元がエラーを検出できず、`size.width` / `size.height` / stride が `c_int` 範囲を超える場合やバッファ不足時に検証なく libyuv を呼び出し、未定義動作のリスクがある。ただし grep の結果、現状 `detile_split_uv_plane` の呼び出し側はコードベース内に存在しないため、破壊的変更の影響はない。

## 現状

`src/convert.rs` の `detile_split_uv_plane` は以下の問題を持つ:

1. **戻り値型が `()`**: 同一セクションの `detile_plane`、`detile_plane_16`、`detile_to_yuy2` はすべて `Result<(), Error>` を返す。`src/convert.rs` の公開関数（約 240 関数）の中でこの関数だけが例外
2. **入力検証が皆無**: `require_c_int`（c_int 範囲チェック）、`checked_buf_size`（オーバーフロー安全なバッファサイズ計算）、stride >= width チェックのいずれもない
3. **`as c_int` キャストが未検証**: `size.width` / `size.height` / 各 stride / `tile_height` が c_int 範囲外でもそのまま FFI に渡される

なお、`detile_plane` と `detile_plane_16` も同様に `require_c_int` / `checked_buf_size` を使わない手動の簡易バッファチェックのみであり、プロジェクト標準の検証パターンに準拠していない。これらは本 issue のスコープ外とし、別途対応する（未起票）。

## 設計方針

検証パターンの参照先として `src/planar.rs` の `split_uv_plane()` を使用する。この関数は:

- `require_c_int` で width, height, src_stride_uv, dst_stride_u, dst_stride_v を検証
- `checked_buf_size` で 3 プレーン分のバッファサイズをオーバーフロー安全に計算
- 戻り値 `Result<(), Error>`、FFI は void なので `Ok(())`

`DetileSplitUVPlane` FFI は void を返す（bindgen 出力で確認済み）。そのため `Error::check` は不要で、検証通過後に `Ok(())` を返す。

**`size: ImageSize` のセマンティクス（重要）**: `width` はフル画像幅（= インターリーブ UV 行のバイト数。NV12 では 1 ピクセルあたり U+V で 2 バイトだが、インターリーブ行全体では画像幅と等しいバイト数になる）。`height` は UV プレーンの行数（4:2:0 なら画像高さ / 2）。`split_uv_plane` の `width`（UV ペア数 = 画像幅 / 2）とはセマンティクスが異なる。

libyuv の C 実装 (`planar_functions.cc`) と単体テスト (`planar_test.cc`) で確認:
- C 実装: `DetileSplitUVRow_C` は 1 行あたり `width` バイトの src を消費し、U を `(width+1)/2` バイト、V を `(width+1)/2` バイト出力する
- 単体テスト: `width` にフル画像幅 `benchmark_width_` を渡し、`dst_stride_u` に `(benchmark_width_ + 1) / 2` を渡している

正しい stride 関係:
- `src_stride_uv >= round_up(size.width, 16)`（タイル配置は 16 バイト単位で処理するため、stride は 16 の倍数に丸めた幅以上必要。libyuv テストでは `(benchmark_width_ + 15) & ~15` を使用）
- `dst_stride_u >= (size.width + 1) / 2`（U プレーン行は画像幅の半分。dst はリニア出力）
- `dst_stride_v >= (size.width + 1) / 2`（V プレーン行は画像幅の半分。dst はリニア出力）

**src_uv のバッファサイズ（タイル配置）**: src はリニアではなくタイル配置バッファである。C 実装のポインタ進行 (`planar_functions.cc:1273-1281`) はタイルグループ間で `src_stride_uv * tile_height` ずつ飛ぶため、安全な最小バッファは `src_stride_uv * ceil(height / tile_height) * tile_height` である。`height % tile_height != 0` のとき、リニア相当サイズ (`src_stride_uv * height`) ちょうどのバッファでは OOB 読み取りが発生する（例: width=32, height=17, tile_height=16, src_stride_uv=32 → リニア相当 544 バイトだが実際は 1024 バイト必要）。dst_u / dst_v はリニア出力なので `checked_buf_size(dst_stride_*, size.height)` で正しい。

**`tile_height` の制約**: libyuv 内部で `(y & (tile_height - 1))` のビットマスクを使用するため、`tile_height` は 2 の累乗でなければならない。`tile_height == 0` は debug ビルドの `assert(tile_height > 0)` で abort する。兄弟関数 `DetilePlane` は `IS_POWEROFTWO(tile_height)` を検査している。

## 完了条件

- `detile_split_uv_plane` が `Result<(), Error>` を返すこと
- 以下の入力検証が追加されていること:
  - `require_c_int` による width, height, src_stride_uv, dst_stride_u, dst_stride_v, tile_height の c_int 範囲チェック
  - `tile_height` が 2 の累乗でない場合のチェック（`is_power_of_two()`。0 も false を返すため別途チェック不要）
  - src_stride_uv >= round_up(size.width, 16)（タイル配置は 16 バイト単位処理）
  - dst_stride_u >= (size.width + 1) / 2、dst_stride_v >= (size.width + 1) / 2（U/V プレーン行は画像幅の半分）
  - src_uv のバッファサイズ検証: `src_stride_uv * ceil(height / tile_height) * tile_height`（タイル配置のパディング分を含む。オーバーフロー安全に計算すること）
  - dst_u, dst_v のバッファサイズ検証: `checked_buf_size(dst_stride_*, size.height, ...)`（リニア出力）
- `tests/test_convert.rs` を新規作成し、以下のテストを追加すること:
  - 正常系: 適切なサイズのバッファで `Ok(())` が返ること
  - 異常系: src_uv / dst_u / dst_v の各バッファが不足するケースで `Err` が返ること
  - 異常系: 各 stride が最小幅未満のケースで `Err` が返ること
  - 異常系: tile_height が 0 または非 2 累乗（例: 3）のケースで `Err` が返ること
  - 境界値: width=0 / height=0 のケースで `Ok(())` が返ること（libyuv は `width <= 0 || height == 0` で早期 return するため no-op）
- `CHANGES.md` の `## develop` セクションに `[CHANGE] detile_split_uv_plane の戻り値型を Result<(), Error> に変更し入力検証を追加する` を追記すること（@voluntas 署名付き）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること

## 解決方法

1. `detile_split_uv_plane` のシグネチャを `pub fn detile_split_uv_plane(...) -> Result<(), Error>` に変更する
2. `src/convert.rs` の import に `checked_buf_size` を追加する（`require_c_int` は既に import 済み）
3. `require_c_int` で以下を c_int 範囲チェックする:
   - `size.width`, `size.height`
   - `src_stride_uv`, `dst_stride_u`, `dst_stride_v`
   - `tile_height`
   - function 名文字列は `"DetileSplitUVPlane"` を使用する
4. `size.width == 0 || size.height == 0` の場合は `Ok(())` を早期 return する（FFI を呼ばない。C 実装の `if (width <= 0 || height == 0) return;` と同一セマンティクス。これにより `src_stride_uv == 0` で C の assert が発火する経路を塞ぐ）
5. `tile_height` のチェック: `!tile_height.is_power_of_two()` で `Err` を返す（`usize::is_power_of_two()` は 0 で false を返すため別途 0 チェック不要。`Error::with_reason(-1, "DetileSplitUVPlane", "tile_height must be a power of two")`）
6. stride チェック:
   - `src_stride_uv < size.width.div_ceil(16) * 16` で `Err`（タイル配置は 16 バイト単位処理。libyuv テストの `(width + 15) & ~15` と同等）
   - `dst_stride_u < (size.width + 1) / 2` で `Err`
   - `dst_stride_v < (size.width + 1) / 2` で `Err`
7. バッファサイズ検証:
   - src_uv（タイル配置）: タイルグループ数 `size.height.div_ceil(tile_height)` を計算し、`src_stride_uv * tile_groups * tile_height` をオーバーフロー安全に計算して `src_uv.len()` と比較する（`checked_mul` を使用）
   - dst_u（リニア出力）: `checked_buf_size(dst_stride_u, size.height, ...)`
   - dst_v（リニア出力）: `checked_buf_size(dst_stride_v, size.height, ...)`
8. FFI は void を返すため、検証通過後に `Ok(())` を返す
9. `tests/test_convert.rs` を新規作成し、正常系・異常系のテストを追加する（既存の `tests/test_mjpeg.rs` の import 規約 `use shiguredo_libyuv::{...}` に準拠する）
10. `CHANGES.md` に `[CHANGE]` エントリを追加する

## 注記

- issue 0036（convert.rs 分割）が先に実施された場合、`src/convert.rs` のファイルパスが変動する。その場合は 0036 の完了条件に従いパス参照を更新すること
