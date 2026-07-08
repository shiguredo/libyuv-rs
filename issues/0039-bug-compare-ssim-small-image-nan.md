# calc_frame_ssim と i420_ssim が小さな画像で NaN を返す

- Priority: High
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/fix-ssim-small-image-nan
- Polished: {YYYY-MM-DD}
- Reporter: @voluntas

## 目的

`src/compare.rs` の `calc_frame_ssim` と `i420_ssim` において、画像サイズが SSIM アルゴリズムの最小ブロックサイズ（8x8）未満の場合に libyuv 内部で `samples == 0` となり除算ゼロが発生し、NaN を返す問題を修正する。

## 優先度根拠

High。

- libyuv の `CalcFrameSsim` / `I420Ssim` は内部で 8x8 ブロック単位で SSIM を計算する。width < 8 または height < 8 の場合、有効なブロックが 0 個になり `samples == 0` となる
- このとき C 側で `ssim_total / samples` が除算ゼロとなり NaN が発生する
- Rust 側では入力検証を exhaustive に行っているが、最小サイズ（8x8）の検証がないため NaN を正常値として返す
- 呼び出し元が NaN を検出できず、後続の計算に波及する

## 現状

`src/compare.rs` の `calc_frame_ssim` (L160-237) と `i420_ssim` (L43-71) は、c_int 範囲チェック、stride チェック、バッファサイズチェックをすべて行っているが、width >= 8 かつ height >= 8 の最小サイズチェックがない。

```rust
// calc_frame_ssim (L160-237) — 検証は充実しているが最小サイズチェックがない
pub fn calc_frame_ssim(
    src_a: &[u8],
    src_a_stride: usize,
    src_b: &[u8],
    src_b_stride: usize,
    size: ImageSize,
) -> Result<f64, Error> {
    // require_c_int, stride >= width, checked_buf_size はすべて実装済み
    // しかし width < 8 または height < 8 のチェックがない
    let result = unsafe {
        sys::CalcFrameSsim(..., size.width as c_int, size.height as c_int)
    };
    Ok(result) // width=4, height=4 で NaN が返る
}
```

同様に `i420_ssim` (L43-71) も `.validate()` による検証は行っているが、validate は最小サイズ（8x8）をチェックしない。

## 設計方針

1. `calc_frame_ssim` と呼び出し元の直前に `size.width < 8 || size.height < 8` のチェックを追加し、該当する場合はエラーを返す
2. `i420_ssim` も同様にチェックを追加する（`.validate()` ではカバーされないため）
3. このチェックは libyuv の SSIM アルゴリズムの要件（最小 8x8 ブロック）に由来するため、libyuv のソースコメントまたはヘッダの記述を根拠として引用する

## 完了条件

- `calc_frame_ssim` と `i420_ssim` が width < 8 または height < 8 の入力に対して NaN ではなく Err を返すこと
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX] calc_frame_ssim と i420_ssim が 8x8 未満の小さな画像で NaN を返す問題を修正する` を追記すること (@voluntas 署名付き)
- `tests/test_compare.rs` に width=4, height=4 等の小さなサイズで Err が返ることを確認するテストを追加すること

## 解決方法

1. `calc_frame_ssim` (L199) の検証ブロックに以下を追加する:
   ```rust
   if size.width < 8 || size.height < 8 {
       return Err(Error::with_reason(
           -1,
           "CalcFrameSsim",
           "image must be at least 8x8 for SSIM calculation",
       ));
   }
   ```
2. `i420_ssim` (L46) の `.validate()` 呼び出しの直後に同様のチェックを追加する
3. libyuv の `CalcFrameSsim` / `I420Ssim` 実装（compare.cc）の 8x8 ブロック制約をコードコメントとして引用する
4. `tests/test_compare.rs` を作成し、width=4, height=4 のケースで Err が返ることを確認するテストを追加する
5. `CHANGES.md` に `[FIX]` エントリを追加する
