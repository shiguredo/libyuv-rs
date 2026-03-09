//! rotate モジュールの PBT

use pbt::*;
use proptest::prelude::*;
use shiguredo_libyuv::*;

proptest! {
    /// I420 の 180 度回転を 2 回でランダムデータが元に戻る
    #[test]
    fn i420_rotate_180_twice(
        ((width, height), (y, u, v)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_i420(w, h)))
    ) {
        let size = ImageSize::new(width, height);

        // 1 回目
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
            i420_rotate(&src, size, &mut dst, size, RotationMode::Rotate180).unwrap();
        }

        // 2 回目
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
            i420_rotate(&src2, size, &mut dst2, size, RotationMode::Rotate180).unwrap();
        }

        prop_assert_eq!(&y, &y3);
        prop_assert_eq!(&u, &u3);
        prop_assert_eq!(&v, &v3);
    }

    /// I420 の 90 度回転を 4 回でランダムデータが元に戻る
    #[test]
    fn i420_rotate_90_four_times(
        ((width, height), (y, u, v)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_i420(w, h)))
    ) {
        let mut cur_y = y.clone();
        let mut cur_u = u.clone();
        let mut cur_v = v.clone();
        let mut cur_w = width;
        let mut cur_h = height;

        for _ in 0..4 {
            let src_size = ImageSize::new(cur_w, cur_h);
            let dst_w = cur_h;
            let dst_h = cur_w;
            let dst_size = ImageSize::new(dst_w, dst_h);

            let mut next_y = vec![0u8; dst_w * dst_h];
            let mut next_u = vec![0u8; (dst_w / 2) * (dst_h / 2)];
            let mut next_v = vec![0u8; (dst_w / 2) * (dst_h / 2)];

            {
                let src = I420Image {
                    y: &cur_y, y_stride: cur_w,
                    u: &cur_u, u_stride: cur_w / 2,
                    v: &cur_v, v_stride: cur_w / 2,
                };
                let mut dst = I420ImageMut {
                    y: &mut next_y, y_stride: dst_w,
                    u: &mut next_u, u_stride: dst_w / 2,
                    v: &mut next_v, v_stride: dst_w / 2,
                };
                i420_rotate(&src, src_size, &mut dst, dst_size, RotationMode::Rotate90).unwrap();
            }

            cur_y = next_y;
            cur_u = next_u;
            cur_v = next_v;
            cur_w = dst_w;
            cur_h = dst_h;
        }

        prop_assert_eq!(&y, &cur_y);
        prop_assert_eq!(&u, &cur_u);
        prop_assert_eq!(&v, &cur_v);
    }

    /// ARGB の 180 度回転を 2 回でランダムデータが元に戻る
    #[test]
    fn argb_rotate_180_twice(
        ((width, height), argb) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_packed(w, h, 4)))
    ) {
        let size = ImageSize::new(width, height);

        let mut argb2 = vec![0u8; width * height * 4];
        {
            let src = ArgbImage { data: &argb, stride: width * 4 };
            let mut dst = ArgbImageMut { data: &mut argb2, stride: width * 4 };
            argb_rotate(&src, size, &mut dst, size, RotationMode::Rotate180).unwrap();
        }

        let mut argb3 = vec![0u8; width * height * 4];
        {
            let src2 = ArgbImage { data: &argb2, stride: width * 4 };
            let mut dst2 = ArgbImageMut { data: &mut argb3, stride: width * 4 };
            argb_rotate(&src2, size, &mut dst2, size, RotationMode::Rotate180).unwrap();
        }

        prop_assert_eq!(&argb, &argb3);
    }

    /// 単一プレーンの 180 度回転を 2 回でランダムデータが元に戻る
    #[test]
    fn rotate_plane_180_twice(
        ((width, height), plane) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_plane(w, h)))
    ) {
        let size = ImageSize::new(width, height);

        let mut buf2 = vec![0u8; width * height];
        rotate_plane(&plane, width, size, &mut buf2, width, size, RotationMode::Rotate180).unwrap();

        let mut buf3 = vec![0u8; width * height];
        rotate_plane(&buf2, width, size, &mut buf3, width, size, RotationMode::Rotate180).unwrap();

        prop_assert_eq!(&plane, &buf3);
    }
}
