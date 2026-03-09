//! 画像品質比較関数

use std::ffi::c_int;

use crate::{Error, I420Image, ImageSize, sys};

/// I420 画像間の PSNR（ピーク信号対雑音比）を計算する
///
/// 返り値は dB 単位の PSNR 値。同一画像の場合は `f64::INFINITY` を返す。
pub fn i420_psnr(
    src_a: &I420Image<'_>,
    src_b: &I420Image<'_>,
    size: ImageSize,
) -> Result<f64, Error> {
    src_a.validate(size, "I420Psnr")?;
    src_b.validate(size, "I420Psnr")?;

    let result = unsafe {
        sys::I420Psnr(
            src_a.y.as_ptr(),
            src_a.y_stride as c_int,
            src_a.u.as_ptr(),
            src_a.u_stride as c_int,
            src_a.v.as_ptr(),
            src_a.v_stride as c_int,
            src_b.y.as_ptr(),
            src_b.y_stride as c_int,
            src_b.u.as_ptr(),
            src_b.u_stride as c_int,
            src_b.v.as_ptr(),
            src_b.v_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(result)
}

/// I420 画像間の SSIM（構造的類似性指標）を計算する
///
/// 返り値は 0.0 から 1.0 の範囲で、1.0 が完全に同一。
pub fn i420_ssim(
    src_a: &I420Image<'_>,
    src_b: &I420Image<'_>,
    size: ImageSize,
) -> Result<f64, Error> {
    src_a.validate(size, "I420Ssim")?;
    src_b.validate(size, "I420Ssim")?;

    let result = unsafe {
        sys::I420Ssim(
            src_a.y.as_ptr(),
            src_a.y_stride as c_int,
            src_a.u.as_ptr(),
            src_a.u_stride as c_int,
            src_a.v.as_ptr(),
            src_a.v_stride as c_int,
            src_b.y.as_ptr(),
            src_b.y_stride as c_int,
            src_b.u.as_ptr(),
            src_b.u_stride as c_int,
            src_b.v.as_ptr(),
            src_b.v_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(result)
}

/// 単一プレーンの PSNR（ピーク信号対雑音比）を計算する
///
/// ARGB 等のパックドフォーマットやグレースケール画像の品質比較に使用する。
/// 返り値は dB 単位の PSNR 値。同一画像の場合は `f64::INFINITY` を返す。
pub fn calc_frame_psnr(
    src_a: &[u8],
    src_a_stride: usize,
    src_b: &[u8],
    src_b_stride: usize,
    size: ImageSize,
) -> Result<f64, Error> {
    if src_a.len() < src_a_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "CalcFramePsnr",
            "source A buffer too small",
        ));
    }
    if src_b.len() < src_b_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "CalcFramePsnr",
            "source B buffer too small",
        ));
    }

    let result = unsafe {
        sys::CalcFramePsnr(
            src_a.as_ptr(),
            src_a_stride as c_int,
            src_b.as_ptr(),
            src_b_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(result)
}

/// 単一プレーンの SSIM（構造的類似性指標）を計算する
///
/// ARGB 等のパックドフォーマットやグレースケール画像の品質比較に使用する。
/// 返り値は 0.0 から 1.0 の範囲で、1.0 が完全に同一。
pub fn calc_frame_ssim(
    src_a: &[u8],
    src_a_stride: usize,
    src_b: &[u8],
    src_b_stride: usize,
    size: ImageSize,
) -> Result<f64, Error> {
    if src_a.len() < src_a_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "CalcFrameSsim",
            "source A buffer too small",
        ));
    }
    if src_b.len() < src_b_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "CalcFrameSsim",
            "source B buffer too small",
        ));
    }

    let result = unsafe {
        sys::CalcFrameSsim(
            src_a.as_ptr(),
            src_a_stride as c_int,
            src_b.as_ptr(),
            src_b_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(result)
}

/// 2 つのバッファの二乗誤差の合計を計算する (stride なし)
///
/// stride を考慮せず、連続したバッファ同士を比較する。
/// count バイト分のデータを比較する。
pub fn compute_sum_square_error(src_a: &[u8], src_b: &[u8], count: usize) -> Result<u64, Error> {
    if src_a.len() < count {
        return Err(Error::with_reason(
            -1,
            "ComputeSumSquareError",
            "source A buffer too small",
        ));
    }
    if src_b.len() < count {
        return Err(Error::with_reason(
            -1,
            "ComputeSumSquareError",
            "source B buffer too small",
        ));
    }

    let result =
        unsafe { sys::ComputeSumSquareError(src_a.as_ptr(), src_b.as_ptr(), count as c_int) };

    Ok(result)
}

/// 2 つのバッファの二乗誤差の合計を計算する (stride 付き)
pub fn compute_sum_square_error_plane(
    src_a: &[u8],
    src_a_stride: usize,
    src_b: &[u8],
    src_b_stride: usize,
    size: ImageSize,
) -> Result<u64, Error> {
    if src_a.len() < src_a_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ComputeSumSquareErrorPlane",
            "source A buffer too small",
        ));
    }
    if src_b.len() < src_b_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ComputeSumSquareErrorPlane",
            "source B buffer too small",
        ));
    }

    let result = unsafe {
        sys::ComputeSumSquareErrorPlane(
            src_a.as_ptr(),
            src_a_stride as c_int,
            src_b.as_ptr(),
            src_b_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(result)
}

/// 二乗誤差の合計から PSNR 値を計算する
pub fn sum_square_error_to_psnr(sse: u64, count: u64) -> f64 {
    unsafe { sys::SumSquareErrorToPsnr(sse, count) }
}

/// ハミング距離を計算する
pub fn compute_hamming_distance(src_a: &[u8], src_b: &[u8]) -> u64 {
    let count = src_a.len().min(src_b.len());
    unsafe { sys::ComputeHammingDistance(src_a.as_ptr(), src_b.as_ptr(), count as c_int) }
}

/// DJB2 ハッシュを計算する
pub fn hash_djb2(src: &[u8], seed: u32) -> u32 {
    unsafe { sys::HashDjb2(src.as_ptr(), src.len() as u64, seed) }
}
