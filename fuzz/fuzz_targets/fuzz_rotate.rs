//! 回転関数のパニック安全性を検証する
//!
//! 任意の画像サイズと RotationMode で回転関数を呼び出し、
//! パニックしないことを確認する。

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use shiguredo_libyuv::*;

#[derive(Arbitrary, Debug)]
struct FuzzRotate {
    width: u8,
    height: u8,
    rotation: u8,
    data: Vec<u8>,
}

fn even_dim(v: u8) -> usize {
    let v = (v as usize).max(1);
    (v * 2).min(256)
}

fn to_rotation(v: u8) -> RotationMode {
    match v % 4 {
        0 => RotationMode::None,
        1 => RotationMode::Rotate90,
        2 => RotationMode::Rotate180,
        _ => RotationMode::Rotate270,
    }
}

/// 回転後のサイズを計算する
fn rotated_size(width: usize, height: usize, mode: RotationMode) -> (usize, usize) {
    match mode {
        RotationMode::None | RotationMode::Rotate180 => (width, height),
        RotationMode::Rotate90 | RotationMode::Rotate270 => (height, width),
    }
}

fuzz_target!(|input: FuzzRotate| {
    let width = even_dim(input.width);
    let height = even_dim(input.height);
    let mode = to_rotation(input.rotation);
    let src_size = ImageSize::new(width, height);
    let (dw, dh) = rotated_size(width, height, mode);
    let dst_size = ImageSize::new(dw, dh);

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

    // I420 回転
    {
        let y = take(&mut pool, width * height);
        let u = take(&mut pool, (width / 2) * (height / 2));
        let v = take(&mut pool, (width / 2) * (height / 2));
        let src = PlanarImage {
            y: &y, y_stride: width,
            u: &u, u_stride: width / 2,
            v: &v, v_stride: width / 2,
        };
        let mut dst_y = vec![0u8; dw * dh];
        let mut dst_u = vec![0u8; (dw / 2) * (dh / 2)];
        let mut dst_v = vec![0u8; (dw / 2) * (dh / 2)];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: dw,
            u: &mut dst_u, u_stride: dw / 2,
            v: &mut dst_v, v_stride: dw / 2,
        };
        let _ = i420_rotate(&src, src_size, &mut dst, dst_size, mode);
    }

    // ARGB 回転
    {
        let argb = take(&mut pool, width * height * 4);
        let src = PackedImage { data: &argb, stride: width * 4 };
        let mut dst_argb = vec![0u8; dw * dh * 4];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: dw * 4 };
        let _ = argb_rotate(&src, src_size, &mut dst, dst_size, mode);
    }

    // 単一プレーン回転
    {
        let plane = take(&mut pool, width * height);
        let mut dst_plane = vec![0u8; dw * dh];
        let _ = rotate_plane(&plane, width, src_size, &mut dst_plane, dw, dst_size, mode);
    }
});
