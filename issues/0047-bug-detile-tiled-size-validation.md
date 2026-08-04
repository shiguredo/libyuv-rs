# detile_plane / detile_plane_16 / detile_to_yuy2 の検証が線形サイズのままで領域外読み出しが発生する

- Priority: High
- Created: 2026-08-04
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Flash
- Branch: feature/fix-detile-tiled-size-validation
- Polished: 2026-08-04
- Reporter: @voluntas

## 目的

`src/convert/hardware.rs` の `detile_plane` / `detile_plane_16` / `detile_to_yuy2` のソースバッファ検証が線形サイズ（`stride * height`）のままで、libyuv がタイル配置で読み出す実アクセス範囲を反映していない。幅が 16 の倍数でない場合や高さがタイル高の倍数でない場合に検証済みサイズを超える読み出し（OOB）が発生する。`detile_split_uv_plane` と同じタイル配置のサイズ計算（およびその前提となる stride 下限チェック）に統一する。

## 優先度根拠

High。

- 検証を通過する入力で領域外読み出し（UB）が発生する
- 同一ファイルの `detile_split_uv_plane` は正しいタイル公式（round16 stride チェック + `stride * ceil(height / tile_height) * tile_height`）を使う一方で、3 関数だけが線形サイズのままと不整合

## 現状

libyuv の `DetilePlane`（`planar_functions.cc`、commit `d23308a2a7442be8e559b1b471862fd7588d6a57` 時点）は行ごとに src を 16 バイト進め、`tile_height` 行ごとに `src_y - src_tile_stride + src_stride_y * tile_height`（`src_tile_stride = 16 * tile_height`）で次のタイル行へジャンプする。1 行の読み出しはタイル列ごとに `src_tile_stride` 間隔のチャンク列であり、1 タイル行の読み出しは幅を 16 の倍数に切り上げた値（padded width）分のタイル配置スパンに及ぶ。実アクセスの最大オフセットの上界は `(div_ceil(height, tile_height) - 1) * stride * tile_height + (tile_height - 1) * 16 + (div_ceil(width, 16) - 1) * 16 * tile_height + 端数` となり、線形サイズ（`stride * height`）より大きくなりうる（この式は最終タイル段がフルタイル高を持つ仮定の上界であり、実際の最大読み出しはこれ以下。下記の例では 784 バイト）。

対象と問題:

- `detile_plane`（`src/convert/hardware.rs`）: ソース検証が `checked_buf_size(src_stride, size.height)` の線形サイズ
- `detile_plane_16`: 同上（u16 要素数ベースの線形サイズ。タイル公式も u16 要素数単位で適用する）
- `detile_to_yuy2`: `Nv12Image::validate`（線形）のみ。さらに `tile_height` の 2 累乗検証・`require_c_int(tile_height)` が欠落しており、libyuv の `DetileToYUY2` は非 2 累乗を検査せず `y & (tile_height - 1)` でマスクするため非 2 累乗の `tile_height` で壊れた挙動になる（`DetilePlane` / `DetilePlane_16` の C 実装は 2 累乗を検査するが `DetileToYUY2` は検査しない）

例: width=32、height=17、tile_height=16、stride=32 で実読み出しの必要サイズは 784 バイトなのに対し、線形検証は 544 バイトしか要求しない。

## 設計方針

`detile_split_uv_plane` の式（round16 stride チェック + `src_stride * div_ceil(height, tile_height) * tile_height`）に統一する。`detile_split_uv_plane` の式が安全なのは `src_stride >= round_up(width, 16)` チェック（`src_min_stride = size.width.div_ceil(16) * 16`、hardware.rs の `detile_split_uv_plane`）を併設しているためであり、3 関数にも同じ stride 下限チェックを追加する。この式は全タイル段を一括で要求する安全側の過大要求であり、0046 の MM21 / MT2T 式（最終タイル段のみ幅丸めベースで計算）より過大になるが、同一ファイル内の既存実装（`detile_split_uv_plane`）との統一を優先する。サイズ計算は `detile_split_uv_plane` と同じく `checked_mul` によるオーバーフロー安全な演算とする。

- `detile_plane` / `detile_plane_16`: `src_stride >= width.div_ceil(16) * 16` チェックを追加し、ソースサイズを `src_stride * div_ceil(height, tile_height) * tile_height` で検証する（`detile_plane_16` は u16 要素数単位で同様）
- `detile_to_yuy2`: `tile_height` の 2 累乗検証と `require_c_int` を追加し、ソース（Y / UV）もタイル配置サイズで検証する。検証は関数内で `src.y` / `src.uv` スライス長を個別に検証する（`Nv12Image::validate` は NV12 全関数共通のため変更しない）。`tile_height` は 2 以上かつ 2 累乗であること（`tile_height >= 2`。tile_height=1 は 2 累乗だが UV タイル高 `tile_height / 2` が 0 になり検証をすり抜けるため不許可）
  - Y の必要サイズ: `y_stride * div_ceil(height, tile_height) * tile_height`（`y_stride >= width.div_ceil(16) * 16` を併設）
  - UV の必要サイズ: `uv_stride * div_ceil(height, tile_height) * (tile_height / 2)`（`DetileToYUY2` の UV タイル高は `tile_height / 2`。`uv_stride >= width.div_ceil(16) * 16` を併設。Y 式をそのまま適用すると 2 倍過大要求になるため注意）
- 検証式の根拠（libyuv の行送り規則）をコメントで明記する
- ゼロサイズ入力（width == 0 / height == 0）は現行の no-op（`Ok` を返す）を維持する（0064 で例外として docstring 明記する方針と整合。no-op のチェックは 2 累乗検証より先に置き、tile_height=0 で `div_ceil` が panic しないようにする）

## 完了条件

- 幅が 16 の倍数でない入力、または高さが `tile_height` の倍数でない入力で、必要サイズを下回るソースバッファが `Err` になること
- 必要サイズちょうどのバッファで `Ok` になり、1 バイト不足（`detile_plane_16` は u16 要素数単位のため 1 要素不足）で `Err` になる境界が成立すること
- `detile_to_yuy2` が非 2 累乗の `tile_height` で `Err` を返すこと
- `detile_to_yuy2` が `c_int` 範囲を超える `tile_height` で `Err` を返すこと
- `detile_to_yuy2` が `tile_height == 1` で `Err` を返すこと（UV タイル高 0 による検証すり抜けの防止）
- `detile_to_yuy2` の docstring に `tile_height` の制約（2 以上かつ 2 累乗）とソースがタイル配置であることが明記されていること
- ゼロサイズ入力（width == 0 / height == 0）で `Ok`（no-op）を維持すること
- `tests/test_convert.rs` に 3 関数の境界値テスト（幅 16 倍数・非倍数 × タイル高倍数・非倍数）が追加されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --workspace --features source-build -- -D warnings` が成功すること
- `cargo test --workspace --features source-build` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリを追加すること

## 解決方法

1. `detile_plane` / `detile_plane_16` に round16 stride チェックを追加し、ソースサイズ検証をタイル配置（`div_ceil`）ベースに変更する
2. `detile_to_yuy2` に `tile_height` の 2 累乗検証（2 以上に限定。`tile_height == 1` は不許可）と `require_c_int` を追加し、Y / UV それぞれのソースサイズ検証をタイル配置ベース（UV は `tile_height / 2`）に変更する
3. 検証式の根拠コメントを追加する
4. `tests/test_convert.rs` にテストを追加する
5. `CHANGES.md` に `[FIX]` エントリを追加する
