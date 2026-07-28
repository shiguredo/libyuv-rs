# compare.rs のゼロ除算リスク・エラー戻り値未チェック・API 一貫性欠如

- Priority: High
- Created: 2026-07-08
- Completed: 2026-07-29
- Model: DeepSeek V4 Pro
- Branch: feature/fix-compare-zero-division-error-check
- Polished: 2026-07-29
- Reporter:

## 目的

`src/compare.rs` の以下の問題を修正する:

1. `sum_square_error_to_psnr` が count == 0 の場合に -inf を返す（エラーとして検出すべき）
2. `calc_frame_psnr` / `i420_psnr` が width == 0 または height == 0 の場合に意味のない値（kMaxPsnr = 128.0）を正常値として返す
3. `hash_djb2` が `Result` を返さず `u32` を直接返しており、他の公開 API と一貫性がない

## 優先度根拠

High。

- `sum_square_error_to_psnr` の count == 0 は -inf を生成し、後続の計算に波及する
- `calc_frame_psnr` / `i420_psnr` のゼロサイズ入力は意味のない値を正常値として返す
- `hash_djb2` の戻り値型変更は破壊的変更であり、`CHANGES.md` に `[CHANGE]` として記載が必要

## 現状

### sum_square_error_to_psnr (src/compare.rs:357-359)

```rust
pub fn sum_square_error_to_psnr(sse: u64, count: u64) -> f64 {
    unsafe { sys::SumSquareErrorToPsnr(sse, count) }
}
```

`count` に対する検証が一切ない。C 実装 (`compare.cc:271-285`) の挙動:
- `sse > 0 && count == 0`: `mse = 0.0 / sse = 0.0` → `10.0 * log10(255.0 * 255.0 * 0.0)` = `10.0 * log10(0.0)` = **-inf**
- `sse == 0`: `kMaxPsnr` (128.0, `compare.h:53`) を返す（除算回避）

### calc_frame_psnr / i420_psnr (src/compare.rs:77-154, 10-38)

`calc_frame_psnr` は c_int 範囲チェック・stride チェック・バッファサイズチェックを行うが、width == 0 / height == 0 のチェックがない。C 実装 (`compare.cc:288-298`) は `samples = width * height` を計算し `SumSquareErrorToPsnr(sse, samples)` を呼ぶ。width == 0 または height == 0 の場合 samples == 0 となり、sse == 0（ループが実行されないため）なので kMaxPsnr (128.0) が返る。

`i420_psnr` (L10-38) も `.validate()` による検証は行うが、ゼロサイズチェックはない。同様に kMaxPsnr を返す。

### calc_frame_ssim / i420_ssim の NaN 問題について

`calc_frame_ssim` / `i420_ssim` は 8x8 未満の画像で `samples == 0` となり `ssim_total / samples` が NaN になる問題があるが、これは **issue 0039** で別途対応する。本 issue では SSIM 系は対象外。

### hash_djb2 (src/compare.rs:371-373)

```rust
pub fn hash_djb2(src: &[u8], seed: u32) -> u32 {
    unsafe { sys::HashDjb2(src.as_ptr(), src.len() as u64, seed) }
}
```

他の全 compare.rs 公開 API（本 issue の修正後は `sum_square_error_to_psnr` を含む）は `Result<_, Error>` を返すが、`hash_djb2` は `u32` を直接返す。C の `HashDjb2` は空スライスに対して seed をそのまま返す well-defined な挙動を持つ。

## 設計方針

1. `sum_square_error_to_psnr`: 戻り値型を `Result<f64, Error>` に変更し、`count == 0` の場合に `Err` を返す。`sse == 0 && count > 0` の場合は C の挙動どおり `kMaxPsnr` (128.0) が返る（libyuv の透過的ラッパーとして C の挙動をそのまま踏襲する）
2. `calc_frame_psnr` / `i420_psnr`: `size.width == 0 || size.height == 0` の場合に `Err` を返す（FFI を呼ばない。意味のない値が正常値として返るのを防ぐ）
3. `hash_djb2`: `Result<u32, Error>` を返すよう変更する。空スライスは C の挙動どおり seed をそのまま返す（`Ok(seed)`）。エラー条件は `src.len()` が u64 範囲を超える場合のみ（実質的に発生しないが API 一貫性のため）

**libyuv のエラー戻り値について**: `CalcFramePsnr` / `CalcFrameSsim` / `I420Psnr` / `I420Ssim` はエラー時に負値を返すコードパスを持たない（`compare.cc` を確認済み）。これらの関数は入力検証を Rust 側で行い、FFI 呼び出し後は結果をそのまま返す設計とする。

## 完了条件

- `sum_square_error_to_psnr(sse, 0)` が `Err` を返すこと（戻り値型 `Result<f64, Error>` に変更）
- `sum_square_error_to_psnr(0, count)` （count > 0）が `Ok(128.0)` を返すこと（C の kMaxPsnr 挙動の踏襲）
- `calc_frame_psnr` / `i420_psnr` が width == 0 または height == 0 の入力に対して `Err` を返すこと
- `hash_djb2` が `Result<u32, Error>` を返すこと
- `hash_djb2(&[], seed)` が `Ok(seed)` を返すこと（C の挙動の踏襲）
- `tests/test_compare.rs` に以下のテストを追加すること:
  - `sum_square_error_to_psnr` の count == 0 で Err
  - `sum_square_error_to_psnr` の sse == 0, count > 0 で Ok(128.0)
  - `calc_frame_psnr` の width == 0 / height == 0 で Err
  - `hash_djb2` の空スライスで Ok(seed)
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること
- `CHANGES.md` の `## develop` セクションに以下を追記すること（@voluntas 署名付き）:
  - `[CHANGE] sum_square_error_to_psnr の戻り値型を Result<f64, Error> に変更し count == 0 の検証を追加する`
  - `[FIX] calc_frame_psnr と i420_psnr がゼロサイズ入力で意味のない値を返す問題を修正する`
  - `[CHANGE] hash_djb2 の戻り値型を Result<u32, Error> に変更する`

## 解決方法

`src/compare.rs` の 4 関数を以下のように修正した:

1. `sum_square_error_to_psnr`: 戻り値型を `Result<f64, Error>` に変更し、`count == 0` で `Err` を返す検証を追加
2. `calc_frame_psnr`: `size.width == 0 || size.height == 0` で `Err` を返す検証を追加（stride チェックの前）
3. `i420_psnr`: `.validate()` の直後に同様のゼロサイズチェックを追加
4. `hash_djb2`: 戻り値型を `Result<u32, Error>` に変更（API 一貫性のため。空スライスは `Ok(seed)` を返す）

doc コメントの `f64::INFINITY` 記述を実挙動 (`kMaxPsnr` = 128.0) に修正した。

`tests/test_compare.rs` を新規作成し、テスト 9 件を追加した:

- 異常系: count == 0、width == 0、height == 0（calc_frame_psnr / i420_psnr 各々）
- 正常系: sse == 0 で Ok(128.0)、同一バッファで Ok(128.0)、空スライスで Ok(seed)、非空データで Ok

`CHANGES.md` の `## develop` に `[CHANGE]` 2 件と `[FIX]` 1 件を追加した。

## 注記

- issue 0039（SSIM の NaN 問題）とは補完関係。0039 は SSIM 系の 8x8 最小サイズ検証、本 issue は PSNR 系のゼロサイズ検証と API 一貫性を扱う。`calc_frame_ssim` / `i420_ssim` のエラーチェックは 0039 の管轄
- `calc_frame_ssim` / `i420_ssim` の `Ok(result)` パターンは libyuv がエラー戻り値を持たないため意図的な設計。NaN の防止は 0039 の入力検証で対応する
