# libyuv 上流バグ: ANY13 マクロの AVX2 バッファオーバーフロー

## 概要

libyuv の `row_any.cc` にある `ANY13` マクロのスタックバッファサイズが AVX2 バリアントに対して不足しており、
`SplitRGBRow_Any_AVX2` 使用時にバッファオーバーフローが発生する。

## 再現環境

- Linux x86_64 + AVX2 対応 CPU (GitHub Actions ランナー)
- macOS (ARM/NEON) では別コードパスを通るため再現しない

## 再現手順

```bash
cargo test -p pbt --test prop_planar -- roundtrip_split_merge_rgb
```

## 原因

`row_any.cc` の `ANY13` マクロ:

```c
#define ANY13(NAMEANY, ANY_SIMD, BPP, MASK)
  SIMD_ALIGNED(uint8_t vin[16 * 3]);   // 48 バイト固定
  SIMD_ALIGNED(uint8_t vout[16 * 3]);  // 48 バイト固定
  ...
  memcpy(vin, src_ptr + np * BPP, r * BPP);
  ANY_SIMD(vin, vout, vout + 16, vout + 32, MASK + 1);
```

`SplitRGBRow_Any_AVX2` は `ANY13(SplitRGBRow_Any_AVX2, SplitRGBRow_AVX2, 3, 31)` で生成される。

MASK=31 のとき:
1. `memcpy(vin, ..., r * 3)` — r は最大 31、31 * 3 = 93 バイトを 48 バイトバッファに書き込み → **オーバーフロー**
2. `SplitRGBRow_AVX2(vin, ...)` — 96 バイト読み込みだが vin は 48 バイト → **オーバーフロー**
3. `vout + 32` に 32 バイト書き込みだが残り 16 バイト → **オーバーフロー**

## 発火条件

`SplitRGBPlane` は stride == width * 3 のとき全行を結合して 1 行として処理する。
結合後の幅が 32 の倍数でなく、余り (`width & 31`) が 17 以上のとき `memcpy` で検出される。

直接呼び出しでも、width が 32 の倍数でなく余りが 17 以上なら同じ。

## CI ログ

```
test nv12_mirror_twice ... ok
*** buffer overflow detected ***: terminated
```

https://github.com/shiguredo/libyuv-rs/actions/runs/22860116385/job/66311383646

## 影響範囲

- `split_rgb_plane` (内部で `SplitRGBRow_Any_AVX2` を使用)
- Linux x86_64 で AVX2 対応 CPU のみ

## 解決方法

`patches/fix-any13-avx2-buffer-overflow.patch` を作成し、build.rs で libyuv ビルド前に自動適用するようにした。

パッチの内容:
- `vin` のサイズを `(MASK + 1) * BPP` に拡大 (AVX2 では 96 バイト)
- `vout` を 3 つの個別バッファ `vr`, `vg`, `vb` に分離し、出力プレーンの重複書き込みを防止

libyuv 上流は最新 HEAD (30809ff6) でも未修正のため、パッチは当面維持する必要がある。
