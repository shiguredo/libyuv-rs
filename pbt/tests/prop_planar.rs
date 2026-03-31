//! planar モジュールの PBT

use pbt::*;
use proptest::prelude::*;
use shiguredo_libyuv::*;

proptest! {
    /// copy_plane でランダムデータが完全一致
    #[test]
    fn copy_plane_exact(
        ((width, height), plane) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_plane(w, h)))
    ) {
        let size = ImageSize::new(width, height);
        let mut dst = vec![0u8; width * height];
        copy_plane(&plane, width, &mut dst, width, size).unwrap();

        prop_assert_eq!(&plane, &dst);
    }

    /// split_uv_plane → merge_uv_plane でランダムデータが元に戻る
    #[test]
    fn roundtrip_split_merge_uv(
        ((width, height), uv) in even_size()
            .prop_flat_map(|(w, h)| {
                let uv_size = w * 2 * h;
                (Just((w, h)), proptest::collection::vec(0..=255u8, uv_size))
            })
    ) {
        let size = ImageSize::new(width, height);

        let mut u = vec![0u8; width * height];
        let mut v = vec![0u8; width * height];
        split_uv_plane(&uv, width * 2, &mut u, width, &mut v, width, size).unwrap();

        let mut uv2 = vec![0u8; width * 2 * height];
        merge_uv_plane(&u, width, &v, width, &mut uv2, width * 2, size).unwrap();

        prop_assert_eq!(&uv, &uv2);
    }

    /// split_rgb_plane → merge_rgb_plane でランダムデータが元に戻る
    #[test]
    fn roundtrip_split_merge_rgb(
        ((width, height), rgb) in aligned32_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_packed(w, h, 3)))
    ) {
        let size = ImageSize::new(width, height);
        let src = Rgb24Image { data: &rgb, stride: width * 3 };

        let mut r = vec![0u8; width * height];
        let mut g = vec![0u8; width * height];
        let mut b = vec![0u8; width * height];
        split_rgb_plane(&src, &mut r, width, &mut g, width, &mut b, width, size).unwrap();

        let mut rgb2 = vec![0u8; width * height * 3];
        let mut dst = Rgb24ImageMut { data: &mut rgb2, stride: width * 3 };
        merge_rgb_plane(&r, width, &g, width, &b, width, &mut dst, size).unwrap();

        prop_assert_eq!(&rgb, &rgb2);
    }

    /// I420 ミラーを 2 回でランダムデータが元に戻る
    #[test]
    fn i420_mirror_twice(
        ((width, height), (y, u, v)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_i420(w, h)))
    ) {
        let size = ImageSize::new(width, height);

        let mut y2 = vec![0u8; width * height];
        let mut u2 = vec![0u8; (width / 2) * (height / 2)];
        let mut v2 = vec![0u8; (width / 2) * (height / 2)];
        {
            let src = I420Image {
                y: &y, y_stride: width,
                u: &u, u_stride: width / 2,
                v: &v, v_stride: width / 2,
            };
            let mut dst = I420ImageMut {
                y: &mut y2, y_stride: width,
                u: &mut u2, u_stride: width / 2,
                v: &mut v2, v_stride: width / 2,
            };
            i420_mirror(&src, &mut dst, size).unwrap();
        }

        let mut y3 = vec![0u8; width * height];
        let mut u3 = vec![0u8; (width / 2) * (height / 2)];
        let mut v3 = vec![0u8; (width / 2) * (height / 2)];
        {
            let src2 = I420Image {
                y: &y2, y_stride: width,
                u: &u2, u_stride: width / 2,
                v: &v2, v_stride: width / 2,
            };
            let mut dst2 = I420ImageMut {
                y: &mut y3, y_stride: width,
                u: &mut u3, u_stride: width / 2,
                v: &mut v3, v_stride: width / 2,
            };
            i420_mirror(&src2, &mut dst2, size).unwrap();
        }

        prop_assert_eq!(&y, &y3);
        prop_assert_eq!(&u, &u3);
        prop_assert_eq!(&v, &v3);
    }

    /// ARGB ミラーを 2 回でランダムデータが元に戻る
    #[test]
    fn argb_mirror_twice(
        ((width, height), argb) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_packed(w, h, 4)))
    ) {
        let size = ImageSize::new(width, height);

        let mut argb2 = vec![0u8; width * height * 4];
        {
            let src = ArgbImage { data: &argb, stride: width * 4 };
            let mut dst = ArgbImageMut { data: &mut argb2, stride: width * 4 };
            argb_mirror(&src, &mut dst, size).unwrap();
        }

        let mut argb3 = vec![0u8; width * height * 4];
        {
            let src2 = ArgbImage { data: &argb2, stride: width * 4 };
            let mut dst2 = ArgbImageMut { data: &mut argb3, stride: width * 4 };
            argb_mirror(&src2, &mut dst2, size).unwrap();
        }

        prop_assert_eq!(&argb, &argb3);
    }

    /// NV12 ミラーを 2 回でランダムデータが元に戻る
    #[test]
    fn nv12_mirror_twice(
        ((width, height), (y, chroma)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_biplanar(w, h)))
    ) {
        let size = ImageSize::new(width, height);

        let mut y2 = vec![0u8; width * height];
        let mut chroma2 = vec![0u8; width * (height / 2)];
        {
            let src = Nv12Image { y: &y, y_stride: width, uv: &chroma, uv_stride: width };
            let mut dst = Nv12ImageMut {
                y: &mut y2, y_stride: width,
                uv: &mut chroma2, uv_stride: width,
            };
            nv12_mirror(&src, &mut dst, size).unwrap();
        }

        let mut y3 = vec![0u8; width * height];
        let mut chroma3 = vec![0u8; width * (height / 2)];
        {
            let src2 = Nv12Image { y: &y2, y_stride: width, uv: &chroma2, uv_stride: width };
            let mut dst2 = Nv12ImageMut {
                y: &mut y3, y_stride: width,
                uv: &mut chroma3, uv_stride: width,
            };
            nv12_mirror(&src2, &mut dst2, size).unwrap();
        }

        prop_assert_eq!(&y, &y3);
        prop_assert_eq!(&chroma, &chroma3);
    }

    /// RGB24 ミラーを 2 回でランダムデータが元に戻る
    #[test]
    fn rgb24_mirror_twice(
        ((width, height), rgb) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_packed(w, h, 3)))
    ) {
        let size = ImageSize::new(width, height);

        let mut rgb2 = vec![0u8; width * height * 3];
        {
            let src = Rgb24Image { data: &rgb, stride: width * 3 };
            let mut dst = Rgb24ImageMut { data: &mut rgb2, stride: width * 3 };
            rgb24_mirror(&src, &mut dst, size).unwrap();
        }

        let mut rgb3 = vec![0u8; width * height * 3];
        {
            let src2 = Rgb24Image { data: &rgb2, stride: width * 3 };
            let mut dst2 = Rgb24ImageMut { data: &mut rgb3, stride: width * 3 };
            rgb24_mirror(&src2, &mut dst2, size).unwrap();
        }

        prop_assert_eq!(&rgb, &rgb3);
    }

    /// mirror_plane を 2 回でランダムデータが元に戻る
    #[test]
    fn mirror_plane_twice(
        ((width, height), plane) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_plane(w, h)))
    ) {
        let size = ImageSize::new(width, height);

        let mut buf2 = vec![0u8; width * height];
        mirror_plane(&plane, width, &mut buf2, width, size);

        let mut buf3 = vec![0u8; width * height];
        mirror_plane(&buf2, width, &mut buf3, width, size);

        prop_assert_eq!(&plane, &buf3);
    }
}
