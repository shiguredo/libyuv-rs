//! 画像品質比較関数

use std::ffi::c_int;

use crate::{Error, I420Image, ImageSize, checked_buf_size, require_c_int, sys};

/// I420 画像間の PSNR（ピーク信号対雑音比）を計算する
///
/// 返り値は dB 単位の PSNR 値。同一画像の場合は `kMaxPsnr` (128.0) を返す。
pub fn i420_psnr(
    src_a: &I420Image<'_>,
    src_b: &I420Image<'_>,
    size: ImageSize,
) -> Result<f64, Error> {
    src_a.validate(size, "I420Psnr")?;
    src_b.validate(size, "I420Psnr")?;

    // ゼロサイズ入力は意味のない値（kMaxPsnr）が正常値として返るためエラーにする
    if size.width == 0 || size.height == 0 {
        return Err(Error::with_reason(
            -1,
            "I420Psnr",
            "width and height must be greater than 0",
        ));
    }

    // SAFETY: require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み。
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
/// width または height が 17 未満では `Err` を返す（U/V プレーンの幅または高さが 8 以下に
/// なり、SSIM のサンプル数が 0 で NaN が発生するため）。
pub fn i420_ssim(
    src_a: &I420Image<'_>,
    src_b: &I420Image<'_>,
    size: ImageSize,
) -> Result<f64, Error> {
    src_a.validate(size, "I420Ssim")?;
    src_b.validate(size, "I420Ssim")?;

    // libyuv の I420Ssim は Y プレーンを元サイズ、U/V プレーンを
    // `(width + 1) >> 1` × `(height + 1) >> 1` に縮小して CalcFrameSsim に渡す
    // (compare.cc)。Y プレーンが 9x9 以上でも width <= 16 または height <= 16 では
    // U/V プレーンの幅または高さが 8 以下になり NaN が発生するため、17x17 未満は
    // Err にする。この前提はバンドル版 libyuv の実装に依存しており、libyuv 更新時に
    // 見直すべき箇所である (NaN の扱いが変更される可能性がある)。
    if size.width <= 16 || size.height <= 16 {
        return Err(Error::with_reason(
            -1,
            "I420Ssim",
            "image must be at least 17x17 for SSIM calculation (U/V planes are downsampled)",
        ));
    }

    // SAFETY: require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み。
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
/// 返り値は dB 単位の PSNR 値。同一画像の場合は `kMaxPsnr` (128.0) を返す。
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

    // ゼロサイズ入力は意味のない値（kMaxPsnr）が正常値として返るためエラーにする
    if size.width == 0 || size.height == 0 {
        return Err(Error::with_reason(
            -1,
            "CalcFramePsnr",
            "width and height must be greater than 0",
        ));
    }

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

    // SAFETY: require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み。
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

    // libyuv の CalcFrameSsim は 8x8 ブロックを走査する (compare.cc)。
    // width <= 8 または height <= 8 では samples == 0 となり除算ゼロ (NaN) が発生する。
    if size.width <= 8 || size.height <= 8 {
        return Err(Error::with_reason(
            -1,
            "CalcFrameSsim",
            "image must be at least 9x9 for SSIM calculation",
        ));
    }

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

    // SAFETY: require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み。
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
        // SAFETY: require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み。
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

    // SAFETY: require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み。
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
///
/// `count` が 0 の場合はエラーを返す。
/// `sse` が 0 かつ `count` が 0 より大きい場合は `kMaxPsnr` (128.0) を返す（C の挙動の踏襲）。
pub fn sum_square_error_to_psnr(sse: u64, count: u64) -> Result<f64, Error> {
    if count == 0 {
        return Err(Error::with_reason(
            -1,
            "SumSquareErrorToPsnr",
            "count must be greater than 0",
        ));
    }
    // SAFETY: SumSquareErrorToPsnr は純粋な数値計算でありポインタ操作を伴わない。
    Ok(unsafe { sys::SumSquareErrorToPsnr(sse, count) })
}

/// ハミング距離を計算する
pub fn compute_hamming_distance(src_a: &[u8], src_b: &[u8]) -> Result<u64, Error> {
    let count = src_a.len().min(src_b.len());
    require_c_int(count, "ComputeHammingDistance", "count exceeds c_int range")?;
    let result =
        // SAFETY: require_c_int と checked_buf_size により全ポインタとサイズの有効性が保証済み。
        unsafe { sys::ComputeHammingDistance(src_a.as_ptr(), src_b.as_ptr(), count as c_int) };
    Ok(result)
}

/// DJB2 ハッシュを計算する
///
/// 空スライスの場合は `seed` をそのまま返す（C の挙動の踏襲）。
pub fn hash_djb2(src: &[u8], seed: u32) -> Result<u32, Error> {
    // SAFETY: libyuv の HashDjb2 は src.as_ptr() から src.len() バイトまでしか読み込まず、純粋な数値計算であり未定義動作を起こさない。
    Ok(unsafe { sys::HashDjb2(src.as_ptr(), src.len() as u64, seed) })
}
