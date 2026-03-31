//! 回転関数
#![allow(clippy::too_many_arguments)]

use std::ffi::c_int;

use crate::{
    Android420Image, ArgbImage, ArgbImageMut, Error, I010Image, I010ImageMut, I210Image,
    I210ImageMut, I410Image, I410ImageMut, I420Image, I420ImageMut, I422Image, I422ImageMut,
    I444Image, I444ImageMut, ImageSize, Nv12Image, RotationMode, checked_buf_size, require_c_int,
    sys,
};

/// I420 画像の回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
/// `dst_size` は出力バッファのサイズを指定する。
pub fn i420_rotate(
    src: &I420Image<'_>,
    src_size: ImageSize,
    dst: &mut I420ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "I420Rotate")?;
    dst.validate(dst_size, "I420Rotate")?;

    let result = unsafe {
        sys::I420Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "I420Rotate")
}

/// ARGB 画像の回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
/// `dst_size` は出力バッファのサイズを指定する。
pub fn argb_rotate(
    src: &ArgbImage<'_>,
    src_size: ImageSize,
    dst: &mut ArgbImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "ARGBRotate")?;
    dst.validate(dst_size, "ARGBRotate")?;

    let result = unsafe {
        sys::ARGBRotate(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "ARGBRotate")
}

/// 単一プレーンの回転
///
/// 90 度 / 270 度回転の場合、出力の幅と高さは入力と逆になる。
pub fn rotate_plane(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u8],
    dst_stride: usize,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(src_size.width, "RotatePlane", "width exceeds c_int range")?;
    require_c_int(src_size.height, "RotatePlane", "height exceeds c_int range")?;
    require_c_int(
        src_stride,
        "RotatePlane",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "RotatePlane",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane",
            "source stride smaller than width",
        ));
    }
    if dst_stride < dst_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane",
            "destination stride smaller than width",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "RotatePlane",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane",
            "source buffer too small",
        ));
    }
    let dst_buf = checked_buf_size(
        dst_stride,
        dst_size.height,
        "RotatePlane",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::RotatePlane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "RotatePlane")
}

// ============================================================
// 10bit 回転
// ============================================================

/// I010 (10bit I420) 画像の回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
/// `dst_size` は出力バッファのサイズを指定する。
pub fn i010_rotate(
    src: &I010Image<'_>,
    src_size: ImageSize,
    dst: &mut I010ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "I010Rotate")?;
    dst.validate(dst_size, "I010Rotate")?;

    let result = unsafe {
        sys::I010Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "I010Rotate")
}

/// I210 (10bit I422) 画像の回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
/// `dst_size` は出力バッファのサイズを指定する。
pub fn i210_rotate(
    src: &I210Image<'_>,
    src_size: ImageSize,
    dst: &mut I210ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "I210Rotate")?;
    dst.validate(dst_size, "I210Rotate")?;

    let result = unsafe {
        sys::I210Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "I210Rotate")
}

/// I410 (10bit I444) 画像の回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
/// `dst_size` は出力バッファのサイズを指定する。
pub fn i410_rotate(
    src: &I410Image<'_>,
    src_size: ImageSize,
    dst: &mut I410ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "I410Rotate")?;
    dst.validate(dst_size, "I410Rotate")?;

    let result = unsafe {
        sys::I410Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "I410Rotate")
}

// ============================================================
// Android / NV12 回転
// ============================================================

/// Android420 から I420 への変換と回転
///
/// `pixel_stride_uv` は UV ピクセルストライド（1: planar、2: interleaved）。
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
pub fn android420_to_i420_rotate(
    src: &Android420Image<'_>,
    src_size: ImageSize,
    pixel_stride_uv: usize,
    dst: &mut I420ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "Android420ToI420Rotate")?;
    dst.validate(dst_size, "Android420ToI420Rotate")?;

    // pixel_stride_uv の c_int 範囲チェック
    require_c_int(
        pixel_stride_uv,
        "Android420ToI420Rotate",
        "pixel_stride_uv exceeds c_int range",
    )?;

    let result = unsafe {
        sys::Android420ToI420Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            pixel_stride_uv as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "Android420ToI420Rotate")
}

/// NV12 から I420 への変換と回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
pub fn nv12_to_i420_rotate(
    src: &Nv12Image<'_>,
    src_size: ImageSize,
    dst: &mut I420ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "NV12ToI420Rotate")?;
    dst.validate(dst_size, "NV12ToI420Rotate")?;

    let result = unsafe {
        sys::NV12ToI420Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "NV12ToI420Rotate")
}

// ============================================================
// I422 / I444 回転
// ============================================================

/// I422 画像の回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
/// `dst_size` は出力バッファのサイズを指定する。
pub fn i422_rotate(
    src: &I422Image<'_>,
    src_size: ImageSize,
    dst: &mut I422ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "I422Rotate")?;
    dst.validate(dst_size, "I422Rotate")?;

    let result = unsafe {
        sys::I422Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "I422Rotate")
}

/// I444 画像の回転
///
/// 90 度 / 270 度回転の場合、出力画像の幅と高さは入力と逆になる。
/// `dst_size` は出力バッファのサイズを指定する。
pub fn i444_rotate(
    src: &I444Image<'_>,
    src_size: ImageSize,
    dst: &mut I444ImageMut<'_>,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    src.validate(src_size, "I444Rotate")?;
    dst.validate(dst_size, "I444Rotate")?;

    let result = unsafe {
        sys::I444Rotate(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "I444Rotate")
}

// ============================================================
// 16bit プレーン回転
// ============================================================

/// 16bit 単一プレーンの回転
///
/// 90 度 / 270 度回転の場合、出力の幅と高さは入力と逆になる。
pub fn rotate_plane_16(
    src: &[u16],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u16],
    dst_stride: usize,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "RotatePlane_16",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "RotatePlane_16",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "RotatePlane_16",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "RotatePlane_16",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック（u16 スライスなので stride は要素数）
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane_16",
            "source stride smaller than width",
        ));
    }
    if dst_stride < dst_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane_16",
            "destination stride smaller than width",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "RotatePlane_16",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane_16",
            "source buffer too small",
        ));
    }
    let dst_buf = checked_buf_size(
        dst_stride,
        dst_size.height,
        "RotatePlane_16",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane_16",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::RotatePlane_16(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "RotatePlane_16")
}

// ============================================================
// 固定角度回転（90 / 180 / 270）
// ============================================================

/// プレーンの 90 度回転
///
/// 出力の幅と高さは入力と逆になる。
pub fn rotate_plane_90(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u8],
    dst_stride: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(src_size.width, "RotatePlane90", "width exceeds c_int range")?;
    require_c_int(
        src_size.height,
        "RotatePlane90",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "RotatePlane90",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "RotatePlane90",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane90",
            "source stride smaller than width",
        ));
    }
    // 90 度回転: 出力は幅と高さが入れ替わるので dst_stride >= src_size.height
    if dst_stride < src_size.height {
        return Err(Error::with_reason(
            -1,
            "RotatePlane90",
            "destination stride smaller than height",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "RotatePlane90",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane90",
            "source buffer too small",
        ));
    }
    // 90 度回転: 出力の高さは src_size.width
    let dst_buf = checked_buf_size(
        dst_stride,
        src_size.width,
        "RotatePlane90",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane90",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::RotatePlane90(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}

/// プレーンの 180 度回転
pub fn rotate_plane_180(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u8],
    dst_stride: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "RotatePlane180",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "RotatePlane180",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "RotatePlane180",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "RotatePlane180",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane180",
            "source stride smaller than width",
        ));
    }
    if dst_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane180",
            "destination stride smaller than width",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "RotatePlane180",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane180",
            "source buffer too small",
        ));
    }
    // 180 度回転: 出力は入力と同じサイズ
    let dst_buf = checked_buf_size(
        dst_stride,
        src_size.height,
        "RotatePlane180",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane180",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::RotatePlane180(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}

/// プレーンの 270 度回転
///
/// 出力の幅と高さは入力と逆になる。
pub fn rotate_plane_270(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u8],
    dst_stride: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "RotatePlane270",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "RotatePlane270",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "RotatePlane270",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "RotatePlane270",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "RotatePlane270",
            "source stride smaller than width",
        ));
    }
    // 270 度回転: 出力は幅と高さが入れ替わるので dst_stride >= src_size.height
    if dst_stride < src_size.height {
        return Err(Error::with_reason(
            -1,
            "RotatePlane270",
            "destination stride smaller than height",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "RotatePlane270",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane270",
            "source buffer too small",
        ));
    }
    // 270 度回転: 出力の高さは src_size.width
    let dst_buf = checked_buf_size(
        dst_stride,
        src_size.width,
        "RotatePlane270",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "RotatePlane270",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::RotatePlane270(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}

// ============================================================
// UV 分割回転
// ============================================================

/// インターリーブ UV を分割して回転
///
/// 90 度 / 270 度回転の場合、出力の幅と高さは入力と逆になる。
pub fn split_rotate_uv(
    src_uv: &[u8],
    src_stride_uv: usize,
    src_size: ImageSize,
    dst_u: &mut [u8],
    dst_stride_u: usize,
    dst_v: &mut [u8],
    dst_stride_v: usize,
    dst_size: ImageSize,
    mode: RotationMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(src_size.width, "SplitRotateUV", "width exceeds c_int range")?;
    require_c_int(
        src_size.height,
        "SplitRotateUV",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride_uv,
        "SplitRotateUV",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_u,
        "SplitRotateUV",
        "destination U stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_v,
        "SplitRotateUV",
        "destination V stride exceeds c_int range",
    )?;

    // src は UV インターリーブなので stride >= width * 2
    let src_min_width = src_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "SplitRotateUV", "width * 2 overflow"))?;
    if src_stride_uv < src_min_width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV",
            "source stride smaller than width * 2",
        ));
    }
    // dst_u/v は stride >= width
    if dst_stride_u < dst_size.width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV",
            "destination U stride smaller than width",
        ));
    }
    if dst_stride_v < dst_size.width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV",
            "destination V stride smaller than width",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride_uv,
        src_size.height,
        "SplitRotateUV",
        "source buffer size overflow",
    )?;
    if src_uv.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV",
            "source buffer too small",
        ));
    }
    let dst_u_buf = checked_buf_size(
        dst_stride_u,
        dst_size.height,
        "SplitRotateUV",
        "destination U buffer size overflow",
    )?;
    if dst_u.len() < dst_u_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV",
            "destination U buffer too small",
        ));
    }
    let dst_v_buf = checked_buf_size(
        dst_stride_v,
        dst_size.height,
        "SplitRotateUV",
        "destination V buffer size overflow",
    )?;
    if dst_v.len() < dst_v_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV",
            "destination V buffer too small",
        ));
    }

    let result = unsafe {
        sys::SplitRotateUV(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            dst_u.as_mut_ptr(),
            dst_stride_u as c_int,
            dst_v.as_mut_ptr(),
            dst_stride_v as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            mode.to_sys(),
        )
    };

    Error::check(result, "SplitRotateUV")
}

/// インターリーブ UV を分割して 90 度回転
///
/// 出力の幅と高さは入力と逆になる。
pub fn split_rotate_uv_90(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst_a: &mut [u8],
    dst_stride_a: usize,
    dst_b: &mut [u8],
    dst_stride_b: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "SplitRotateUV90",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "SplitRotateUV90",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "SplitRotateUV90",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_a,
        "SplitRotateUV90",
        "destination A stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_b,
        "SplitRotateUV90",
        "destination B stride exceeds c_int range",
    )?;

    // src は UV インターリーブなので stride >= width * 2
    let src_min_width = src_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "SplitRotateUV90", "width * 2 overflow"))?;
    if src_stride < src_min_width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV90",
            "source stride smaller than width * 2",
        ));
    }
    // 90 度回転: 出力は幅と高さが入れ替わるので dst_stride >= src_size.height
    if dst_stride_a < src_size.height {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV90",
            "destination A stride smaller than height",
        ));
    }
    if dst_stride_b < src_size.height {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV90",
            "destination B stride smaller than height",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "SplitRotateUV90",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV90",
            "source buffer too small",
        ));
    }
    // 90 度回転: 出力の高さは src_size.width
    let dst_a_buf = checked_buf_size(
        dst_stride_a,
        src_size.width,
        "SplitRotateUV90",
        "destination A buffer size overflow",
    )?;
    if dst_a.len() < dst_a_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV90",
            "destination A buffer too small",
        ));
    }
    let dst_b_buf = checked_buf_size(
        dst_stride_b,
        src_size.width,
        "SplitRotateUV90",
        "destination B buffer size overflow",
    )?;
    if dst_b.len() < dst_b_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV90",
            "destination B buffer too small",
        ));
    }

    unsafe {
        sys::SplitRotateUV90(
            src.as_ptr(),
            src_stride as c_int,
            dst_a.as_mut_ptr(),
            dst_stride_a as c_int,
            dst_b.as_mut_ptr(),
            dst_stride_b as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}

/// インターリーブ UV を分割して 180 度回転
pub fn split_rotate_uv_180(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst_a: &mut [u8],
    dst_stride_a: usize,
    dst_b: &mut [u8],
    dst_stride_b: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "SplitRotateUV180",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "SplitRotateUV180",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "SplitRotateUV180",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_a,
        "SplitRotateUV180",
        "destination A stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_b,
        "SplitRotateUV180",
        "destination B stride exceeds c_int range",
    )?;

    // src は UV インターリーブなので stride >= width * 2
    let src_min_width = src_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "SplitRotateUV180", "width * 2 overflow"))?;
    if src_stride < src_min_width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV180",
            "source stride smaller than width * 2",
        ));
    }
    // dst_a/b は stride >= width
    if dst_stride_a < src_size.width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV180",
            "destination A stride smaller than width",
        ));
    }
    if dst_stride_b < src_size.width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV180",
            "destination B stride smaller than width",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "SplitRotateUV180",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV180",
            "source buffer too small",
        ));
    }
    // 180 度回転: 出力は入力と同じサイズ
    let dst_a_buf = checked_buf_size(
        dst_stride_a,
        src_size.height,
        "SplitRotateUV180",
        "destination A buffer size overflow",
    )?;
    if dst_a.len() < dst_a_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV180",
            "destination A buffer too small",
        ));
    }
    let dst_b_buf = checked_buf_size(
        dst_stride_b,
        src_size.height,
        "SplitRotateUV180",
        "destination B buffer size overflow",
    )?;
    if dst_b.len() < dst_b_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV180",
            "destination B buffer too small",
        ));
    }

    unsafe {
        sys::SplitRotateUV180(
            src.as_ptr(),
            src_stride as c_int,
            dst_a.as_mut_ptr(),
            dst_stride_a as c_int,
            dst_b.as_mut_ptr(),
            dst_stride_b as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}

/// インターリーブ UV を分割して 270 度回転
///
/// 出力の幅と高さは入力と逆になる。
pub fn split_rotate_uv_270(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst_a: &mut [u8],
    dst_stride_a: usize,
    dst_b: &mut [u8],
    dst_stride_b: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "SplitRotateUV270",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "SplitRotateUV270",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "SplitRotateUV270",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_a,
        "SplitRotateUV270",
        "destination A stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_b,
        "SplitRotateUV270",
        "destination B stride exceeds c_int range",
    )?;

    // src は UV インターリーブなので stride >= width * 2
    let src_min_width = src_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "SplitRotateUV270", "width * 2 overflow"))?;
    if src_stride < src_min_width {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV270",
            "source stride smaller than width * 2",
        ));
    }
    // 270 度回転: 出力は幅と高さが入れ替わるので dst_stride >= src_size.height
    if dst_stride_a < src_size.height {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV270",
            "destination A stride smaller than height",
        ));
    }
    if dst_stride_b < src_size.height {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV270",
            "destination B stride smaller than height",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "SplitRotateUV270",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV270",
            "source buffer too small",
        ));
    }
    // 270 度回転: 出力の高さは src_size.width
    let dst_a_buf = checked_buf_size(
        dst_stride_a,
        src_size.width,
        "SplitRotateUV270",
        "destination A buffer size overflow",
    )?;
    if dst_a.len() < dst_a_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV270",
            "destination A buffer too small",
        ));
    }
    let dst_b_buf = checked_buf_size(
        dst_stride_b,
        src_size.width,
        "SplitRotateUV270",
        "destination B buffer size overflow",
    )?;
    if dst_b.len() < dst_b_buf {
        return Err(Error::with_reason(
            -1,
            "SplitRotateUV270",
            "destination B buffer too small",
        ));
    }

    unsafe {
        sys::SplitRotateUV270(
            src.as_ptr(),
            src_stride as c_int,
            dst_a.as_mut_ptr(),
            dst_stride_a as c_int,
            dst_b.as_mut_ptr(),
            dst_stride_b as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}

// ============================================================
// 転置
// ============================================================

/// インターリーブ UV を分割して転置
///
/// 出力の幅と高さは入力と逆になる。
pub fn split_transpose_uv(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst_a: &mut [u8],
    dst_stride_a: usize,
    dst_b: &mut [u8],
    dst_stride_b: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "SplitTransposeUV",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "SplitTransposeUV",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "SplitTransposeUV",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_a,
        "SplitTransposeUV",
        "destination A stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_b,
        "SplitTransposeUV",
        "destination B stride exceeds c_int range",
    )?;

    // src は UV インターリーブなので stride >= width * 2
    let src_min_width = src_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "SplitTransposeUV", "width * 2 overflow"))?;
    if src_stride < src_min_width {
        return Err(Error::with_reason(
            -1,
            "SplitTransposeUV",
            "source stride smaller than width * 2",
        ));
    }
    // 転置: 出力は幅と高さが入れ替わるので dst_stride >= src_size.height
    if dst_stride_a < src_size.height {
        return Err(Error::with_reason(
            -1,
            "SplitTransposeUV",
            "destination A stride smaller than height",
        ));
    }
    if dst_stride_b < src_size.height {
        return Err(Error::with_reason(
            -1,
            "SplitTransposeUV",
            "destination B stride smaller than height",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "SplitTransposeUV",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "SplitTransposeUV",
            "source buffer too small",
        ));
    }
    // 転置: 出力の高さは src_size.width
    let dst_a_buf = checked_buf_size(
        dst_stride_a,
        src_size.width,
        "SplitTransposeUV",
        "destination A buffer size overflow",
    )?;
    if dst_a.len() < dst_a_buf {
        return Err(Error::with_reason(
            -1,
            "SplitTransposeUV",
            "destination A buffer too small",
        ));
    }
    let dst_b_buf = checked_buf_size(
        dst_stride_b,
        src_size.width,
        "SplitTransposeUV",
        "destination B buffer size overflow",
    )?;
    if dst_b.len() < dst_b_buf {
        return Err(Error::with_reason(
            -1,
            "SplitTransposeUV",
            "destination B buffer too small",
        ));
    }

    unsafe {
        sys::SplitTransposeUV(
            src.as_ptr(),
            src_stride as c_int,
            dst_a.as_mut_ptr(),
            dst_stride_a as c_int,
            dst_b.as_mut_ptr(),
            dst_stride_b as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}

/// プレーンの転置
///
/// 出力の幅と高さは入力と逆になる。
pub fn transpose_plane(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u8],
    dst_stride: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "TransposePlane",
        "width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "TransposePlane",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "TransposePlane",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "TransposePlane",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "TransposePlane",
            "source stride smaller than width",
        ));
    }
    // 転置: 出力は幅と高さが入れ替わるので dst_stride >= src_size.height
    if dst_stride < src_size.height {
        return Err(Error::with_reason(
            -1,
            "TransposePlane",
            "destination stride smaller than height",
        ));
    }

    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "TransposePlane",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "TransposePlane",
            "source buffer too small",
        ));
    }
    // 転置: 出力の高さは src_size.width
    let dst_buf = checked_buf_size(
        dst_stride,
        src_size.width,
        "TransposePlane",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "TransposePlane",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::TransposePlane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
        );
    }

    Ok(())
}
