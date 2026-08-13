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

// 異常系: i420_ssim の 8x8 で Err が返ること
// calc_frame_ssim 側の 8x8 / 4x4 テスト（0039 で追加）と対になる既存テスト。
// 17x17 未満の要件（width <= 16 || height <= 16）に包含されるが、calc_frame_ssim と
// 対の回帰テストとして残す
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
// calc_frame_ssim 側の 8x8 / 4x4 テストと対になる既存テスト（8x8 テストのコメント参照）
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

// 異常系: i420_ssim の width / height が 16 以下（17x17 未満）で Err が返ること
// Y プレーンが 9x9 以上でも U/V プレーンが 8 以下になるケースを width / height の
// 非対称（17x16 / 16x17）を含めて検証する
#[test]
fn i420_ssim_rejects_under_17x17() {
    // 16x16: Y は 9x9 以上だが U/V が 8x8 になり NaN
    // 17x16: U/V が 9x8 になり NaN（width だけ 17 では足りない）
    // 16x17: U/V が 8x9 になり NaN（height だけ 17 では足りない）
    for (width, height) in [(16usize, 16usize), (17, 16), (16, 17)] {
        let uv_width = width.div_ceil(2);
        let uv_height = height.div_ceil(2);
        let y = vec![128u8; width * height];
        let u = vec![128u8; uv_width * uv_height];
        let v = vec![128u8; uv_width * uv_height];
        let src = I420Image {
            y: &y,
            y_stride: width,
            u: &u,
            u_stride: uv_width,
            v: &v,
            v_stride: uv_width,
        };
        let size = ImageSize::new(width, height);

        let result = i420_ssim(&src, &src, size);
        let err = result.expect_err("17x17 未満では Err が返るべき");
        assert!(
            err.to_string().contains("image must be at least 17x17"),
            "{width}x{height} では 17x17 要件の reason が返るべき: {}",
            err
        );
    }
}

// 正常系: i420_ssim の 17x17 で Ok が返り、NaN でないこと
#[test]
fn i420_ssim_accepts_17x17() {
    // 17x17: Y が 17x17、U/V が 9x9 になり、全プレーンで samples > 0 になる
    let y = vec![128u8; 17 * 17];
    let u = vec![128u8; 9 * 9];
    let v = vec![128u8; 9 * 9];
    let src = I420Image {
        y: &y,
        y_stride: 17,
        u: &u,
        u_stride: 9,
        v: &v,
        v_stride: 9,
    };
    let size = ImageSize::new(17, 17);

    let result = i420_ssim(&src, &src, size);
    let ssim = result.expect("17x17 では Ok が返るべき");
    // 同一バッファで全プレーンが完全一致するため SSIM は 1.0 になる
    // （is_finite() で NaN / ±Inf を排除し、プラットフォーム差を吸収するため
    // prop_compare と同じ 1e-6 の許容で 1.0 を確認する）
    assert!(
        ssim.is_finite() && (ssim - 1.0).abs() < 1e-6,
        "NaN でなく 1.0 が返るべき: {}",
        ssim
    );
}
