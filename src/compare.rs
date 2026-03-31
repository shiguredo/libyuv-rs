//! 画像品質比較関数

use std::ffi::c_int;

use crate::{Error, I420Image, ImageSize, checked_buf_size, require_c_int, sys};

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
    // c_int 範囲チェック
    require_c_int(size.width, "CalcFramePsnr", "width exceeds c_int range")?;
    require_c_int(size.height, "CalcFramePsnr", "height exceeds c_int range")?;
    require_c_int(
        src_a_stride,
        "CalcFramePsnr",
        "stride A exceeds c_int range",
    )?;
    require_c_int(
        src_b_stride,
        "CalcFramePsnr",
        "stride B exceeds c_int range",
    )?;

    // stride が width より小さい場合、libyuv が範囲外アクセスする
    if src_a_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "CalcFramePsnr",
            "stride A smaller than width",
        ));
    }
    if src_b_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "CalcFramePsnr",
            "stride B smaller than width",
        ));
    }

    // オーバーフロー防止のため checked_mul を使用
    let a_size = checked_buf_size(
        src_a_stride,
        size.height,
        "CalcFramePsnr",
        "buffer A size overflow",
    )?;
    if src_a.len() < a_size {
        return Err(Error::with_reason(
            -1,
            "CalcFramePsnr",
            "source A buffer too small",
        ));
    }
    let b_size = checked_buf_size(
        src_b_stride,
        size.height,
        "CalcFramePsnr",
        "buffer B size overflow",
    )?;
    if src_b.len() < b_size {
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
    // c_int 範囲チェック
    require_c_int(size.width, "CalcFrameSsim", "width exceeds c_int range")?;
    require_c_int(size.height, "CalcFrameSsim", "height exceeds c_int range")?;
    require_c_int(
        src_a_stride,
        "CalcFrameSsim",
        "stride A exceeds c_int range",
    )?;
    require_c_int(
        src_b_stride,
        "CalcFrameSsim",
        "stride B exceeds c_int range",
    )?;

    // stride が width より小さい場合、libyuv が範囲外アクセスする
    if src_a_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "CalcFrameSsim",
            "stride A smaller than width",
        ));
    }
    if src_b_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "CalcFrameSsim",
            "stride B smaller than width",
        ));
    }

    // オーバーフロー防止のため checked_mul を使用
    let a_size = checked_buf_size(
        src_a_stride,
        size.height,
        "CalcFrameSsim",
        "buffer A size overflow",
    )?;
    if src_a.len() < a_size {
        return Err(Error::with_reason(
            -1,
            "CalcFrameSsim",
            "source A buffer too small",
        ));
    }
    let b_size = checked_buf_size(
        src_b_stride,
        size.height,
        "CalcFrameSsim",
        "buffer B size overflow",
    )?;
    if src_b.len() < b_size {
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
    // c_int 範囲チェック
    require_c_int(count, "ComputeSumSquareError", "count exceeds c_int range")?;

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
    // c_int 範囲チェック
    require_c_int(
        size.width,
        "ComputeSumSquareErrorPlane",
        "width exceeds c_int range",
    )?;
    require_c_int(
        size.height,
        "ComputeSumSquareErrorPlane",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_a_stride,
        "ComputeSumSquareErrorPlane",
        "stride A exceeds c_int range",
    )?;
    require_c_int(
        src_b_stride,
        "ComputeSumSquareErrorPlane",
        "stride B exceeds c_int range",
    )?;

    // stride が width より小さい場合、libyuv が範囲外アクセスする
    if src_a_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "ComputeSumSquareErrorPlane",
            "stride A smaller than width",
        ));
    }
    if src_b_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "ComputeSumSquareErrorPlane",
            "stride B smaller than width",
        ));
    }

    // オーバーフロー防止のため checked_mul を使用
    let a_size = checked_buf_size(
        src_a_stride,
        size.height,
        "ComputeSumSquareErrorPlane",
        "buffer A size overflow",
    )?;
    if src_a.len() < a_size {
        return Err(Error::with_reason(
            -1,
            "ComputeSumSquareErrorPlane",
            "source A buffer too small",
        ));
    }
    let b_size = checked_buf_size(
        src_b_stride,
        size.height,
        "ComputeSumSquareErrorPlane",
        "buffer B size overflow",
    )?;
    if src_b.len() < b_size {
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
pub fn compute_hamming_distance(src_a: &[u8], src_b: &[u8]) -> Result<u64, Error> {
    let count = src_a.len().min(src_b.len());
    require_c_int(count, "ComputeHammingDistance", "count exceeds c_int range")?;
    let result =
        unsafe { sys::ComputeHammingDistance(src_a.as_ptr(), src_b.as_ptr(), count as c_int) };
    Ok(result)
}

/// DJB2 ハッシュを計算する
pub fn hash_djb2(src: &[u8], seed: u32) -> u32 {
    unsafe { sys::HashDjb2(src.as_ptr(), src.len() as u64, seed) }
}
