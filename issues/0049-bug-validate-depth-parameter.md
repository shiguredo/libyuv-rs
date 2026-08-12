# planar の 16bit 変換関数群で depth パラメータが未検証のまま libyuv に渡される

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-validate-depth-parameter
- Polished: 2026-08-12
- Reporter: @voluntas

## 目的

`src/planar.rs` の 16bit 変換関数群が `depth` パラメータを検証せずに libyuv へ渡している。libyuv 側は `1 << depth` などのシフト演算を行うため、範囲外の `depth` で未定義動作（UB）を誘発する。Rust 側で関数ごとの有効範囲に基づく `depth` 検証を追加する。

## 優先度根拠

High。

- 範囲外の `depth`（例: `1 << depth` は 31 以上で int 表現不能の UB、`depth - 10` は 10 未満で負シフト UB）で libyuv のシフトが C 標準上 UB になる。`ConvertToMSBPlane_16` / `ConvertToLSBPlane_16` はシフト式（`int scale = 1 << ...`）が早期 return より先に評価されるため、ゼロサイズ入力でも範囲外 `depth` なら UB を誘発する
- libyuv 側の `assert`（関数ごとに `depth >= 1` / `depth >= 8` / `depth >= 10` など）は C 版 row 関数（`MergeAR64Row_C` / `MergeXR30Row_C` / `MergeARGB16To8Row_C` / `MergeUVRow_16_C` / `SplitUVRow_16_C`）と `MergeUVPlane_16` 本体に存在するが、他の plane 関数と SIMD 版 row 関数には存在しない。さらに Release（NDEBUG）ビルドでは assert 自体がコンパイル時に除去され、`ConvertToMSBPlane_16` / `ConvertToLSBPlane_16` は assert を一切持たない

## 現状

以下の関数（`src/planar.rs`）が `depth: i32` を無検証でネイティブに渡している:

- `merge_uv_plane_16`
- `split_uv_plane_16`
- `merge_ar64_plane`
- `merge_xr30_plane`
- `merge_argb16_to_8_plane`
- `convert_to_lsb_plane_16`
- `convert_to_msb_plane_16`

libyuv の実装（commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）の関数ごとの有効範囲とシフト演算は以下のとおり:

- `MergeUVPlane_16`（`planar_functions.cc` 本体に assert）/ `SplitUVPlane_16`（本体に assert なし）: 有効範囲 8..=16。`int shift = 16 - depth;` は row 関数 `MergeUVRow_16_C` / `SplitUVRow_16_C`（`row_common.cc`。assert もここに存在）
- `MergeAR64Row_C`（`row_common.cc`）: `assert(depth >= 1); assert(depth <= 16);`、`int shift = 16 - depth; int max = (1 << depth) - 1;` → 有効範囲 1..=16
- `MergeXR30Row_C`（`row_common.cc`）: `assert(depth >= 10); assert(depth <= 16);`、`int shift = depth - 10;` → 有効範囲 10..=16
- `MergeARGB16To8Row_C`（`row_common.cc`）: `assert(depth >= 8); assert(depth <= 16);`、`int shift = depth - 8;` → 有効範囲 8..=16
- `ConvertToMSBPlane_16`（`planar_functions.cc`）: assert なし、`int scale = 1 << (16 - depth);`（depth > 16 で負シフト、depth <= -15 で int 溢れ UB）
- `ConvertToLSBPlane_16`（`planar_functions.cc`）: assert なし、`int scale = 1 << depth;`（depth >= 31 で int 表現不能 UB、負値も UB）

C 側の assert は `build.rs` の CMake Release プロファイル（NDEBUG）によりコンパイル時に除去される。

## 設計方針

関数ごとの有効範囲（C 実装の assert と一致する範囲）で `depth` を検証し、範囲外は `Err` を返す。エラーメッセージは `Error::with_reason(-1, function, ...)` 形式で、`reason` が `&'static str` のため関数ごとの静的文字列（例: `"depth must be between 8 and 16"`）に展開する。docstring に関数ごとの有効範囲を明記する。

- `merge_uv_plane_16` / `split_uv_plane_16` / `merge_argb16_to_8_plane`: 8..=16
- `merge_ar64_plane`: 1..=16（C 側契約（assert）と一致。8..=16 に制限すると C で動作する depth 1..=7 を拒否する機能退行になるため、C 契約に合わせる）
- `merge_xr30_plane`: 10..=16（C 側契約（assert）と一致。8..=16 だと depth=8, 9 の負シフト UB が残るため必須）
- `convert_to_lsb_plane_16` / `convert_to_msb_plane_16`: 8..=16（C 側に assert がなく契約が明示されていないため、crate の 16bit 変換関数の下限 8 に合わせた crate 仕様として採用。シフト単体では MSB が -14..=16、LSB が 0..=30 で安全だが、depth 8 未満は 16bit 変換の実用範囲外として許可しない。なお 8..=16 内でも row 関数の乗算（LSB depth=16 のとき scale=65536 となり、`src_y[x] * scale` が int 範囲を超えうる）は libyuv 側の現行実装として C 標準上 UB のまま残るが、本 issue のスコープ外とする）

検証は各関数の先頭で、既存の `require_c_int` 検証と同じ並びに追加する（エラーの `function` 引数は各 C 関数名、例: `"MergeXR30Plane"`）。`depth` の型は `i32` のまま実行時検証とする（C ABI が `c_int` のため）。ゼロサイズ入力（width == 0 / height == 0）は C 実装の早期 return により現状 no-op（`Ok`）だが（convert 系は範囲外 depth ならシフト UB が先に起きうる）、ゼロサイズ時は他の検証（`require_c_int` / stride / バッファ）が必ず通過するため、depth 検証を既存検証と同じ並びに置けばゼロサイズ + 範囲外 depth は `Err` になる。この挙動（ゼロサイズでも範囲外 depth は `Err`）を本 issue の仕様として確定する（ゼロサイズ入力自体を Err に統一するのは 0064 のスコープ）。

## 完了条件

- 各関数が有効範囲外の `depth` で `Err` を返し、有効範囲内（境界値を含む）の `depth` で `Ok` を返すこと:
  - `merge_uv_plane_16` / `split_uv_plane_16` / `merge_argb16_to_8_plane`: 8..=16 以外で `Err`、8 と 16 で `Ok`（境界値: 7 / 8 / 16 / 17、負値: -1 / `i32::MIN`、最大値: `i32::MAX`）
  - `merge_ar64_plane`: 1..=16 以外で `Err`、1 と 16 で `Ok`（境界値: 0 / 1 / 16 / 17、負値: -1 / `i32::MIN`、最大値: `i32::MAX`）
  - `merge_xr30_plane`: 10..=16 以外で `Err`、10 と 16 で `Ok`（境界値: 9 / 10 / 16 / 17、負値: -1 / `i32::MIN`、最大値: `i32::MAX`）
  - `convert_to_lsb_plane_16` / `convert_to_msb_plane_16`: 8..=16 以外で `Err`、8 と 16 で `Ok`（境界値: 7 / 8 / 16 / 17、負値: -1 / `i32::MIN`、最大値: `i32::MAX`。現状セクションで UB と明示した代表点（LSB は 31、MSB は -15）も含む）
- ゼロサイズ入力（width == 0 / height == 0）+ 範囲外 `depth` で `Err` になること（設計方針で仕様確定した挙動。ゼロサイズ + 有効範囲内 `depth` は現行どおり no-op（`Ok`）のままで、そのテストはゼロサイズ挙動の変更とあわせて 0064 のスコープとする。ゼロサイズ入力自体の扱いも 0064 のスコープ）
- 7 関数の docstring に `depth` の有効範囲（関数ごと）が明記されていること
- `tests/test_planar.rs` に境界値テスト（関数ごとの有効範囲の Err / Ok 境界）が追加されていること（0053 でテストファイルが分割された場合は `tests/test_planar/` 配下。正常系・その他のエラーパスの網羅は 0053 のスコープ）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. 7 関数に `depth` の範囲検証（関数ごとの有効範囲: 8..=16 / 1..=16 / 10..=16）を追加する（ゼロサイズ + 範囲外 `depth` も `Err` になる先頭配置）
2. docstring に関数ごとの有効範囲を明記する
3. `tests/test_planar.rs` に境界値テスト（関数ごとの範囲境界、ゼロサイズ + 範囲外 `depth` を含む）を追加する
4. `CHANGES.md` に `[FIX]` エントリを追加する
