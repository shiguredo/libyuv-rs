# compare.rs のゼロ除算リスク・エラー戻り値未チェック・API 一貫性欠如

- Priority: High
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/fix-compare-zero-division-error-check
- Polished: 2026-07-08
- Reporter:

## 目的

`src/compare.rs` の以下の 3 つの問題を修正する:

1. `sum_square_error_to_psnr` が count == 0 のゼロ除算を検出しない
2. `calc_frame_psnr` / `calc_frame_ssim` が libyuv のエラー戻り値（負値）をチェックせず常に Ok を返す
3. `hash_djb2` が `Result` を返さず `u32` を直接返しており、他の公開 API と一貫性がない

## 優先度根拠

High。

- `sum_square_error_to_psnr` のゼロ除算は C 側で NaN/Inf を生成し、後続の計算に波及する
- `calc_frame_psnr` / `calc_frame_ssim` のエラー握り潰しは不正な PSNR/SSIM 値を正常値として返すリスクがある
- `hash_djb2` の戻り値型変更は破壊的変更であり、`CHANGES.md` に `[CHANGE]` として記載が必要

## 現状

### sum_square_error_to_psnr (src/compare.rs:357-358)

```rust
pub fn sum_square_error_to_psnr(sse: u64, count: u64) -> f64 {
    unsafe { sys::SumSquareErrorToPsnr(sse, count) }
}
```
`count` に対する検証が一切なく、`count == 0` の場合 C 側で `10.0 * log10(255.0 * 255.0 * count / sse)` の計算時にゼロ除算が発生し `NaN`/`Inf` になる。

### calc_frame_psnr / calc_frame_ssim (src/compare.rs:142-153, 225-236)
libyuv の `CalcFramePsnr` / `CalcFrameSsim` はエラー時に負値（-1.0）を返すが、Rust 側では戻り値を常に `Ok(result)` でラップしているため、呼び出し元がエラーを検出できない。

### hash_djb2 (src/compare.rs:371-372)

```rust
pub fn hash_djb2(src: &[u8], seed: u32) -> u32 {
    unsafe { sys::HashDjb2(src.as_ptr(), src.len() as u64, seed) }
}
```

他の全 compare.rs 公開 API は `Result<_, Error>` を返すが、`hash_djb2` は `u32` を直接返す。

## 設計方針

1. `sum_square_error_to_psnr`: 戻り値型を `Result<f64, Error>` に変更し、`count == 0` の場合に `Err` を返す。`sse == 0` かつ `count > 0` の場合は `f64::INFINITY`（libyuv の挙動）となる
2. `calc_frame_psnr` / `calc_frame_ssim`: 戻り値が負（-1.0 以下）の場合に `Err` を返す
3. `hash_djb2`: `Result<u32, Error>` を返すよう変更する。空スライスの場合はエラーとする

## 完了条件

- `sum_square_error_to_psnr(0, 0)` が `Err` を返すこと（戻り値型変更）
- `calc_frame_psnr` / `calc_frame_ssim` が libyuv エラー時に `Err` を返すこと
- `hash_djb2` が `Result<u32, Error>` を返すこと
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --all` が成功すること
- `CHANGES.md` の `## develop` セクションに `[CHANGE] sum_square_error_to_psnr の戻り値型を Result<f64, Error> に変更` エントリを追加すること
- `CHANGES.md` の `## develop` セクションに `[FIX] calc_frame_psnr/calc_frame_ssim が libyuv のエラー戻り値を検出するよう修正` エントリを追加すること

## 解決方法

1. `sum_square_error_to_psnr` のシグネチャを `Result<f64, Error>` に変更し、`if count == 0 { return Err(Error::with_reason(-1, "sum_square_error_to_psnr", "count must be greater than 0")) }` を追加する
2. `calc_frame_psnr` / `calc_frame_ssim` の `Ok(result)` を `if result < 0.0 { Err(...) } else { Ok(result) }` に変更する
3. `hash_djb2` のシグネチャを `Result<u32, Error>` に変更し、`src.is_empty()` 時にエラーを返す
4. `CHANGES.md` に `[CHANGE]` と `[FIX]` エントリを追加する
5. 必要に応じて `tests/test_compare.rs` にエラーパスのテストを追加する
