//! スケール関数のパニック安全性を検証する
//!
//! 任意のソース・デスティネーションサイズと FilterMode で
//! スケール関数を呼び出し、パニックしないことを確認する。

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use shiguredo_libyuv::*;

#[derive(Arbitrary, Debug)]
struct FuzzScale {
    src_width: u8,
    src_height: u8,
    dst_width: u8,
    dst_height: u8,
    filter: u8,
    data: Vec<u8>,
}

fn even_dim(v: u8) -> usize {
    let v = (v as usize).max(1);
    (v * 2).min(256)
}

fn to_filter(v: u8) -> FilterMode {
    match v % 4 {
        0 => FilterMode::None,
        1 => FilterMode::Linear,
        2 => FilterMode::Bilinear,
        _ => FilterMode::Box,
    }
}

fuzz_target!(|input: FuzzScale| {
    let sw = even_dim(input.src_width);
    let sh = even_dim(input.src_height);
    let dw = even_dim(input.dst_width);
    let dh = even_dim(input.dst_height);
    let src_size = ImageSize::new(sw, sh);
    let dst_size = ImageSize::new(dw, dh);
    let filter = to_filter(input.filter);

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

    // I420 スケール
    {
        let y = take(&mut pool, sw * sh);
        let u = take(&mut pool, (sw / 2) * (sh / 2));
        let v = take(&mut pool, (sw / 2) * (sh / 2));
        let src = PlanarImage {
            y: &y, y_stride: sw,
            u: &u, u_stride: sw / 2,
            v: &v, v_stride: sw / 2,
        };
        let mut dst_y = vec![0u8; dw * dh];
        let mut dst_u = vec![0u8; (dw / 2) * (dh / 2)];
        let mut dst_v = vec![0u8; (dw / 2) * (dh / 2)];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: dw,
            u: &mut dst_u, u_stride: dw / 2,
            v: &mut dst_v, v_stride: dw / 2,
        };
        let _ = i420_scale(&src, src_size, &mut dst, dst_size, filter);
    }

    // NV12 スケール
    {
        let y = take(&mut pool, sw * sh);
        let chroma = take(&mut pool, sw * (sh / 2));
        let src = BiplanarImage { y: &y, y_stride: sw, chroma: &chroma, chroma_stride: sw };
        let mut dst_y = vec![0u8; dw * dh];
        let mut dst_chroma = vec![0u8; dw * (dh / 2)];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y, y_stride: dw,
            chroma: &mut dst_chroma, chroma_stride: dw,
        };
        let _ = nv12_scale(&src, src_size, &mut dst, dst_size, filter);
    }

    // ARGB スケール
    {
        let argb = take(&mut pool, sw * sh * 4);
        let src = PackedImage { data: &argb, stride: sw * 4 };
        let mut dst_argb = vec![0u8; dw * dh * 4];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: dw * 4 };
        let _ = argb_scale(&src, src_size, &mut dst, dst_size, filter);
    }

    // 単一プレーンスケール
    {
        let plane = take(&mut pool, sw * sh);
        let mut dst_plane = vec![0u8; dw * dh];
        let _ = scale_plane(&plane, sw, src_size, &mut dst_plane, dw, dst_size, filter);
    }
});
