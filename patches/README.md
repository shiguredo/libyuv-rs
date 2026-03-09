# patches

libyuv 上流のバグを修正するためのパッチ群。
`build.rs` の `apply_patches` 関数により、libyuv のビルド前に `git apply` で自動適用される。

## fix-any13-avx2-buffer-overflow.patch

### 関連 issue

- `issues/pending/0017-bug-libyuv-any13-avx2-buffer-overflow.md`

### 対象ファイル

- `source/row_any.cc` — `ANY13` マクロ

### 背景

libyuv の `row_any.cc` には、SIMD 命令で処理しきれない端数ピクセルを処理するための `ANY13` マクロがある。
このマクロは入力 (interleaved RGB) を受け取り、R/G/B の 3 プレーンに分離する SIMD 関数のラッパーである。

```c
#define ANY13(NAMEANY, ANY_SIMD, BPP, MASK)
```

- `BPP` — 入力の 1 ピクセルあたりのバイト数 (RGB の場合 3)
- `MASK` — SIMD 処理単位 - 1 (SSE 系は 15、AVX2 は 31)

### 問題

`ANY13` マクロ内のスタックバッファが SSE 系 (MASK=15) を前提とした固定サイズでハードコードされている。

```c
SIMD_ALIGNED(uint8_t vin[16 * 3]);   // 48 バイト固定
SIMD_ALIGNED(uint8_t vout[16 * 3]);  // 48 バイト固定
```

AVX2 バリアント `SplitRGBRow_Any_AVX2` は `ANY13(SplitRGBRow_Any_AVX2, SplitRGBRow_AVX2, 3, 31)` で展開される。
MASK=31 のとき、以下の 3 箇所でバッファオーバーフローが発生する。

#### 1. 入力バッファ `vin` への書き込み

```c
memcpy(vin, src_ptr + np * BPP, r * BPP);
```

`r` は最大 31 なので、最大 31 * 3 = 93 バイトを 48 バイトのバッファに書き込む。

#### 2. SIMD 関数 `SplitRGBRow_AVX2` による `vin` の読み込み

```c
ANY_SIMD(vin, vout, vout + 16, vout + 32, MASK + 1);  // 32 ピクセル処理
```

`SplitRGBRow_AVX2` は AVX2 命令 (`vmovdqu`) で 32 バイト単位 × 3 回 = 96 バイトを読み込む。
`vin` は 48 バイトしかないため、48 バイトのオーバーリードが発生する。

#### 3. 出力バッファ `vout` への書き込み

```c
ANY_SIMD(vin, vout, vout + 16, vout + 32, MASK + 1);
```

`SplitRGBRow_AVX2` は各出力プレーンに 32 バイトを書き込む。

| 出力先 | 書き込み範囲 | バッファ範囲 | 結果 |
|--------|-------------|-------------|------|
| `vout` | 0 - 31 | 0 - 47 | 安全 |
| `vout + 16` | 16 - 47 | 0 - 47 | `vout` の R 出力を上書き |
| `vout + 32` | 32 - 63 | 0 - 47 | **16 バイトのオーバーフロー** |

`vout + 16` への書き込みが `vout` の後半を上書きし、`vout + 32` への書き込みがバッファ末尾を超える。

### 発火条件

- x86_64 + AVX2 対応 CPU (Linux CI 環境など)
- `SplitRGBPlane` を stride == width * 3 で呼び出すと全行が結合され、結合後の幅が 32 の倍数でない場合に発火
- 行結合なしでも、width が 32 の倍数でなければ発火
- macOS ARM (NEON, MASK=15) では `vin[48]` で十分なため発火しない

### 修正内容

入力バッファと出力バッファの両方を MASK と BPP に基づく動的サイズに変更する。

**入力バッファ:**

```c
// 修正前: 48 バイト固定
SIMD_ALIGNED(uint8_t vin[16 * 3]);

// 修正後: MASK と BPP に基づく動的サイズ (AVX2 では 96 バイト)
SIMD_ALIGNED(uint8_t vin[((MASK) + 1) * (BPP)]);
```

**出力バッファ:**

```c
// 修正前: 48 バイトの単一バッファを 16 バイトオフセットで共有
SIMD_ALIGNED(uint8_t vout[16 * 3]);
ANY_SIMD(vin, vout, vout + 16, vout + 32, MASK + 1);

// 修正後: プレーンごとに独立したバッファを確保
SIMD_ALIGNED(uint8_t vr[(MASK) + 1]);
SIMD_ALIGNED(uint8_t vg[(MASK) + 1]);
SIMD_ALIGNED(uint8_t vb[(MASK) + 1]);
ANY_SIMD(vin, vr, vg, vb, MASK + 1);
```

この修正により、SSE 系 (MASK=15) でも AVX2 (MASK=31) でも正しいサイズのバッファが確保される。

### 上流の状況

libyuv 上流の最新 HEAD (`30809ff6`, 2026-03-03) では未修正。
SSE 系 (MASK=15) では `vin[48]` が十分なサイズであり、`vout` の出力も 16 バイト単位でちょうど収まるため、
Google の CI で AVX2 固有の問題が検出されていない可能性がある。
