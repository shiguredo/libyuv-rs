//! 変換 API の単体テストで使う共通ヘルパー
//!
//! 色変換の ground truth 計算に使う参照実装を提供する。
//! 参照実装は libyuv の C 実装 (`row_common.cc`) と同じ整数演算を再現しており、
//! SIMD 実装は C 実装を模倣して作られているため、どの環境でも同じ期待値になる。

/// 8bit 値にクランプする
pub fn clamp8(v: i32) -> u8 {
    v.clamp(0, 255) as u8
}

/// BT.601 limited range の YUV 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `YuvPixel` (kYuvI601Constants) と同じ計算。
/// 係数: YG=18997, YB=-1160, UB=128, UG=25, VG=52, VR=102
pub fn yuv601_to_argb_pixel(y: u8, u: u8, v: u8) -> [u8; 4] {
    // ARM / RISC-V 版の LOAD_YUV_CONSTANTS / CALC_RGB16 と同じ式を使う
    // (x86 版とは定数の置き方が異なるが、結果は同一)
    let y32 = (y as u32) * 0x0101;
    let yg = 18997i32;
    let yb = -1160i32;
    let ub = 128i32;
    let ug = 25i32;
    let vg = 52i32;
    let vr = 102i32;
    let y1 = ((y32 as u64 * yg as u64) >> 16) as i32 + yb;
    let ui = (u as i32) - 128;
    let vi = (v as i32) - 128;
    let b = y1 + ui * ub;
    let g = y1 - (ui * ug + vi * vg);
    let r = y1 + vi * vr;
    [clamp8(b >> 6), clamp8(g >> 6), clamp8(r >> 6), 255]
}

/// BT.601 full range (JPEG) の YUV 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `YuvPixel` (kYuvJPEGConstants) と同じ計算。
/// 係数: YG=16320, YB=32, UB=113, UG=22, VG=46, VR=90
pub fn yuv_jpeg_to_argb_pixel(y: u8, u: u8, v: u8) -> [u8; 4] {
    let y32 = (y as u32) * 0x0101;
    let yg = 16320i32;
    let yb = 32i32;
    let ub = 113i32;
    let ug = 22i32;
    let vg = 46i32;
    let vr = 90i32;
    let y1 = ((y32 as u64 * yg as u64) >> 16) as i32 + yb;
    let ui = (u as i32) - 128;
    let vi = (v as i32) - 128;
    let b = y1 + ui * ub;
    let g = y1 - (ui * ug + vi * vg);
    let r = y1 + vi * vr;
    [clamp8(b >> 6), clamp8(g >> 6), clamp8(r >> 6), 255]
}

/// BT.709 limited range の YUV 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `YuvPixel` (kYuvH709Constants) と同じ計算。
/// 係数: YG=18997, YB=-1160, UB=128, UG=14, VG=34, VR=115
pub fn yuv_h709_to_argb_pixel(y: u8, u: u8, v: u8) -> [u8; 4] {
    let y32 = (y as u32) * 0x0101;
    let yg = 18997i32;
    let yb = -1160i32;
    let ub = 128i32;
    let ug = 14i32;
    let vg = 34i32;
    let vr = 115i32;
    let y1 = ((y32 as u64 * yg as u64) >> 16) as i32 + yb;
    let ui = (u as i32) - 128;
    let vi = (v as i32) - 128;
    let b = y1 + ui * ub;
    let g = y1 - (ui * ug + vi * vg);
    let r = y1 + vi * vr;
    [clamp8(b >> 6), clamp8(g >> 6), clamp8(r >> 6), 255]
}

/// BT.2020 limited range の YUV 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `YuvPixel` (kYuv2020Constants) と同じ計算。
/// 係数: YG=19003, YB=-1160, UB=128, UG=12, VG=42, VR=107
pub fn yuv_2020_to_argb_pixel(y: u8, u: u8, v: u8) -> [u8; 4] {
    let y32 = (y as u32) * 0x0101;
    let yg = 19003i32;
    let yb = -1160i32;
    let ub = 128i32;
    let ug = 12i32;
    let vg = 42i32;
    let vr = 107i32;
    let y1 = ((y32 as u64 * yg as u64) >> 16) as i32 + yb;
    let ui = (u as i32) - 128;
    let vi = (v as i32) - 128;
    let b = y1 + ui * ub;
    let g = y1 - (ui * ug + vi * vg);
    let r = y1 + vi * vr;
    [clamp8(b >> 6), clamp8(g >> 6), clamp8(r >> 6), 255]
}

/// 10bit YUV 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `YuvPixel10` (kYuvI601Constants) と同じ計算。
/// 10bit 値は上位 8bit に丸めてから 8bit と同じ計算で RGB に変換する。
pub fn yuv601_10_to_argb_pixel(y: u16, u: u16, v: u16) -> [u8; 4] {
    // YuvPixel10_16: y32 = (y << 6) | (y >> 4), u/v は clamp255(u >> 2)
    let y32 = ((y as u32) << 6) | ((y as u32) >> 4);
    let u8 = (u >> 2).min(255) as u8;
    let v8 = (v >> 2).min(255) as u8;
    let yg = 18997i32;
    let yb = -1160i32;
    let ub = 128i32;
    let ug = 25i32;
    let vg = 52i32;
    let vr = 102i32;
    let y1 = ((y32 as u64 * yg as u64) >> 16) as i32 + yb;
    let ui = (u8 as i32) - 128;
    let vi = (v8 as i32) - 128;
    let b = y1 + ui * ub;
    let g = y1 - (ui * ug + vi * vg);
    let r = y1 + vi * vr;
    [clamp8(b >> 6), clamp8(g >> 6), clamp8(r >> 6), 255]
}

/// ARGB (メモリ順 B,G,R,A) のデータを RGB565 に変換する
///
/// libyuv の `ARGBToRGB565` と同じビットレイアウト。
/// R 上位 5bit, G 上位 6bit, B 上位 5bit を 16bit にパックする。
/// 出力はリトルエンディアンの 2 バイト (低バイトが R 下位ビット)。
pub fn argb_to_rgb565_pixel(argb: [u8; 4]) -> [u8; 2] {
    let b = argb[0] as u32;
    let g = argb[1] as u32;
    let r = argb[2] as u32;
    let rgb565 = ((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3);
    [(rgb565 & 0xff) as u8, (rgb565 >> 8) as u8]
}

/// 8bit 値を 10bit に拡張する (I420ToI010)
///
/// libyuv の `Convert8To16Row_C` は scale=1024 を `* 0x0101` したうえで
/// `(src * scale) >> 16` を計算する。すなわち `src * 1024 * 257 >> 16` になる。
pub fn u8_to_u10(v: u8) -> u16 {
    (((v as u32) * 1024 * 257) >> 16) as u16
}

/// 12bit 値を 8bit に縮小する (I012ToI420)
///
/// libyuv の `C16TO8(v, 4096)` は `clamp255((v * 4096) >> 16)` = `clamp255(v >> 4)` になる。
pub fn u12_to_u8(v: u16) -> u8 {
    (((v as u32) * 4096) >> 16).min(255) as u8
}

/// 16bit 値を 8bit に縮小する (P010ToNV12 等、depth=16 として扱う場合)
///
/// libyuv の `C16TO8(v, 256)` は `clamp255((v * 256) >> 16)` = `clamp255(v >> 8)` になる。
pub fn u16_to_u8(v: u16) -> u8 {
    (((v as u32) * 256) >> 16).min(255) as u8
}

/// ARGB 1 ピクセル (メモリ順 B,G,R,A) を AR30 (10bit パック) に変換する
///
/// libyuv の `ARGBToAR30Row_C` と同じビットレイアウト。
/// AR30 は 32bit に A(2bit) R(10bit) G(10bit) B(10bit) をパックする。
/// 8bit の B/G/R は上位 2bit を複製して 10bit に拡張する ((v >> 6) | (v << 2))。
pub fn argb_to_ar30_pixel(argb: [u8; 4]) -> [u8; 4] {
    let b0 = ((argb[0] as u32) >> 6) | ((argb[0] as u32) << 2);
    let g0 = ((argb[1] as u32) >> 6) | ((argb[1] as u32) << 2);
    let r0 = ((argb[2] as u32) >> 6) | ((argb[2] as u32) << 2);
    let a0 = (argb[3] as u32) >> 6;
    let ar30 = b0 | (g0 << 10) | (r0 << 20) | (a0 << 30);
    [
        (ar30 & 0xff) as u8,
        ((ar30 >> 8) & 0xff) as u8,
        ((ar30 >> 16) & 0xff) as u8,
        ((ar30 >> 24) & 0xff) as u8,
    ]
}

/// AR30 (10bit パック) 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `AR30ToARGBRow_C` と同じビットレイアウト。
/// B/G/R は 10bit フィールドの上位 8bit を取り出し、A は 2bit を 0x55 倍で 8bit に拡張する。
pub fn ar30_to_argb_pixel(ar30_le: [u8; 4]) -> [u8; 4] {
    let ar30 = u32::from_le_bytes(ar30_le);
    let b = (ar30 >> 2) & 0xff;
    let g = (ar30 >> 12) & 0xff;
    let r = (ar30 >> 22) & 0xff;
    let a = ((ar30 >> 30) * 0x55) & 0xff;
    [b as u8, g as u8, r as u8, a as u8]
}

/// I400 (グレースケール) 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `YPixel` (kYuvI601Constants) と同じ計算。Y を B/G/R に複製する。
/// 係数: YG=18997, YGB=-1160
pub fn y601_to_argb_pixel(y: u8) -> [u8; 4] {
    let yg = 18997i32;
    let ygb = -1160i32;
    let y1 = ((y as u32) * 0x0101 * (yg as u32)) >> 16;
    let v = clamp8(((y1 as i32) + ygb) >> 6);
    [v, v, v, 255]
}

/// ARGB (メモリ順 B,G,R,A) 1 ピクセルを RGBA (メモリ順 A,B,G,R) に変換する
///
/// libyuv の `ARGBToRGBA` はチャンネル順を A,B,G,R (メモリ) に並べ替えるだけ。
/// ARGB (B,G,R,A) → RGBA メモリ (A,B,G,R)。
pub fn argb_to_rgba_pixel(argb: [u8; 4]) -> [u8; 4] {
    [argb[3], argb[0], argb[1], argb[2]]
}

/// I444 の 2x2 ブロックから NV12 の UV 1 ペアを計算する
///
/// libyuv の `HalfMergeUVRow_C` と同じ 4 点平均。
/// u は 4 点の水平 2 × 垂直 2 のブロック、v も同様。
pub fn i444_uv_merge(u: [u8; 4], v: [u8; 4]) -> [u8; 2] {
    let u_avg = ((u[0] as u32 + u[1] as u32 + u[2] as u32 + u[3] as u32 + 2) >> 2) as u8;
    let v_avg = ((v[0] as u32 + v[1] as u32 + v[2] as u32 + v[3] as u32 + 2) >> 2) as u8;
    [u_avg, v_avg]
}

/// AYUV (メモリ順 V,U,Y,A) の 2x2 ブロックから NV12 の UV 1 ペアを計算する
///
/// libyuv の `AYUVToUVRow_C` と同じ 4 点平均。
/// U は AYUV バイト 1 / 5 / stride+1 / stride+5、V は 0 / 4 / stride+0 / stride+4。
pub fn ayuv_uv_merge(u4: [u8; 4], v4: [u8; 4]) -> [u8; 2] {
    let u_avg = ((u4[0] as u32 + u4[1] as u32 + u4[2] as u32 + u4[3] as u32 + 2) >> 2) as u8;
    let v_avg = ((v4[0] as u32 + v4[1] as u32 + v4[2] as u32 + v4[3] as u32 + 2) >> 2) as u8;
    [u_avg, v_avg]
}

/// NV21 (VU インターリーブ) の 2 ピクセルを YUV24 (V,U,Y) に変換する
///
/// libyuv の `NV21ToYUV24Row_C` と同じ配置。2 ピクセルで VU 1 ペアを使い回す。
pub fn nv21_to_yuv24_pixel(y0: u8, y1: u8, vu: [u8; 2]) -> [u8; 6] {
    [vu[0], vu[1], y0, vu[0], vu[1], y1]
}

/// RGB565 (リトルエンディアン 2 バイト) 1 ピクセルを ARGB (メモリ順 B,G,R,A) に変換する
///
/// libyuv の `RGB565ToARGBRow_C` と同じビット展開。
pub fn rgb565_to_argb_pixel(rgb565: [u8; 2]) -> [u8; 4] {
    let b5 = (rgb565[0] & 0x1f) as u32;
    let g6 = (((rgb565[0] >> 5) | ((rgb565[1] & 0x07) << 3)) & 0x3f) as u32;
    let r5 = (rgb565[1] >> 3) as u32;
    let b = ((b5 << 3) | (b5 >> 2)) as u8;
    let g = ((g6 << 2) | (g6 >> 4)) as u8;
    let r = ((r5 << 3) | (r5 >> 2)) as u8;
    [b, g, r, 255]
}

/// BT.601 full range (JPEG) の ARGB 1 ピクセル (メモリ順 B,G,R,A) を YUV に変換する
///
/// libyuv の `ARGBToJ420` (kArgbJPEGConstants) と同じ係数。
/// Y = (77*R + 150*G + 29*B + 128) >> 8
/// U = (32768 + 128*B - 85*G - 43*R) >> 8
/// V = (32768 + 128*R - 107*G - 21*B) >> 8
/// UV は 2x2 ブロックの平均を取ってから変換する (ARGBToUVMatrixRow_C)。
pub fn argb_to_yuv_jpeg_pixel(argb: [u8; 4]) -> (u8, u8, u8) {
    let b = argb[0] as i32;
    let g = argb[1] as i32;
    let r = argb[2] as i32;
    let y = (77 * r + 150 * g + 29 * b + 128) >> 8;
    let u = (32768 + 128 * b - 85 * g - 43 * r) >> 8;
    let v = (32768 + 128 * r - 107 * g - 21 * b) >> 8;
    (y as u8, u as u8, v as u8)
}

/// ARGB (メモリ順 B,G,R,A) の 2x2 ブロックの平均を取ってから
/// BT.601 full range (JPEG) の YUV に変換する (ARGBToJ420 の UV 用)
pub fn argb_block_to_yuv_jpeg_uv(block: [[u8; 4]; 4]) -> (u8, u8) {
    let mut b_sum = 0i32;
    let mut g_sum = 0i32;
    let mut r_sum = 0i32;
    for px in block {
        b_sum += px[0] as i32;
        g_sum += px[1] as i32;
        r_sum += px[2] as i32;
    }
    let b = (b_sum + 2) >> 2;
    let g = (g_sum + 2) >> 2;
    let r = (r_sum + 2) >> 2;
    let u = (32768 + 128 * b - 85 * g - 43 * r) >> 8;
    let v = (32768 + 128 * r - 107 * g - 21 * b) >> 8;
    (u as u8, v as u8)
}
