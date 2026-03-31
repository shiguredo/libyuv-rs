//! 比較関数のパニック安全性を検証する
//!
//! 任意の画像データで比較関数を呼び出し、パニックしないことを確認する。

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use shiguredo_libyuv::*;

#[derive(Arbitrary, Debug)]
struct FuzzCompare {
    width: u8,
    height: u8,
    seed: u32,
    data: Vec<u8>,
}

fn even_dim(v: u8) -> usize {
    let v = (v as usize).max(1);
    (v * 2).min(256)
}

fuzz_target!(|input: FuzzCompare| {
    let width = even_dim(input.width);
    let height = even_dim(input.height);
    let size = ImageSize::new(width, height);

    let y_len = width * height;
    let uv_len = (width / 2) * (height / 2);

    let mut pool = input.data;
    let take = |pool: &mut Vec<u8>, n: usize| -> Vec<u8> {
        let mut buf = vec![0u8; n];
        let copy_len = pool.len().min(n);
        buf[..copy_len].copy_from_slice(&pool[..copy_len]);
        if copy_len < pool.len() {
            *pool = pool[copy_len..].to_vec();
        } else {
            pool.clear();
        }
        buf
    };

    // I420 PSNR / SSIM
    {
        let ya = take(&mut pool, y_len);
        let ua = take(&mut pool, uv_len);
        let va = take(&mut pool, uv_len);
        let yb = take(&mut pool, y_len);
        let ub = take(&mut pool, uv_len);
        let vb = take(&mut pool, uv_len);
        let src_a = PlanarImage {
            y: &ya, y_stride: width,
            u: &ua, u_stride: width / 2,
            v: &va, v_stride: width / 2,
        };
        let src_b = PlanarImage {
            y: &yb, y_stride: width,
            u: &ub, u_stride: width / 2,
            v: &vb, v_stride: width / 2,
        };
        let _ = i420_psnr(&src_a, &src_b, size);
        let _ = i420_ssim(&src_a, &src_b, size);
    }

    // SSE
    {
        let pa = take(&mut pool, y_len);
        let pb = take(&mut pool, y_len);
        let _ = compute_sum_square_error_plane(&pa, width, &pb, width, size);
    }

    // PSNR 変換
    {
        let _ = sum_square_error_to_psnr(0, y_len as u64);
        let _ = sum_square_error_to_psnr(y_len as u64, y_len as u64);
    }

    // ハミング距離
    {
        let a = take(&mut pool, y_len);
        let b = take(&mut pool, y_len);
        let _ = compute_hamming_distance(&a, &b);
    }

    // DJB2 ハッシュ
    {
        let data = take(&mut pool, y_len);
        let _ = hash_djb2(&data, input.seed);
    }
});
