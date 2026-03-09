//! プレーン操作関数のパニック安全性を検証する
//!
//! ミラー、ブレンド、split/merge、attenuate 等の
//! プレーン操作関数を呼び出し、パニックしないことを確認する。

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use shiguredo_libyuv::*;

#[derive(Arbitrary, Debug)]
struct FuzzPlanar {
    width: u8,
    height: u8,
    value: u8,
    interpolation: u8,
    shade: u32,
    rect_x: u8,
    rect_y: u8,
    rect_w: u8,
    rect_h: u8,
    data: Vec<u8>,
}

fn even_dim(v: u8) -> usize {
    let v = (v as usize).max(1);
    (v * 2).min(256)
}

fuzz_target!(|input: FuzzPlanar| {
    let width = even_dim(input.width);
    let height = even_dim(input.height);
    let size = ImageSize::new(width, height);

    let y_len = width * height;
    let uv_len = (width / 2) * (height / 2);
    let chroma_len = width * (height / 2);
    let argb_len = width * height * 4;
    let rgb_len = width * height * 3;

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

    // copy_plane
    {
        let plane = take(&mut pool, y_len);
        let mut dst = vec![0u8; y_len];
        let _ = copy_plane(&plane, width, &mut dst, width, size);
    }

    // set_plane
    {
        let mut dst = vec![0u8; y_len];
        let _ = set_plane(&mut dst, width, size, input.value);
    }

    // split_uv_plane / merge_uv_plane
    {
        let uv = take(&mut pool, width * 2 * height);
        let mut u_dst = vec![0u8; y_len];
        let mut v_dst = vec![0u8; y_len];
        let _ = split_uv_plane(&uv, width * 2, &mut u_dst, width, &mut v_dst, width, size);

        let mut uv2 = vec![0u8; width * 2 * height];
        let _ = merge_uv_plane(&u_dst, width, &v_dst, width, &mut uv2, width * 2, size);
    }

    // swap_uv_plane
    {
        let uv = take(&mut pool, width * 2 * height);
        let mut vu = vec![0u8; width * 2 * height];
        let _ = swap_uv_plane(&uv, width * 2, &mut vu, width * 2, size);
    }

    // split_rgb_plane / merge_rgb_plane
    {
        let rgb = take(&mut pool, rgb_len);
        let src = PackedImage { data: &rgb, stride: width * 3 };
        let mut r = vec![0u8; y_len];
        let mut g = vec![0u8; y_len];
        let mut b = vec![0u8; y_len];
        let _ = split_rgb_plane(&src, &mut r, width, &mut g, width, &mut b, width, size);

        let mut rgb2 = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut rgb2, stride: width * 3 };
        let _ = merge_rgb_plane(&r, width, &g, width, &b, width, &mut dst, size);
    }

    // I420 ミラー
    {
        let y = take(&mut pool, y_len);
        let u = take(&mut pool, uv_len);
        let v = take(&mut pool, uv_len);
        let src = PlanarImage {
            y: &y, y_stride: width,
            u: &u, u_stride: width / 2,
            v: &v, v_stride: width / 2,
        };
        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = i420_mirror(&src, &mut dst, size);
    }

    // NV12 ミラー
    {
        let y = take(&mut pool, y_len);
        let chroma = take(&mut pool, chroma_len);
        let src = BiplanarImage { y: &y, y_stride: width, chroma: &chroma, chroma_stride: width };
        let mut dst_y = vec![0u8; y_len];
        let mut dst_chroma = vec![0u8; chroma_len];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y, y_stride: width,
            chroma: &mut dst_chroma, chroma_stride: width,
        };
        let _ = nv12_mirror(&src, &mut dst, size);
    }

    // ARGB ミラー
    {
        let argb = take(&mut pool, argb_len);
        let src = PackedImage { data: &argb, stride: width * 4 };
        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = argb_mirror(&src, &mut dst, size);
    }

    // RGB24 ミラー
    {
        let rgb = take(&mut pool, rgb_len);
        let src = PackedImage { data: &rgb, stride: width * 3 };
        let mut dst_rgb = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut dst_rgb, stride: width * 3 };
        let _ = rgb24_mirror(&src, &mut dst, size);
    }

    // mirror_plane
    {
        let plane = take(&mut pool, y_len);
        let mut dst = vec![0u8; y_len];
        mirror_plane(&plane, width, &mut dst, width, size);
    }

    // I420 ブレンド
    {
        let y0 = take(&mut pool, y_len);
        let u0 = take(&mut pool, uv_len);
        let v0 = take(&mut pool, uv_len);
        let y1 = take(&mut pool, y_len);
        let u1 = take(&mut pool, uv_len);
        let v1 = take(&mut pool, uv_len);
        let alpha = take(&mut pool, y_len);
        let src0 = PlanarImage {
            y: &y0, y_stride: width,
            u: &u0, u_stride: width / 2,
            v: &v0, v_stride: width / 2,
        };
        let src1 = PlanarImage {
            y: &y1, y_stride: width,
            u: &u1, u_stride: width / 2,
            v: &v1, v_stride: width / 2,
        };
        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = i420_blend(&src0, &src1, &alpha, width, &mut dst, size);
    }

    // ARGB ブレンド
    {
        let argb0 = take(&mut pool, argb_len);
        let argb1 = take(&mut pool, argb_len);
        let src0 = PackedImage { data: &argb0, stride: width * 4 };
        let src1 = PackedImage { data: &argb1, stride: width * 4 };
        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = argb_blend(&src0, &src1, &mut dst, size);
    }

    // blend_plane
    {
        let p0 = take(&mut pool, y_len);
        let p1 = take(&mut pool, y_len);
        let alpha = take(&mut pool, y_len);
        let mut dst = vec![0u8; y_len];
        let _ = blend_plane(
            &p0, width, &p1, width, &alpha, width, &mut dst, width, size,
        );
    }

    // interpolate_plane
    {
        let p0 = take(&mut pool, y_len);
        let p1 = take(&mut pool, y_len);
        let mut dst = vec![0u8; y_len];
        let _ = interpolate_plane(
            &p0, width, &p1, width, &mut dst, width, size, input.interpolation,
        );
    }

    // ARGB attenuate / unattenuate / shade / gray / sepia
    {
        let argb = take(&mut pool, argb_len);
        let src = PackedImage { data: &argb, stride: width * 4 };

        let mut dst1 = vec![0u8; argb_len];
        let mut d = PackedImageMut { data: &mut dst1, stride: width * 4 };
        let _ = argb_attenuate(&src, &mut d, size);

        let mut dst2 = vec![0u8; argb_len];
        let mut d = PackedImageMut { data: &mut dst2, stride: width * 4 };
        let _ = argb_unattenuate(&src, &mut d, size);

        let mut dst3 = vec![0u8; argb_len];
        let mut d = PackedImageMut { data: &mut dst3, stride: width * 4 };
        let _ = argb_shade(&src, &mut d, size, input.shade);

        let mut buf = argb.clone();
        let mut d = PackedImageMut { data: &mut buf, stride: width * 4 };
        let _ = argb_gray(&mut d, size);

        let mut buf2 = argb.clone();
        let mut d = PackedImageMut { data: &mut buf2, stride: width * 4 };
        let _ = argb_sepia(&mut d, size);
    }

    // I420 rect（偶数座標・偶数サイズで画像範囲内に収める）
    {
        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        // 偶数アライメントで画像範囲内に収める
        let rx = (input.rect_x as usize % (width / 2)) * 2;
        let ry = (input.rect_y as usize % (height / 2)) * 2;
        let max_rw = width - rx;
        let max_rh = height - ry;
        if max_rw >= 2 && max_rh >= 2 {
            let rw = ((input.rect_w as usize % max_rw).max(2)) & !1;
            let rh = ((input.rect_h as usize % max_rh).max(2)) & !1;
            if rx + rw <= width && ry + rh <= height {
                let _ = i420_rect(&mut dst, size, rx, ry, rw, rh, 16, 128, 128);
            }
        }
    }

    // ARGB rect（画像範囲内に収める）
    {
        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let rx = input.rect_x as usize % width;
        let ry = input.rect_y as usize % height;
        let max_rw = width - rx;
        let max_rh = height - ry;
        if max_rw >= 1 && max_rh >= 1 {
            let rw = (input.rect_w as usize % max_rw).max(1);
            let rh = (input.rect_h as usize % max_rh).max(1);
            if rx + rw <= width && ry + rh <= height {
                let _ = argb_rect(&mut dst, size, rx, ry, rw, rh, 0xFF00FF00);
            }
        }
    }
});
