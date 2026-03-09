//! compare モジュールの PBT

use pbt::*;
use proptest::prelude::*;
use shiguredo_libyuv::*;

proptest! {
    /// 同一ランダム I420 画像の PSNR は非常に高い値
    #[test]
    fn i420_psnr_identical(
        ((width, height), (y, u, v)) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_i420(w, h)))
    ) {
        let size = ImageSize::new(width, height);
        let src = I420Image {
            y: &y, y_stride: width,
            u: &u, u_stride: width / 2,
            v: &v, v_stride: width / 2,
        };

        let psnr = i420_psnr(&src, &src, size).unwrap();
        prop_assert!(psnr >= 100.0, "PSNR should be very high for identical images, got {}", psnr);
    }

    /// 同一ランダム I420 画像の SSIM は 1.0（最低 18x18 以上）
    #[test]
    fn i420_ssim_identical(
        ((width, height), (y, u, v)) in (9..=32usize, 9..=32usize)
            .prop_map(|(w, h)| (w * 2, h * 2))
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_i420(w, h)))
    ) {
        let size = ImageSize::new(width, height);
        let src = I420Image {
            y: &y, y_stride: width,
            u: &u, u_stride: width / 2,
            v: &v, v_stride: width / 2,
        };

        let ssim = i420_ssim(&src, &src, size).unwrap();
        prop_assert!(!ssim.is_nan(), "SSIM should not be NaN, got {}", ssim);
        prop_assert!((ssim - 1.0).abs() < 1e-6, "SSIM should be 1.0 for identical images, got {}", ssim);
    }

    /// 同一ランダムバッファの二乗誤差は 0
    #[test]
    fn sse_identical(
        ((width, height), plane) in even_size()
            .prop_flat_map(|(w, h)| (Just((w, h)), arb_plane(w, h)))
    ) {
        let size = ImageSize::new(width, height);
        let sse = compute_sum_square_error_plane(&plane, width, &plane, width, size).unwrap();
        prop_assert_eq!(sse, 0);
    }
}
