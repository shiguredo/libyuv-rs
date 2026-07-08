# convert.rs をサブモジュールに分割する

- Priority: Medium
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/refactor-split-convert-rs
- Polished: 2026-07-08
- Reporter: @voluntas

## 目的

`src/convert.rs` が 6994 行、60 セクション、240 以上の変換関数を単一ファイルに抱えており保守性が著しく低下している。自然な分割境界が明瞭なため `src/convert/` ディレクトリに分割する。

## 優先度根拠

Medium。現時点で機能上の問題はないが、MJPEG 追加でさらに肥大化しており保守困難。また open issue 0030、0032、0037 がいずれも `src/convert.rs` を編集対象としており、これらより先に本 issue を実施することで競合を回避できる。

## 現状

`src/convert.rs` 単一ファイル (6994 行、60 セクション) に以下の変換カテゴリが混在している。全公開関数は互いに独立して libyuv FFI を直接呼び出しており、Rust レベルでの相互依存はない。プライベート関数 (`validate_alpha_src`, `validate_alpha_dst`, `validate_mjpeg_input`) も使用元セクション内でのみ呼ばれており、クロスセクション依存は存在しない。

## 設計方針

### モジュール分割 (10 モジュール)

| モジュール | 内容 | 概算行数 |
|---|---|---|
| `i420.rs` | I420↔ARGB/ABGR/RGB24/NV12/NV21/I422/I444/I400、I420→10bit/AR30 等、I420 コピー | ~700 |
| `nv.rs` | NV12/NV21↔ARGB/ABGR/RGB24/I420、I444/I422→NV12/NV21、NV コピー、abgr→NV、NV→raw/rgb565/yuv24/nv24 | ~800 |
| `argb.rs` | ARGB/ABGR/RGBA/BGRA 相互変換、AR30/AB30/AR64/AB64 変換、RAW 変換、ARGB→YUV、ARGB コピー | ~1100 |
| `subsampling.rs` | I422/I444 相互変換、I422/I444↔RGB24、I422/I444 コピー | ~350 |
| `high_bitdepth.rs` | I010/I210/I410 変換 (10bit)、I012/I212/I412 変換 (12bit)、I420/I422→10bit | ~1200 |
| `colorspace.rs` | H 系 (BT.709 8bit/10bit)、U 系 (BT.2020 8bit/10bit) | ~900 |
| `packed.rs` | P010/P210/P410、RGB565/ARGB1555/ARGB4444、YUY2/UYVY | ~1100 |
| `jpeg.rs` | J400/J420/J422/J444 全変換 (J 系 JPEG 色空間) | ~700 |
| `hardware.rs` | Android420、MM21、MT2T、AYUV、Detile 全関数 | ~550 |
| `mjpeg.rs` | MJPEG デコード (mjpeg_size, mjpeg_to_*) | ~170 |

`alpha.rs` の内容 (アルファチャンネル変換) と `grayscale.rs` (I400/J400 変換) は `i420.rs` に統合する。`detile.rs` (4 関数) は `hardware.rs` に統合する。

### 関数のモジュール割り当てルール

分割の判断に迷うクロスカテゴリ関数は以下のルールで割り当てる:

1. **出力先フォーマットのモジュールに配置する** (例: `i420_to_nv12` → 出力が NV12 なので `nv.rs`)
2. 例外: 「I420 <-> X」「I420 -> X」と明示されたセクション内の関数は `i420.rs` に配置する
3. プライベートヘルパー (`validate_alpha_*`, `validate_mjpeg_input`) は使用元モジュールにそのまま残す

### import と属性の扱い

1. 各サブモジュールは自身が必要な `use crate::{...}` と `use std::ffi::c_int` のみを記載する
2. `src/convert/mod.rs` には `#![expect(clippy::too_many_arguments)]` を **付けない**。inner attribute は子モジュールに伝播しないため、各サブモジュールファイルの先頭に `#![expect(clippy::too_many_arguments)]` を個別に付与する (issue 0032 の作業を先取り)
3. `src/convert/mod.rs` で各サブモジュールを `pub mod xxx; pub use xxx::*;` で再エクスポートする

### 公開 API 互換性

`src/lib.rs` の `mod convert; pub use convert::*;` は変更不要。`convert/mod.rs` が全サブモジュールを `pub use` することで、既存の `shiguredo_libyuv::function_name` のアクセスパスが維持される。テスト・PBT・fuzz の全呼び出し元も変更不要。

## 完了条件

- `src/convert/` ディレクトリが存在し、10 サブモジュールに分割されていること
- 全公開関数が `shiguredo_libyuv::` からアクセス可能なままであること
- 各サブモジュールの先頭に `#![expect(clippy::too_many_arguments)]` が付与されていること
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること
- `CHANGES.md` の `## develop` → `### misc` セクションに `[CHANGE] convert.rs をサブモジュールに分割する` を追記すること (@voluntas 署名付き)
- 0030、0032、0037 の issue ファイルで `src/convert.rs` を参照している箇所を新しいファイルパスに更新すること

## 解決方法

1. `src/convert/` ディレクトリを作成する
2. `src/convert/mod.rs` を作成し、10 サブモジュールを宣言・再エクスポートする
3. 各サブモジュールファイルを作成し、対応するセクションの関数を移動する。各ファイルの先頭に `#![expect(clippy::too_many_arguments)]` を付与する
4. 各ファイルに必要な `use` 文のみを記載する (不要なインポートは clippy で警告される)
5. プライベートヘルパー (`validate_alpha_src`, `validate_alpha_dst`, `validate_mjpeg_input`) は使用元モジュールに移動する
6. `src/convert.rs` を削除する
7. `cargo clippy --all-targets --all-features -- -D warnings` と `cargo test --workspace` で検証する
8. `CHANGES.md` にエントリを追加する
9. 0030、0032、0037 のファイルパス参照を更新する

### 分割の補足: 既存コードの構成改善

分割と同時に、以下の軽微な構成改善を行う (挙動変更なし、純粋な移動):

- 「追加コピー」セクション (L3622-3713): `i422_copy`, `i444_copy` → `subsampling.rs`、`nv21_copy` → `nv.rs`
- 「追加 NV12/NV21 変換」セクション (L4120-4377): 出力フォーマットに基づき各モジュールに分散
- `abgr_to_nv12`, `abgr_to_nv21` → `argb.rs` (ARGB 版と対称のため)
- `i400_to_i400` → 命名は変更しない (公開 API の破壊的変更を避けるため)
