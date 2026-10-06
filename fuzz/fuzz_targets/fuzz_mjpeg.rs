//! MJPEG 系 API のパニック安全性を検証する。
//!
//! 戦略:
//!   - `mjpeg_size(src)` を最初に呼び、画像サイズが取得できれば動的算出した dst バッファで
//!     `mjpeg_to_i420` / `mjpeg_to_nv12` / `mjpeg_to_nv21` / `mjpeg_to_argb` の **すべて** を呼ぶ
//!   - 取得できなければ固定サイズ (8x8) と固定 dst バッファで各 MJPGTo* を呼び、
//!     入力検証が空 / 不正 src を適切にエラー化することを確認する
//!
//! クラッシュ・メモリ不正アクセスが起きないことを検査する。

#![no_main]

use libfuzzer_sys::fuzz_target;
use shiguredo_libyuv::{
    ArgbImageMut, I420ImageMut, ImageSize, Nv12ImageMut, Nv21ImageMut, mjpeg_size, mjpeg_to_argb,
    mjpeg_to_i420, mjpeg_to_nv12, mjpeg_to_nv21,
};

// 確保するピクセル数の上限。fuzz harness の OOM を防ぐためのガード。
// 4 MiB ピクセルで各 dst バッファを確保しても、I420 (1.5 bytes/px) + NV12/NV21 (1.5 bytes/px x 2)
// + ARGB (4 bytes/px) を順に確保していくため、最大同時使用量は ARGB の 16 MiB に収まる。
const MAX_PIXELS: usize = 4 * 1024 * 1024;

fuzz_target!(|src: &[u8]| {
    match mjpeg_size(src) {
        Ok(size) => {
            // OOM 対策: ピクセル数が上限を超えるならスキップ
            let pixels = size.width.saturating_mul(size.height);
            if pixels == 0 || pixels > MAX_PIXELS {
                return;
            }
            run_all_converters(src, size, pixels);
        }
        Err(_) => {
            // 取得できなかった場合は固定サイズで入力検証パスを刺激する
            let small = ImageSize::new(8, 8);
            run_all_converters(src, small, small.width * small.height);
        }
    }
});

fn run_all_converters(src: &[u8], size: ImageSize, pixels: usize) {
    // I420
    {
        let uv_w = size.width.div_ceil(2);
        let uv_h = size.height.div_ceil(2);
        let uv_size = uv_w * uv_h;
        let mut y = vec![0u8; pixels];
        let mut u = vec![0u8; uv_size];
        let mut v = vec![0u8; uv_size];
        let mut dst = I420ImageMut {
            y: &mut y,
            y_stride: size.width,
            u: &mut u,
            u_stride: uv_w,
            v: &mut v,
            v_stride: uv_w,
        };
        let _ = mjpeg_to_i420(src, &mut dst, size);
    }

    // NV12
    {
        let uv_w = size.width.div_ceil(2) * 2;
        let uv_h = size.height.div_ceil(2);
        let uv_size = uv_w * uv_h;
        let mut y = vec![0u8; pixels];
        let mut uv = vec![0u8; uv_size];
        let mut dst = Nv12ImageMut {
            y: &mut y,
            y_stride: size.width,
            uv: &mut uv,
            uv_stride: uv_w,
        };
        let _ = mjpeg_to_nv12(src, &mut dst, size);
    }

    // NV21
    {
        let uv_w = size.width.div_ceil(2) * 2;
        let uv_h = size.height.div_ceil(2);
        let uv_size = uv_w * uv_h;
        let mut y = vec![0u8; pixels];
        let mut uv = vec![0u8; uv_size];
        let mut dst = Nv21ImageMut {
            y: &mut y,
            y_stride: size.width,
            uv: &mut uv,
            uv_stride: uv_w,
        };
        let _ = mjpeg_to_nv21(src, &mut dst, size);
    }

    // ARGB
    {
        let mut data = vec![0u8; pixels * 4];
        let mut dst = ArgbImageMut {
            data: &mut data,
            stride: size.width * 4,
        };
        let _ = mjpeg_to_argb(src, &mut dst, size);
    }
}
