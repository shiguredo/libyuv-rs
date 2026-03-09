//! convert モジュールの PBT

use pbt::*;
use proptest::prelude::*;
use shiguredo_libyuv::*;

proptest! {
    /// ARGB → ABGR → ARGB の往復で完全一致（チャンネル入替はロスレス）
    #[test]
    fn roundtrip_argb_abgr(
        ((width, height), argb) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_packed(w, h, 4)))
    ) {
        let size = ImageSize::new(width, height);

        let mut abgr_buf = vec![0u8; width * height * 4];
        let src = ArgbImage { data: &argb, stride: width * 4 };
        let mut dst = AbgrImageMut { data: &mut abgr_buf, stride: width * 4 };
        argb_to_abgr(&src, &mut dst, size).unwrap();

        let mut argb2 = vec![0u8; width * height * 4];
        let src2 = AbgrImage { data: &abgr_buf, stride: width * 4 };
        let mut dst2 = ArgbImageMut { data: &mut argb2, stride: width * 4 };
        abgr_to_argb(&src2, &mut dst2, size).unwrap();

        prop_assert_eq!(&argb, &argb2);
    }

    /// NV12 → I420 → NV12 の往復で完全一致（ロスレス）
    #[test]
    fn roundtrip_nv12_i420(
        ((width, height), (y, chroma)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_biplanar(w, h)))
    ) {
        let size = ImageSize::new(width, height);
        let src = Nv12Image { y: &y, y_stride: width, uv: &chroma, uv_stride: width };

        let mut y2 = vec![0u8; width * height];
        let mut u2 = vec![0u8; (width / 2) * (height / 2)];
        let mut v2 = vec![0u8; (width / 2) * (height / 2)];
        let mut i420_dst = I420ImageMut {
            y: &mut y2, y_stride: width,
            u: &mut u2, u_stride: width / 2,
            v: &mut v2, v_stride: width / 2,
        };
        nv12_to_i420(&src, &mut i420_dst, size).unwrap();

        let i420_src = I420Image {
            y: &y2, y_stride: width,
            u: &u2, u_stride: width / 2,
            v: &v2, v_stride: width / 2,
        };
        let mut y3 = vec![0u8; width * height];
        let mut chroma3 = vec![0u8; width * (height / 2)];
        let mut nv12_dst = Nv12ImageMut {
            y: &mut y3, y_stride: width,
            uv: &mut chroma3, uv_stride: width,
        };
        i420_to_nv12(&i420_src, &mut nv12_dst, size).unwrap();

        prop_assert_eq!(&y, &y3);
        prop_assert_eq!(&chroma, &chroma3);
    }

    /// I420 コピーでランダムデータが完全一致
    #[test]
    fn i420_copy_exact(
        ((width, height), (y, u, v)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_i420(w, h)))
    ) {
        let size = ImageSize::new(width, height);
        let src = I420Image {
            y: &y, y_stride: width,
            u: &u, u_stride: width / 2,
            v: &v, v_stride: width / 2,
        };

        let mut y2 = vec![0u8; width * height];
        let mut u2 = vec![0u8; (width / 2) * (height / 2)];
        let mut v2 = vec![0u8; (width / 2) * (height / 2)];
        let mut dst = I420ImageMut {
            y: &mut y2, y_stride: width,
            u: &mut u2, u_stride: width / 2,
            v: &mut v2, v_stride: width / 2,
        };
        i420_copy(&src, &mut dst, size).unwrap();

        prop_assert_eq!(&y, &y2);
        prop_assert_eq!(&u, &u2);
        prop_assert_eq!(&v, &v2);
    }

    /// ARGB コピーでランダムデータが完全一致
    #[test]
    fn argb_copy_exact(
        ((width, height), argb) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_packed(w, h, 4)))
    ) {
        let size = ImageSize::new(width, height);
        let src = ArgbImage { data: &argb, stride: width * 4 };

        let mut argb2 = vec![0u8; width * height * 4];
        let mut dst = ArgbImageMut { data: &mut argb2, stride: width * 4 };
        argb_copy(&src, &mut dst, size).unwrap();

        prop_assert_eq!(&argb, &argb2);
    }

    /// NV12 コピーでランダムデータが完全一致
    #[test]
    fn nv12_copy_exact(
        ((width, height), (y, chroma)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_biplanar(w, h)))
    ) {
        let size = ImageSize::new(width, height);
        let src = Nv12Image { y: &y, y_stride: width, uv: &chroma, uv_stride: width };

        let mut y2 = vec![0u8; width * height];
        let mut chroma2 = vec![0u8; width * (height / 2)];
        let mut dst = Nv12ImageMut {
            y: &mut y2, y_stride: width,
            uv: &mut chroma2, uv_stride: width,
        };
        nv12_copy(&src, &mut dst, size).unwrap();

        prop_assert_eq!(&y, &y2);
        prop_assert_eq!(&chroma, &chroma2);
    }
}
