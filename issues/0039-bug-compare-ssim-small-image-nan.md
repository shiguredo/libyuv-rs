# calc_frame_ssim と i420_ssim が小さな画像で NaN を返す

- Priority: High
- Created: 2026-07-08
- Completed: 2026-07-30
- Model: DeepSeek V4 Pro
- Branch: feature/fix-ssim-small-image-nan
- Polished: 2026-07-29
- Reporter: @voluntas

## 目的

`src/compare.rs` の `calc_frame_ssim` と `i420_ssim` において、画像サイズが SSIM アルゴリズムの最小要件（width >= 9 かつ height >= 9）を満たさない場合に libyuv 内部で `samples == 0` となり除算ゼロが発生し、NaN を返す問題を修正する。

## 優先度根拠

High。

- libyuv の `CalcFrameSsim` / `I420Ssim` は内部で 8x8 ブロック単位で SSIM を計算する。ループ条件は `i < height - 8` と `j < width - 8`（`compare.cc:399-401`）であるため、width <= 8 または height <= 8 の場合ループが一度も実行されず `samples == 0` となる
- このとき C 側で `ssim_total / samples`（`compare.cc:410`）が除算ゼロとなり NaN が発生する
- Rust 側では入力検証を exhaustive に行っているが、最小サイズの検証がないため NaN を正常値として返す
- 呼び出し元が NaN を検出できず、後続の計算に波及する

## 現状

`src/compare.rs` の `calc_frame_ssim` (L160-237) と `i420_ssim` (L43-71) は、c_int 範囲チェック、stride チェック、バッファサイズチェックをすべて行っているが、最小サイズ（width >= 9 かつ height >= 9）のチェックがない。

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
    // しかし width <= 8 または height <= 8 のチェックがない
    let result = unsafe {
        sys::CalcFrameSsim(..., size.width as c_int, size.height as c_int)
    };
    Ok(result) // width=8, height=8 で NaN が返る
}
```

同様に `i420_ssim` (L43-71) も `.validate()` による検証は行っているが、validate は最小サイズをチェックしない。

## 設計方針

1. `calc_frame_ssim` の検証ブロックに `size.width <= 8 || size.height <= 8` のチェックを追加し、該当する場合はエラーを返す
2. `i420_ssim` も同様にチェックを追加する（`.validate()` ではカバーされないため）
3. このチェックは libyuv の SSIM アルゴリズムのループ条件（`compare.cc:399-401` の `i < height - 8`, `j < width - 8`）に由来する。8x8 ブロックを 4 ピクセルステップで走査するため、最低でも 9 ピクセル以上の幅と高さが必要

## 完了条件

- `calc_frame_ssim` と `i420_ssim` が width <= 8 または height <= 8 の入力に対して NaN ではなく Err を返すこと
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること
- `CHANGES.md` の `## develop` セクションに `[FIX] calc_frame_ssim と i420_ssim が 9x9 未満の小さな画像で NaN を返す問題を修正する` を追記すること (@voluntas 署名付き)
- `tests/test_compare.rs` に width=8, height=8（境界値）および width=4, height=4 で Err が返ることを確認するテストを追加すること

## 解決方法

1. `calc_frame_ssim` の検証ブロックに `size.width <= 8 || size.height <= 8` チェックを追加した
2. `i420_ssim` の `.validate()` 呼び出し直後に同様のチェックを追加した
3. `tests/test_compare.rs` に 8x8（境界値）および 4x4 で Err が返るテスト 4 件を追加した
4. `CHANGES.md` に `[FIX]` エントリを追加した
