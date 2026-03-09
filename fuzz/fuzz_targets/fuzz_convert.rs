//! 色変換関数のパニック安全性を検証する
//!
//! 有効なサイズのバッファを構築し、全変換関数を呼び出す。
//! パニックしないことを確認する。

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use shiguredo_libyuv::*;

#[derive(Arbitrary, Debug)]
struct FuzzImage {
    width: u8,
    height: u8,
    data: Vec<u8>,
}

/// 偶数幅・偶数高さに正規化する（最小 2、最大 256）
fn even_dim(v: u8) -> usize {
    let v = (v as usize).max(1);
    (v * 2).min(256)
}

fuzz_target!(|input: FuzzImage| {
    let width = even_dim(input.width);
    let height = even_dim(input.height);
    let size = ImageSize::new(width, height);

    let y_len = width * height;
    let uv_len = (width / 2) * (height / 2);
    let chroma_len = width * (height / 2);
    let argb_len = width * height * 4;
    let rgb_len = width * height * 3;

    // 入力データからバッファを生成（足りない分は 0 埋め）
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

    let y = take(&mut pool, y_len);
    let u = take(&mut pool, uv_len);
    let v = take(&mut pool, uv_len);
    let chroma = take(&mut pool, chroma_len);
    let argb = take(&mut pool, argb_len);

    // I420 → ARGB / ABGR / RGB24 / NV12 / NV21 / I422 / I444
    {
        let src = PlanarImage {
            y: &y, y_stride: width,
            u: &u, u_stride: width / 2,
            v: &v, v_stride: width / 2,
        };
        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = i420_to_argb(&src, &mut dst, size);

        let mut dst_abgr = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_abgr, stride: width * 4 };
        let _ = i420_to_abgr(&src, &mut dst, size);

        let mut dst_rgb = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut dst_rgb, stride: width * 3 };
        let _ = i420_to_rgb24(&src, &mut dst, size);

        let mut dst_y = vec![0u8; y_len];
        let mut dst_chroma = vec![0u8; chroma_len];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y, y_stride: width,
            chroma: &mut dst_chroma, chroma_stride: width,
        };
        let _ = i420_to_nv12(&src, &mut dst, size);
        let _ = i420_to_nv21(&src, &mut dst, size);

        let mut dst_y2 = vec![0u8; y_len];
        let mut dst_u2 = vec![0u8; (width / 2) * height];
        let mut dst_v2 = vec![0u8; (width / 2) * height];
        let mut dst = PlanarImageMut {
            y: &mut dst_y2, y_stride: width,
            u: &mut dst_u2, u_stride: width / 2,
            v: &mut dst_v2, v_stride: width / 2,
        };
        let _ = i420_to_i422(&src, &mut dst, size);

        let mut dst_y3 = vec![0u8; y_len];
        let mut dst_u3 = vec![0u8; width * height];
        let mut dst_v3 = vec![0u8; width * height];
        let mut dst = PlanarImageMut {
            y: &mut dst_y3, y_stride: width,
            u: &mut dst_u3, u_stride: width,
            v: &mut dst_v3, v_stride: width,
        };
        let _ = i420_to_i444(&src, &mut dst, size);

        // I420 コピー
        let mut dst_y4 = vec![0u8; y_len];
        let mut dst_u4 = vec![0u8; uv_len];
        let mut dst_v4 = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y4, y_stride: width,
            u: &mut dst_u4, u_stride: width / 2,
            v: &mut dst_v4, v_stride: width / 2,
        };
        let _ = i420_copy(&src, &mut dst, size);
    }

    // ARGB → I420 / ABGR / RGB24 / NV12 / NV21 / I422 / I444
    {
        let src = PackedImage { data: &argb, stride: width * 4 };

        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = argb_to_i420(&src, &mut dst, size);

        let mut dst_abgr = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_abgr, stride: width * 4 };
        let _ = argb_to_abgr(&src, &mut dst, size);

        let mut dst_rgb = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut dst_rgb, stride: width * 3 };
        let _ = argb_to_rgb24(&src, &mut dst, size);

        let mut dst_y2 = vec![0u8; y_len];
        let mut dst_chroma = vec![0u8; chroma_len];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y2, y_stride: width,
            chroma: &mut dst_chroma, chroma_stride: width,
        };
        let _ = argb_to_nv12(&src, &mut dst, size);
        let _ = argb_to_nv21(&src, &mut dst, size);

        let mut dst_y3 = vec![0u8; y_len];
        let mut dst_u3 = vec![0u8; (width / 2) * height];
        let mut dst_v3 = vec![0u8; (width / 2) * height];
        let mut dst = PlanarImageMut {
            y: &mut dst_y3, y_stride: width,
            u: &mut dst_u3, u_stride: width / 2,
            v: &mut dst_v3, v_stride: width / 2,
        };
        let _ = argb_to_i422(&src, &mut dst, size);

        let mut dst_y4 = vec![0u8; y_len];
        let mut dst_u4 = vec![0u8; width * height];
        let mut dst_v4 = vec![0u8; width * height];
        let mut dst = PlanarImageMut {
            y: &mut dst_y4, y_stride: width,
            u: &mut dst_u4, u_stride: width,
            v: &mut dst_v4, v_stride: width,
        };
        let _ = argb_to_i444(&src, &mut dst, size);

        // ARGB コピー
        let mut dst_argb2 = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb2, stride: width * 4 };
        let _ = argb_copy(&src, &mut dst, size);
    }

    // ABGR → I420 / ARGB
    {
        let src = PackedImage { data: &argb, stride: width * 4 };

        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = abgr_to_i420(&src, &mut dst, size);

        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = abgr_to_argb(&src, &mut dst, size);
    }

    // RGB24 → I420 / ARGB
    {
        let rgb = take(&mut pool.clone(), rgb_len);
        let src = PackedImage { data: &rgb, stride: width * 3 };

        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = rgb24_to_i420(&src, &mut dst, size);

        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = rgb24_to_argb(&src, &mut dst, size);
    }

    // NV12 → I420 / ARGB / ABGR / RGB24
    {
        let src = BiplanarImage { y: &y, y_stride: width, chroma: &chroma, chroma_stride: width };

        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = nv12_to_i420(&src, &mut dst, size);

        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = nv12_to_argb(&src, &mut dst, size);

        let mut dst_abgr = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_abgr, stride: width * 4 };
        let _ = nv12_to_abgr(&src, &mut dst, size);

        let mut dst_rgb = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut dst_rgb, stride: width * 3 };
        let _ = nv12_to_rgb24(&src, &mut dst, size);

        // NV12 コピー
        let mut dst_y2 = vec![0u8; y_len];
        let mut dst_chroma = vec![0u8; chroma_len];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y2, y_stride: width,
            chroma: &mut dst_chroma, chroma_stride: width,
        };
        let _ = nv12_copy(&src, &mut dst, size);
    }

    // NV21 → I420 / ARGB / ABGR / RGB24 / NV12
    {
        let src = BiplanarImage { y: &y, y_stride: width, chroma: &chroma, chroma_stride: width };

        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = nv21_to_i420(&src, &mut dst, size);

        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = nv21_to_argb(&src, &mut dst, size);

        let mut dst_abgr = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_abgr, stride: width * 4 };
        let _ = nv21_to_abgr(&src, &mut dst, size);

        let mut dst_rgb = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut dst_rgb, stride: width * 3 };
        let _ = nv21_to_rgb24(&src, &mut dst, size);

        let mut dst_y2 = vec![0u8; y_len];
        let mut dst_chroma = vec![0u8; chroma_len];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y2, y_stride: width,
            chroma: &mut dst_chroma, chroma_stride: width,
        };
        let _ = nv21_to_nv12(&src, &mut dst, size);
    }

    // I422 → I420 / I444 / ARGB / ABGR / RGB24 / NV21
    {
        let u422 = take(&mut pool.clone(), (width / 2) * height);
        let v422 = take(&mut pool.clone(), (width / 2) * height);
        let src = PlanarImage {
            y: &y, y_stride: width,
            u: &u422, u_stride: width / 2,
            v: &v422, v_stride: width / 2,
        };

        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = i422_to_i420(&src, &mut dst, size);

        let mut dst_y2 = vec![0u8; y_len];
        let mut dst_u2 = vec![0u8; width * height];
        let mut dst_v2 = vec![0u8; width * height];
        let mut dst = PlanarImageMut {
            y: &mut dst_y2, y_stride: width,
            u: &mut dst_u2, u_stride: width,
            v: &mut dst_v2, v_stride: width,
        };
        let _ = i422_to_i444(&src, &mut dst, size);

        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = i422_to_argb(&src, &mut dst, size);

        let mut dst_abgr = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_abgr, stride: width * 4 };
        let _ = i422_to_abgr(&src, &mut dst, size);

        let mut dst_rgb = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut dst_rgb, stride: width * 3 };
        let _ = i422_to_rgb24(&src, &mut dst, size);

        let mut dst_y3 = vec![0u8; y_len];
        let mut dst_chroma = vec![0u8; chroma_len];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y3, y_stride: width,
            chroma: &mut dst_chroma, chroma_stride: width,
        };
        let _ = i422_to_nv21(&src, &mut dst, size);
    }

    // I444 → I420 / ARGB / ABGR / RGB24 / NV12 / NV21
    {
        let u444 = take(&mut pool.clone(), width * height);
        let v444 = take(&mut pool.clone(), width * height);
        let src = PlanarImage {
            y: &y, y_stride: width,
            u: &u444, u_stride: width,
            v: &v444, v_stride: width,
        };

        let mut dst_y = vec![0u8; y_len];
        let mut dst_u = vec![0u8; uv_len];
        let mut dst_v = vec![0u8; uv_len];
        let mut dst = PlanarImageMut {
            y: &mut dst_y, y_stride: width,
            u: &mut dst_u, u_stride: width / 2,
            v: &mut dst_v, v_stride: width / 2,
        };
        let _ = i444_to_i420(&src, &mut dst, size);

        let mut dst_argb = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_argb, stride: width * 4 };
        let _ = i444_to_argb(&src, &mut dst, size);

        let mut dst_abgr = vec![0u8; argb_len];
        let mut dst = PackedImageMut { data: &mut dst_abgr, stride: width * 4 };
        let _ = i444_to_abgr(&src, &mut dst, size);

        let mut dst_rgb = vec![0u8; rgb_len];
        let mut dst = PackedImageMut { data: &mut dst_rgb, stride: width * 3 };
        let _ = i444_to_rgb24(&src, &mut dst, size);

        let mut dst_y2 = vec![0u8; y_len];
        let mut dst_chroma = vec![0u8; chroma_len];
        let mut dst = BiplanarImageMut {
            y: &mut dst_y2, y_stride: width,
            chroma: &mut dst_chroma, chroma_stride: width,
        };
        let _ = i444_to_nv12(&src, &mut dst, size);
        let _ = i444_to_nv21(&src, &mut dst, size);
    }
});
