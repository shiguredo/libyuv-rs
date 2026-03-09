//! scale モジュールの PBT

use pbt::*;
use proptest::prelude::*;
use shiguredo_libyuv::*;

proptest! {
    /// I420 等倍スケールでランダムデータが完全一致
    #[test]
    fn i420_scale_identity(
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
        i420_scale(&src, size, &mut dst, size, FilterMode::None).unwrap();

        prop_assert_eq!(&y, &y2);
        prop_assert_eq!(&u, &u2);
        prop_assert_eq!(&v, &v2);
    }

    /// ARGB 等倍スケールでランダムデータが完全一致
    #[test]
    fn argb_scale_identity(
        ((width, height), argb) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_packed(w, h, 4)))
    ) {
        let size = ImageSize::new(width, height);
        let src = ArgbImage { data: &argb, stride: width * 4 };

        let mut argb2 = vec![0u8; width * height * 4];
        let mut dst = ArgbImageMut { data: &mut argb2, stride: width * 4 };
        argb_scale(&src, size, &mut dst, size, FilterMode::None).unwrap();

        prop_assert_eq!(&argb, &argb2);
    }

    /// NV12 等倍スケールでランダムデータが完全一致
    #[test]
    fn nv12_scale_identity(
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
        nv12_scale(&src, size, &mut dst, size, FilterMode::None).unwrap();

        prop_assert_eq!(&y, &y2);
        prop_assert_eq!(&chroma, &chroma2);
    }

    /// 単一プレーン等倍スケールでランダムデータが完全一致
    #[test]
    fn scale_plane_identity(
        ((width, height), plane) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_plane(w, h)))
    ) {
        let size = ImageSize::new(width, height);

        let mut dst = vec![0u8; width * height];
        scale_plane(&plane, width, size, &mut dst, width, size, FilterMode::None).unwrap();

        prop_assert_eq!(&plane, &dst);
    }
}
