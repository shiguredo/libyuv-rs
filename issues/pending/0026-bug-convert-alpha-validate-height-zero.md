# アルファプレーン検証で size.height == 0 時に panic する

- Priority: High
- Created: 2026-06-19
- Completed: {YYYY-MM-DD}
- Model: Kimi K2.7 Code
- Branch: feature/fix-convert-alpha-validate-height-zero
- Polished: {YYYY-MM-DD}
- Reporter:

## 目的

`convert.rs` の `validate_alpha_src` / `validate_alpha_dst` が `size.height == 0` のとき `usize` の減算で panic するバグを修正する。

## 優先度根拠

debug ビルドで即座に panic し、release ビルドでも整数ラップにより誤った検証結果になりうる。公開 API (`i420_alpha_to_argb` 等) を通じて容易に引き起こせる。

## 現状

`src/convert.rs:4976-5009` の `validate_alpha_src` / `validate_alpha_dst` は、バッファサイズを以下のように計算している。

```rust
let required = src_stride_a * (size.height - 1) + size.width;
```

`size.height == 0` のとき `size.height - 1` が `usize` のオーバーフローとなり panic する。

## 設計方針

`size.height == 0` の場合は必要バッファサイズが 0 なので即 `Ok(())` を返す。それ以外の場合は `checked_mul` / `checked_add` で安全に計算する。

## 完了条件

- `size.height == 0` の入力で `i420_alpha_to_argb` 等が panic せず、`Error` を返すこと
- `src_stride_a < size.width` の場合も適切にエラーにすること
- 該当する境界値テストが追加されていること
- `cargo test --all` が成功すること

## 解決方法

1. `validate_alpha_src` / `validate_alpha_dst` に `size.height == 0` の早期リターンを追加する
2. `src_stride_a * (size.height - 1) + size.width` を `checked_mul` / `checked_add` に置き換える
3. `src_stride_a >= size.width` / `dst_stride_a >= size.width` の検証を追加する
4. `tests/test_convert.rs` を新設し、境界値をテストする
