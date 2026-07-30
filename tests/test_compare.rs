//! 画像品質比較 API の単体テスト
//!
//! `sum_square_error_to_psnr` / `calc_frame_psnr` / `i420_psnr` / `hash_djb2` の
//! エラーパス・境界値を検証する。

use shiguredo_libyuv::{
    I420Image, ImageSize, calc_frame_psnr, calc_frame_ssim, hash_djb2, i420_psnr, i420_ssim,
    sum_square_error_to_psnr,
};

// 異常系: sum_square_error_to_psnr の count == 0 で Err が返ること
#[test]
fn sum_square_error_to_psnr_count_zero() {
    let result = sum_square_error_to_psnr(100, 0);
    assert!(result.is_err());
}

// 正常系: sum_square_error_to_psnr の sse == 0, count > 0 で Ok(128.0) が返ること
// （C の kMaxPsnr 挙動の踏襲）
#[test]
fn sum_square_error_to_psnr_sse_zero() {
    let result = sum_square_error_to_psnr(0, 100);
    assert!(result.is_ok());
    let psnr = result.expect("sse == 0, count > 0 であれば Ok が返るはず");
    assert!((psnr - 128.0).abs() < f64::EPSILON);
}

// 異常系: calc_frame_psnr の width == 0 で Err が返ること
#[test]
fn calc_frame_psnr_width_zero() {
    let src_a = vec![0u8; 16];
    let src_b = vec![0u8; 16];
    let size = ImageSize::new(0, 4);

    let result = calc_frame_psnr(&src_a, 4, &src_b, 4, size);
    assert!(result.is_err());
}

// 異常系: calc_frame_psnr の height == 0 で Err が返ること
#[test]
fn calc_frame_psnr_height_zero() {
    let src_a = vec![0u8; 16];
    let src_b = vec![0u8; 16];
    let size = ImageSize::new(4, 0);

    let result = calc_frame_psnr(&src_a, 4, &src_b, 4, size);
    assert!(result.is_err());
}

// 異常系: i420_psnr の width == 0 で Err が返ること
#[test]
fn i420_psnr_width_zero() {
    let y = vec![0u8; 16];
    let u = vec![0u8; 4];
    let v = vec![0u8; 4];
    let src = I420Image {
        y: &y,
        y_stride: 4,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let size = ImageSize::new(0, 4);

    let result = i420_psnr(&src, &src, size);
    assert!(result.is_err());
}

// 異常系: i420_psnr の height == 0 で Err が返ること
#[test]
fn i420_psnr_height_zero() {
    let y = vec![0u8; 16];
    let u = vec![0u8; 4];
    let v = vec![0u8; 4];
    let src = I420Image {
        y: &y,
        y_stride: 4,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let size = ImageSize::new(4, 0);

    let result = i420_psnr(&src, &src, size);
    assert!(result.is_err());
}

// 正常系: hash_djb2 の空スライスで Ok(seed) が返ること（C の挙動の踏襲）
#[test]
fn hash_djb2_empty_slice() {
    let result = hash_djb2(&[], 42);
    assert!(result.is_ok());
    assert_eq!(result.expect("空スライスでは Ok(seed) が返るはず"), 42);
}

// 正常系: hash_djb2 が非空データで Ok を返すこと
#[test]
fn hash_djb2_non_empty() {
    let result = hash_djb2(&[1, 2, 3, 4], 0);
    assert!(result.is_ok());
}

// 正常系: calc_frame_psnr が同一バッファで Ok(128.0) を返すこと
// （同一データは SSE == 0 となり、C の kMaxPsnr 挙動で 128.0 が返る）
#[test]
fn calc_frame_psnr_identical_buffers() {
    let src = vec![128u8; 64];
    let size = ImageSize::new(8, 8);

    let result = calc_frame_psnr(&src, 8, &src, 8, size);
    let psnr = result.expect("同一バッファであれば Ok が返るはず");
    assert!((psnr - 128.0).abs() < f64::EPSILON);
}

// 異常系: calc_frame_ssim の 8x8（境界値）で Err が返ること
// libyuv の SSIM は 8x8 ブロック走査のため 9x9 未満では samples == 0 となり NaN になる
#[test]
fn calc_frame_ssim_rejects_8x8() {
    let src = vec![128u8; 64];
    let size = ImageSize::new(8, 8);

    let result = calc_frame_ssim(&src, 8, &src, 8, size);
    assert!(result.is_err(), "8x8 では Err が返るべき");
}

// 異常系: calc_frame_ssim の 4x4 で Err が返ること
#[test]
fn calc_frame_ssim_rejects_4x4() {
    let src = vec![128u8; 16];
    let size = ImageSize::new(4, 4);

    let result = calc_frame_ssim(&src, 4, &src, 4, size);
    assert!(result.is_err(), "4x4 では Err が返るべき");
}

// 異常系: i420_ssim の 8x8（境界値）で Err が返ること
#[test]
fn i420_ssim_rejects_8x8() {
    let y = vec![128u8; 64];
    let u = vec![128u8; 16];
    let v = vec![128u8; 16];
    let src = I420Image {
        y: &y,
        y_stride: 8,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let size = ImageSize::new(8, 8);

    let result = i420_ssim(&src, &src, size);
    assert!(result.is_err(), "8x8 では Err が返るべき");
}

// 異常系: i420_ssim の 4x4 で Err が返ること
#[test]
fn i420_ssim_rejects_4x4() {
    let y = vec![128u8; 16];
    let u = vec![128u8; 4];
    let v = vec![128u8; 4];
    let src = I420Image {
        y: &y,
        y_stride: 4,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let size = ImageSize::new(4, 4);

    let result = i420_ssim(&src, &src, size);
    assert!(result.is_err(), "4x4 では Err が返るべき");
}
